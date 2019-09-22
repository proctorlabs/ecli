#[derive(Debug)]
pub enum AppError {
    Fatal(String),
    Info(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Fatal(format!("{}", e))
    }
}

impl From<serde_yaml::Error> for AppError {
    fn from(e: serde_yaml::Error) -> Self {
        AppError::Fatal(format!("Failure parsing configuration file: {}", e))
    }
}

impl From<templar::TemplarError> for AppError {
    fn from(e: templar::TemplarError) -> Self {
        AppError::Fatal(format!("{}", e))
    }
}
