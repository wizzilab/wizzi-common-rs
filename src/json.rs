use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub enum Error {
    Json {
        data: String,
        error: String,
    },
    Format {
        data: String,
        object: serde_json::Value,
        error: String,
    },
    Io(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

pub fn from_str<S: AsRef<str>, T: DeserializeOwned + Sized>(data: S) -> Result<T, Error> {
    let data = data.as_ref();
    match serde_json::from_str(data) {
        Ok(v) => Ok(v),
        Err(e) => match serde_json::from_str::<serde_json::Value>(data) {
            Ok(v) => Err(Error::Format {
                data: data.to_string(),
                object: v,
                error: e.to_string(),
            }),
            Err(e) => Err(Error::Json {
                data: data.to_string(),
                error: e.to_string(),
            }),
        },
    }
}

pub async fn from_file_path<P: AsRef<std::path::Path>, T: DeserializeOwned + Sized>(
    path: P,
) -> Result<T, Error> {
    let path = path.as_ref();
    let data = tokio::fs::read_to_string(path).await?;
    from_str(data)
}
