//! native_tls module
use std::fmt::{self, Formatter};
use std::fs::File;
use std::future::Future;
use std::io::{self, Error as IoError, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::future::Ready;
use futures_util::{ready, stream, Stream};
use pin_project::pin_project;
use tokio::net::TcpListener as TokioTcpListener;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio_native_tls::native_tls::{Identity, TlsAcceptor};
use tokio_native_tls::{TlsAcceptor as AsyncTlsAcceptor, TlsStream};

use super::{Acceptor, Listener, Accepted};

/// Builder to set the configuration for the TLS server.
pub struct NativeTlsConfig {
    pkcs12_path: Option<PathBuf>,
    pkcs12: Vec<u8>,
    password: String,
}

impl fmt::Debug for NativeTlsConfig {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("NativeTlsConfig").finish()
    }
}

impl Default for NativeTlsConfig {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl NativeTlsConfig {
    /// Create new `NativeTlsConfig`
    #[inline]
    pub fn new() -> Self {
        NativeTlsConfig {
            pkcs12_path: None,
            pkcs12: vec![],
            password: String::new(),
        }
    }

    /// Sets the pkcs12 via File Path, returns [`std::io::Error`] if the file cannot be open
    #[inline]
    pub fn with_pkcs12_path(mut self, path: impl AsRef<Path>) -> Self {
        self.pkcs12_path = Some(path.as_ref().into());
        self
    }

    /// Sets the pkcs12 via bytes slice
    #[inline]
    pub fn with_pkcs12(mut self, pkcs12: impl Into<Vec<u8>>) -> Self {
        self.pkcs12 = pkcs12.into();
        self
    }
    /// Sets the password
    #[inline]
    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    /// Generate identity
    #[inline]
    pub fn identity(mut self) -> Result<Identity, IoError> {
        if self.pkcs12.is_empty() {
            if let Some(path) = &self.pkcs12_path {
                let mut file = File::open(path)?;
                file.read_to_end(&mut self.pkcs12)?;
            }
        }
        Identity::from_pkcs12(&self.pkcs12, &self.password).map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))
    }
}
impl From<NativeTlsConfig> for Identity {
    fn from(config: NativeTlsConfig) -> Self {
        config.identity().unwrap()
    }
}