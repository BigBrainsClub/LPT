use memchr::memmem;
use smallvec::SmallVec;
use url::{Host, Url};
use crate::{
    config::Config,
    BAD_REPLACE_LIST,
    REPLACE_CHARS,
    text::{
        schema::{
            TextValidator,
            DataEnum,
            LineEnum,
            ValidationError
        },
        validator_credits::{
            is_valid_email,
            is_valid_login,
            is_valid_phone_number
        }
    }
};


impl TextValidator {
    fn get_type_line(line: &[u8]) -> (LineEnum, Vec<u8>) {
        if line.starts_with(b"http://") || line.starts_with(b"https://") {
            return (LineEnum::Http, line.to_vec());
        } 
        if line.starts_with(b"android://") {
            return (LineEnum::Android, line.to_vec());
        }
        let android_find = memmem::Finder::new(b"==@");
        if android_find.find(&line).is_some() {
            return (LineEnum::Android, [b"android://", line].concat());
        }
        let finder_http = memmem::Finder::new(b"http://");
        let finder_https = memmem::Finder::new(b"https://");
    
        if finder_http.find(line).is_some() || finder_https.find(line).is_some() {
            return (LineEnum::ReversedHttp, line.to_vec());
        }
        (LineEnum::WithoutHttp, line.to_vec())
    }

    fn validate_host(url: &str) -> Result<(Vec<u8>, Option<u16>), ValidationError> {
        let parsed_url = Url::parse(url).ok().ok_or(ValidationError::ParseError)?;
        let host = parsed_url.host().ok_or(ValidationError::ParseError)?;
        let host_text = match host {
            Host::Domain(dom) => dom.to_string(),
            Host::Ipv4(ipv4) => ipv4.to_string(),
            Host::Ipv6(ipv6) => ipv6.to_string(),
        };
        let full = format!("https://{}", host_text).into_bytes();
        Ok((full, parsed_url.port()))
    }

    
    fn get_parts_in_line(input: &[u8], config: &Config) -> Result<Self, ValidationError> {
        let mut processed_line: SmallVec<[u8; 128]> = SmallVec::with_capacity(input.len());
    
        let mut replace_map = [false; 256];
        for &b in REPLACE_CHARS {
            replace_map[b as usize] = true;
        }
    
        for &b in input {
            processed_line.push(if replace_map[b as usize] { b':' } else { b });
        }

        let count_split = processed_line.iter().filter(|&&b| b == b':').count();
        if !(2..=5).contains(&count_split) {
            return Err(ValidationError::ParseError);
        }
        let (linetype, line) = Self::get_type_line(&processed_line);
        let mut parts: SmallVec<[&[u8]; 6]> = line.splitn(6, |&b| b == b':').collect();

        let (mut login, password, url) = match linetype {
            LineEnum::ReversedHttp => {
                parts.reverse();
                let login = parts.pop();
                let password = parts.pop();
                let url = Some(parts.join(&b':').to_vec());
                (login, password, url)
            }
            _ => {
                let password = parts.pop();
                let login = parts.pop();
                let url = Some(parts.join(&b':').to_vec());
                (login, password, url)
            }
        };
        
        let login_process = login.as_deref().map(|l| {
            let mut filtered = SmallVec::<[u8; 64]>::with_capacity(l.len());
            for &b in l {
                if !BAD_REPLACE_LIST.contains(&b) {
                    filtered.push(b);
                }
            }
            filtered
        });
        login = login_process.as_deref();
    
        let mut result = Self {
            url,
            port: None,
            login: login.map(|x| {
                if config.login_to_lower_case {
                    return x.to_ascii_lowercase()
                }
                x.to_vec()
            }),
            password: password.map(|x| x.to_vec()),
            datatype: DataEnum::NonUsed,
            linetype: if linetype == LineEnum::ReversedHttp {
                LineEnum::Http
            } else {
                linetype
            },
        };
        if result.linetype == LineEnum::WithoutHttp {
            result.linetype = LineEnum::Http;
            let url = result.url.as_mut().ok_or(ValidationError::ParseError)?;
            let start_url = b"https://";
            unsafe {
                let old_len = url.len();
                let prefix_len = start_url.len();
                url.reserve(prefix_len);
                std::ptr::copy(url.as_ptr(), url.as_mut_ptr().add(prefix_len), old_len);
                std::ptr::copy_nonoverlapping(start_url.as_ptr(), url.as_mut_ptr(), prefix_len);

                url.set_len(old_len + prefix_len);
            }
        }
        if matches!(result.linetype, LineEnum::Http) {
            let url_bytes = &result.url.ok_or(ValidationError::ParseError)?;
            let url_str = String::from_utf8_lossy(url_bytes);
            let (host, port) = Self::validate_host(&url_str)?;
            result.url = Some(host);
            result.port = port;
            Ok(result)
        } else if matches!(result.linetype, LineEnum::Android) {
            Ok(result)
        } else {
            Err(ValidationError::ParseError)
        }
    }
    
    fn validate_full_length(&self, config: &Config) -> Result<(), ValidationError> {
        if !config.check_length {
            return Ok(())
        }
        let (min, max) = config.all_length;
        if (min..=max).contains(&(self.full_line().len() as u8)) {
            Ok(())
        } else {
            Err(ValidationError::LengthError)
        }
    }

    fn checking_bad_words_in_credits(&self, filter_vector: &[Vec<u8>]) -> Result<(), ValidationError> {
        if filter_vector.is_empty() {
            return Ok(());
        }
        let login: &[u8] = self.login.as_deref().ok_or(ValidationError::FindDataTypeError)?;
        let password = self.password.as_deref().ok_or(ValidationError::FindDataTypeError)?;
        let mut buf = SmallVec::<[u8; 128]>::with_capacity(login.len() + password.len() + 1);
        buf.extend_from_slice(login);
        buf.push(b':');
        buf.extend_from_slice(password);
        if filter_vector.iter().any(|pattern| 
            !pattern.is_empty() && memmem::Finder::new(&pattern.to_ascii_lowercase()).find(&buf.to_ascii_lowercase()).is_some()
        ) {Ok(())} else {
            Err(ValidationError::FilterError)
        }
    }

    fn find_type_credits(&self, config: &Config) -> Result<DataEnum, ValidationError> {
        if !config.find_data_type {
            return Ok(DataEnum::NonUsed)
        }
        let login = self.login.as_ref().ok_or(ValidationError::FindDataTypeError)?;
        if is_valid_email(&login, config) {
            Ok(DataEnum::Email)
        } else if is_valid_phone_number(&login) {
            Ok(DataEnum::Number)
        } else if is_valid_login(&login) {
            Ok(DataEnum::Login)
        } else {
            Err(ValidationError::FindDataTypeError)
        }
    }
    
    fn validate_credit_length(&self, config: &Config) -> Result<(), ValidationError> {
        if !config.check_length {
            return Ok(())
        }
        let password_len = self.password.as_ref().ok_or(ValidationError::LengthError)?.len() as u8;
        if !(config.password_length.0..=config.password_length.1).contains(&password_len) {
            return Err(ValidationError::LengthError);
        }
    
        let login_len = self.login.as_ref().ok_or(ValidationError::LengthError)?.len();
        let (min, max) = match self.datatype {
            DataEnum::Email => config.email_length,
            DataEnum::Login => config.login_length,
            DataEnum::Number => config.number_length,
            DataEnum::NonUsed => config.default_length,
        };
        match (min as usize..=max as usize).contains(&login_len) {
            true => Ok(()),
            false => Err(ValidationError::LengthError)
        }
    }

    fn check_equal(&self, config: &Config) -> Result<(), ValidationError> {
        if !config.remove_non_uniq_lp {
            return Ok(())
        }
        match self.login.as_ref().ok_or(ValidationError::ParseError)? == self.password.as_ref().ok_or(ValidationError::ParseError)? {
            true => {Err(ValidationError::LpEqualError)},
            false => {Ok(())}
        }
    }

    pub fn validate(line: &[u8], config: &Config, filter_vector: &[Vec<u8>]) -> Result<TextValidator, ValidationError> {
        let mut validator = Self::get_parts_in_line(line, config)?;
        validator.checking_bad_words_in_credits(filter_vector)?;
        validator.datatype = validator.find_type_credits(config)?;
        validator.check_equal(config)?;
        validator.validate_credit_length(config)?;
        validator.validate_full_length(config)?;
        return Ok(validator)
    }
}