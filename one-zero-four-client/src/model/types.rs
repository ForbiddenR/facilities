#[derive(Debug, Default, Clone, Copy)]
pub enum LoginStatus {
    #[default]
    Successful,
    Fail,
}

impl From<u8> for LoginStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Successful,
            _ => Self::Fail,
        }
    }
}

impl From<LoginStatus> for u8 {
    fn from(value: LoginStatus) -> Self {
        match value {
            LoginStatus::Successful => 0,
            LoginStatus::Fail => 1,
        }
    }
}
