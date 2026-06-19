use std::fmt;

use crate::error::TextureLoadError;

#[derive(Debug)]
pub enum ImportError {
    ReadConfig { path: String, source: std::io::Error },
    ParseConfig {
        path: String,
        source: Box<dyn std::error::Error>,
    },
    Texture(TextureLoadError),
    Sample {
        path: String,
        source: Box<dyn std::error::Error>,
    },
    Walk { path: String, source: std::io::Error },
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::ReadConfig { path, source } => {
                write!(f, "failed to read config file '{path}': {source}")
            }
            ImportError::ParseConfig { path, source } => {
                write!(f, "failed to parse config file '{path}': {source}")
            }
            ImportError::Texture(e) => write!(f, "{e}"),
            ImportError::Sample { path, source } => {
                write!(f, "failed to load sample '{path}': {source}")
            }
            ImportError::Walk { path, source } => {
                write!(f, "failed to walk directory '{path}': {source}")
            }
        }
    }
}

impl std::error::Error for ImportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ImportError::ReadConfig { source, .. } => Some(source),
            ImportError::ParseConfig { source, .. } => Some(source.as_ref()),
            ImportError::Texture(e) => Some(e),
            ImportError::Sample { source, .. } => Some(source.as_ref()),
            ImportError::Walk { source, .. } => Some(source),
        }
    }
}

impl From<TextureLoadError> for ImportError {
    fn from(e: TextureLoadError) -> Self {
        ImportError::Texture(e)
    }
}
