use memchr::memmem;
use smallvec::SmallVec;
use url::Url;
use crate::{config::Config, BAD_REPLACE_LIST, REPLACE_CHARS};

#[derive(PartialEq, Debug, Clone)]
pub enum DataEnum {
    Login,
    Email,
    Number,
    NonUsed,
}

#[derive(PartialEq, Debug, Clone)]
enum LineEnum {
    Http,
    Android,
    ReversedHttp,
    WithoutHttp
}

impl std::fmt::Display for DataEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataEnum::Email => "email.txt",
                DataEnum::Login => "login.txt",
                DataEnum::Number => "number.txt",
                DataEnum::NonUsed => "default.txt",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct TextValidator {
    url: Option<Vec<u8>>,
    port: Option<u16>,
    login: Option<Vec<u8>>,
    password: Option<Vec<u8>>,
    pub datatype: DataEnum,
    linetype: LineEnum,
}

impl TextValidator {
    fn get_type_line(line: &[u8]) -> LineEnum {
        if line.starts_with(b"http://") || line.starts_with(b"https://") {
            return LineEnum::Http;
        } 
        if line.starts_with(b"android://") {
            return LineEnum::Android;
        }
        let finder_http = memmem::Finder::new(b"http://");
        let finder_https = memmem::Finder::new(b"https://");
    
        if finder_http.find(line).is_some() || finder_https.find(line).is_some() {
            return LineEnum::ReversedHttp;
        }
    
        LineEnum::WithoutHttp
    }
    
    fn validate_host(url: &str) -> Option<(Vec<u8>, Option<u16>)> {
        let parsed_url = Url::parse(url).ok()?;
        let domain = parsed_url.domain()?;
    
        Some((format!("https://{}", domain).into_bytes(), parsed_url.port()))
    }
    
    fn get_parts_in_line(input: &[u8]) -> Option<Self> {
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
            return None;
        }
        let mut parts: SmallVec<[&[u8]; 6]> = processed_line.splitn(6, |&b| b == b':').collect();
    
        let linetype = Self::get_type_line(&processed_line);
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
            login: login.map(|x| x.to_vec()),
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
            if let Some(ref mut url) = result.url {
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
        }
        if matches!(result.linetype, LineEnum::Http) {
            if let Some(url_bytes) = &result.url {
                let url_str = String::from_utf8_lossy(url_bytes);
                let (host, port) = Self::validate_host(&url_str)?;
                result.url = Some(host);
                result.port = port;
                return Some(result);
            }
            None
        } else if matches!(result.linetype, LineEnum::Android) {
            Some(result)
        } else {
            None
        }
    }
    
    fn validate_full_length(line: &[u8], config: &Config) -> bool {
        let (min, max) = config.all_length;
        (min as usize..=max as usize).contains(&(line.len()))
    }

    fn checking_bad_words_in_credits(&self, filter_vector: &[Vec<u8>]) -> Option<bool> {
        if filter_vector.is_empty() {
            return Some(false)
        }
        let login = self.login.as_deref()?;
        let password = self.password.as_deref()?;
        let mut buf = SmallVec::<[u8; 128]>::with_capacity(login.len() + password.len() + 1);
        buf.extend_from_slice(login);
        buf.push(b':');
        buf.extend_from_slice(password);
        Some(filter_vector.iter().any(|pattern| 
            !pattern.is_empty() && memmem::Finder::new(&pattern.to_ascii_lowercase()).find(&buf.to_ascii_lowercase()).is_some()
        ))
    }

    fn find_type_credits(&self, config: &Config) -> Option<DataEnum> {
        if !config.find_data_type {
            return Some(DataEnum::NonUsed);
        }
    
        let login = self.login.as_ref()?;
    
        if config.parse_email && Self::is_valid_email(&login) {
            Some(DataEnum::Email)
        } else if config.parse_number && Self::is_valid_phone_number(&login) {
            Some(DataEnum::Number)
        } else if config.parse_login && Self::is_valid_login(&login) {
            Some(DataEnum::Login)
        } else {
            None
        }
    }
    #[inline(always)]
    fn is_valid_login(login: &[u8]) -> bool {
        let len = login.len();
        if len == 0 {
            return false;
        }
        unsafe {
            let mut ptr = login.as_ptr();
            let end = ptr.add(len);
            let first = *ptr;
            if !((first >= b'a' && first <= b'z') || (first >= b'A' && first <= b'Z')) {
                return false;
            }
            ptr = ptr.add(1);
            while ptr < end {
                let b = *ptr;
                if !((b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || (b >= b'0' && b <= b'9') || b == b'_' || b == b'-') {
                    return false;
                }
                ptr = ptr.add(1);
            }
        }
        true
    }
    #[inline(always)]
    fn is_valid_phone_number(number: &[u8]) -> bool {
        let len = number.len();
        if len == 0 {
            return false;
        }

        unsafe {
            let mut ptr = number.as_ptr();
            let end = ptr.add(len);
            if *ptr == b'+' {
                ptr = ptr.add(1);
            }

            let mut digit_count = 0;
            while ptr < end && (*ptr >= b'0' && *ptr <= b'9') {
                digit_count += 1;
                ptr = ptr.add(1);
                if digit_count > 4 {
                    return false;
                }
            }
            if ptr < end && (*ptr == b'-' || *ptr == b'.' || *ptr == b' ') {
                ptr = ptr.add(1);
            }
            if ptr < end && *ptr == b'(' {
                ptr = ptr.add(1);
                digit_count = 0;
                while ptr < end && (*ptr >= b'0' && *ptr <= b'9') {
                    digit_count += 1;
                    ptr = ptr.add(1);
                    if digit_count > 3 {
                        return false;
                    }
                }
                if ptr == end || *ptr != b')' {
                    return false;
                }
                ptr = ptr.add(1);
            }
            if ptr < end && (*ptr == b'-' || *ptr == b'.' || *ptr == b' ') {
                ptr = ptr.add(1);
            }
            digit_count = 0;
            while ptr < end && (*ptr >= b'0' && *ptr <= b'9') {
                digit_count += 1;
                ptr = ptr.add(1);
                if digit_count > 4 {
                    return false;
                }
            }
            if ptr < end && (*ptr == b'-' || *ptr == b'.' || *ptr == b' ') {
                ptr = ptr.add(1);
            }
            digit_count = 0;
            while ptr < end && (*ptr >= b'0' && *ptr <= b'9') {
                digit_count += 1;
                ptr = ptr.add(1);
                if digit_count > 4 {
                    return false;
                }
            }
            if ptr < end && (*ptr == b'-' || *ptr == b'.' || *ptr == b' ') {
                ptr = ptr.add(1);
            }
            digit_count = 0;
            while ptr < end && (*ptr >= b'0' && *ptr <= b'9') {
                digit_count += 1;
                ptr = ptr.add(1);
                if digit_count > 9 {
                    return false;
                }
            }
            ptr == end
        }
    }


    #[inline(always)]
    fn is_valid_email(email: &[u8]) -> bool {
        let len = email.len();

        let at_pos = memchr::memchr(b'@', email);
        let dot_pos = memchr::memrchr(b'.', email);
        if at_pos.is_none() || dot_pos.is_none() {
            return false;
        }
        let at_pos = at_pos.unwrap();
        let dot_pos = dot_pos.unwrap();
        if at_pos == 0 || at_pos == len - 1 {
            return false;
        }

        if dot_pos <= at_pos + 1 || dot_pos == len - 1 {
            return false;
        }

        if (len - at_pos) < 3 || (len - dot_pos) < 3 {
            return false;
        }

        unsafe {
            let mut ptr = email.as_ptr();
            let end = ptr.add(len);
            while ptr < end {
                if *ptr <= b' ' {
                    return false;
                }
                ptr = ptr.add(1);
            }
        }
        true
    }

    
    fn validate_credit_length(&self, config: &Config) -> Option<bool> {
        if !config.check_length {
            return Some(true);
        }
    
        let password_len = self.password.as_ref()?.len();
        if !(config.password_length.0 as usize..=config.password_length.1 as usize).contains(&password_len) {
            return None;
        }
    
        let login_len = self.login.as_ref()?.len();
        let (min, max) = match self.datatype {
            DataEnum::Email => config.email_length,
            DataEnum::Login => config.login_length,
            DataEnum::Number => config.number_length,
            _ => config.default_length,
        };
    
        Some((min as usize..=max as usize).contains(&login_len))
    }

    pub fn validate(line: &[u8], config: &Config, filter_vector: &[Vec<u8>]) -> Option<Self> {
        let mut validator = Self::get_parts_in_line(line)?;
        if validator.checking_bad_words_in_credits(filter_vector)? {
            return None;
        }
        validator.datatype = validator.find_type_credits(config)?;
        if !validator.validate_credit_length(config)? || !Self::validate_full_length(line, config) {
            return None;
        }
        Some(validator)
    }
    pub fn credits(&self) -> Vec<u8> {
        let login = self.login.as_deref().unwrap_or_default();
        let password = self.password.as_deref().unwrap_or_default();
        [login, b":", password].concat()
    }
    
    pub fn domain(&self) -> Vec<u8> {
        let url = self.url.as_deref().unwrap_or_default();
        if let Some(port) = self.port {
            let mut buffer = Vec::from(url);
            buffer.push(b':');
            buffer.extend(port.to_string().as_bytes());
            buffer
        } else {
            Vec::from(url)
        }
    }
    
    pub fn full_line(&self) -> Vec<u8> {
        let mut buffer = self.domain();
        buffer.push(b':');
        buffer.extend(self.credits());
        buffer
    }
}