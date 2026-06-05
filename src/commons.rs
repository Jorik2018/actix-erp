// src/error/app_error.rs

#[derive(Debug)]
pub enum AppError {
    Database(String),
    NotFound,
}

pub type AppResult<T> = Result<T, AppError>;