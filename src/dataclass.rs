use crate::schemes::{
    SResultUlp,
    DataEnum,
    LineEnum
};

impl SResultUlp {
    pub fn new(
        url: String,
        port: Option<u16>,
        login: String,
        password: String,
        datatype: DataEnum,
        linetype: LineEnum
    ) -> Self {
        SResultUlp {
            url,
            port,
            login,
            password,
            datatype,
            linetype
        }
    }
    pub fn credits(self) -> String {
        format!("{}:{}", self.login, self.password)
    }

    pub fn urllogpass(self) -> String {
        if let Some(port) = self.port {
            return format!("{}:{}:{}:{}", self.url, port, self.login, self.password);
        }
        format!("{}:{}:{}", self.url, self.login, self.password)
    }
}