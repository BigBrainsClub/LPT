#[derive(PartialEq, Clone, Debug)]
pub enum DataEnum {
    Login,
    Email,
    Number,
    NonUsed,
}

impl std::fmt::Display for DataEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataEnum::Email => "email",
                DataEnum::Login => "login",
                DataEnum::Number => "number",
                DataEnum::NonUsed => "unused_find_data"
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LineEnum {
    Http,
    Android,
    ReversedHttp,
    WithoutHttp
}
impl std::fmt::Display for LineEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LineEnum::Android => "android",
                LineEnum::Http => "http",
                _ => "N_A"
            }
        )
    }
}

pub enum ValidationError {
    LengthError,
    FindDataTypeError,
    ParseError,
    FilterError,
    LpEqualError
}

#[derive(Clone, Debug)]
pub struct TextValidator {
    pub url: Option<Vec<u8>>,
    pub port: Option<u16>,
    pub login: Option<Vec<u8>>,
    pub password: Option<Vec<u8>>,
    pub datatype: DataEnum,
    pub linetype: LineEnum,
}