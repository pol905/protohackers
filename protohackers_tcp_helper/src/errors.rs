pub enum ProtoHackersError {
  IOError(std::io::Error),
  UTF8Error(std::string::FromUtf8Error),
  SerdeJsonError(serde_json::Error),
}

impl From<std::io::Error> for ProtoHackersError {
  fn from(error: std::io::Error) -> Self {
      Self::IOError(error)
  }
}

impl From<std::string::FromUtf8Error> for ProtoHackersError {
  fn from(error: std::string::FromUtf8Error) -> Self {
      Self::UTF8Error(error)
  }
}

impl From<serde_json::Error> for ProtoHackersError {
  fn from(error: serde_json::Error) -> Self {
    Self::SerdeJsonError(error)
  }
}

impl std::fmt::Debug for ProtoHackersError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          Self::IOError(err) => {
              write!(f, "IO Error. {}", err) // how is err converted to string ?
          }
          Self::UTF8Error(err) => {
              write!(f, "Failed to parse. {}", err)
          }
          Self::SerdeJsonError(err) => {
            write!(f, "Failed to parse. {}", err)
        }
      }
  }
}
