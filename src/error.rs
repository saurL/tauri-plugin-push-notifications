use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;
#[cfg(mobile)]
use tauri::plugin::mobile::PluginInvokeError;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
#[cfg(mobile)]
impl From<&str> for PluginInvokeError {
    fn from(s: &str) -> Self {
        PluginInvokeError::InvokeRejected(crate::error::ErrorResponse {
            code: None,
            message: Some(s.to_string()),
            data: (),
        })
    }
}

#[cfg(mobile)]
impl From<String> for PluginInvokeError {
    fn from(s: String) -> Self {
        PluginInvokeError::InvokeRejected(crate::error::ErrorResponse {
            code: None,
            message: Some(s),
            data: (),
        })
    }
}
#[cfg(mobile)]

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::PluginInvoke(s.into())
    }
}
#[cfg(mobile)]

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::PluginInvoke(s.into())
    }
}