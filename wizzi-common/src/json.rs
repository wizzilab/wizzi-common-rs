use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum DecodingError {
    Json {
        data: String,
        error: String,
    },
    StructFormat {
        object: serde_json::Value,
        error: String,
    },
}

pub fn from_str<S: AsRef<str>, T: DeserializeOwned + Sized>(data: S) -> Result<T, DecodingError> {
    let data = data.as_ref();
    match serde_json::from_str(data) {
        Ok(v) => Ok(v),
        Err(e) => match serde_json::from_str::<serde_json::Value>(data) {
            Ok(v) => Err(DecodingError::StructFormat {
                object: v,
                error: e.to_string(),
            }),
            Err(e) => Err(DecodingError::Json {
                data: data.to_string(),
                error: e.to_string(),
            }),
        },
    }
}

#[derive(Debug, Clone)]
pub enum EncodingError<T: Debug + Clone> {
    Json { data: T, error: String },
}

pub fn to_string<T: Serialize + Debug + Clone>(data: &T) -> Result<String, EncodingError<T>> {
    match serde_json::to_string(data) {
        Ok(v) => Ok(v),
        Err(e) => Err(EncodingError::Json {
            data: data.clone(),
            error: e.to_string(),
        }),
    }
}

#[derive(Debug, Clone)]
pub enum DecodingFileError {
    Json {
        path: String,
        data: String,
        error: String,
    },
    StructFormat {
        path: String,
        object: serde_json::Value,
        error: String,
    },
    BadFile {
        path: String,
        error: std::io::ErrorKind,
    },
}

pub async fn from_file_path<P: AsRef<std::path::Path>, T: DeserializeOwned + Sized>(
    path: P,
) -> Result<T, DecodingFileError> {
    let path = path.as_ref();
    let data = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| DecodingFileError::BadFile {
            path: path
                .to_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "???".to_string()),
            error: e.kind(),
        })?;
    from_str(data).map_err(|e| match e {
        DecodingError::Json { data, error } => DecodingFileError::Json {
            path: path
                .to_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "???".to_string()),
            data,
            error,
        },
        DecodingError::StructFormat { object, error } => DecodingFileError::StructFormat {
            path: path
                .to_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "???".to_string()),
            object,
            error,
        },
    })
}
