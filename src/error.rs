use std::error::Error;
use std::fmt::{self, Display};
use std::io;
use std::result;

#[derive(Debug)]
/// Common errors that can occur during HTTP requests.
pub enum HttpError {
    /// IO Error
    Io(io::Error),
    /// Error generated by the `http` crate.
    Http(http::Error),
    /// TLS error encountered while connecting to an https server.
    #[cfg(feature = "tls")]
    Tls(native_tls::Error),
    /// Invalid URL ecountered while processing the request or response.
    InvalidUrl(&'static str),
    /// Server sent an invalid response.
    InvalidResponse(&'static str),
    /// Decoding error happened while trying to decode text.
    DecodingError(&'static str),
    /// Other errors.
    Other(&'static str),
    /// JSON decoding/encoding error.
    #[cfg(feature = "json")]
    Json(serde_json::Error),
}

impl Display for HttpError {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpError::Io(e) => write!(w, "Io({})", e),
            HttpError::Http(e) => write!(w, "Http({})", e),
            #[cfg(feature = "tls")]
            HttpError::Tls(e) => write!(w, "Tls({})", e),
            HttpError::InvalidUrl(s) => write!(w, "InvalidUrl({})", s),
            HttpError::InvalidResponse(s) => write!(w, "InvalidResponse({})", s),
            HttpError::DecodingError(s) => write!(w, "DecodingError({})", s),
            HttpError::Other(s) => write!(w, "Other({}", s),
            #[cfg(feature = "json")]
            HttpError::Json(e) => write!(w, "JsonError({})", e),
        }
    }
}

impl Error for HttpError {
    fn description(&self) -> &str {
        match self {
            HttpError::Io(e) => e.description(),
            HttpError::Http(e) => e.description(),
            #[cfg(feature = "tls")]
            HttpError::Tls(e) => e.description(),
            HttpError::InvalidUrl(s) => s,
            HttpError::InvalidResponse(s) => s,
            HttpError::DecodingError(s) => s,
            HttpError::Other(s) => s,
            #[cfg(feature = "json")]
            HttpError::Json(e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            HttpError::Io(e) => Some(e),
            HttpError::Http(e) => Some(e),
            #[cfg(feature = "tls")]
            HttpError::Tls(e) => Some(e),
            #[cfg(feature = "json")]
            HttpError::Json(e) => Some(e),
            _ => None,
        }
    }
}

macro_rules! impl_from {
    ($t:ty, $i:ident) => {
        impl From<$t> for HttpError {
            fn from(err: $t) -> HttpError {
                HttpError::$i(err)
            }
        }
    };
}

impl_from!(io::Error, Io);
impl_from!(http::Error, Http);
#[cfg(feature = "tls")]
impl_from!(native_tls::Error, Tls);
#[cfg(feature = "json")]
impl_from!(serde_json::Error, Json);

/// Wrapper for the `Result` type with an `HttpError`.
pub type HttpResult<T = ()> = result::Result<T, HttpError>;
