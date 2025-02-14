pub mod main;
pub mod validator_credits;
pub mod schema;


impl schema::TextValidator {
    pub fn credits(&self) -> Vec<u8> {
        let login = self.login.as_deref().unwrap_or_default();
        let password = self.password.as_deref().unwrap_or_default();
        [login, b":", password].concat()
    }
    
    fn domain(&self) -> Vec<u8> {
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