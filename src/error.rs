#[derive(Debug)]
pub enum CubatureError {
    BoundsInconsistent,
    FailedToConverge,
    IntegrandEvalFailed,
    OutputBuffersInconsistent,
    IncorrectOutputBufferSize,
    Unknown,
}

impl std::error::Error for CubatureError {}

impl std::fmt::Display for CubatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CubatureError::BoundsInconsistent => write!(f, "bounds inconsistent"),
            CubatureError::FailedToConverge => write!(f, "failed to converge"),
            CubatureError::IntegrandEvalFailed => write!(f, "integrand evaluation failed"),
            CubatureError::OutputBuffersInconsistent => write!(f, "output buffers inconsistent"),
            CubatureError::IncorrectOutputBufferSize => write!(f, "incorrect output buffer size"),
            CubatureError::Unknown => write!(f, "unknown error"),
        }
    }
}

impl From<::std::os::raw::c_int> for CubatureError {
    fn from(value: ::std::os::raw::c_int) -> Self {
        match value {
            _ if value == Self::BoundsInconsistent as i32 => Self::BoundsInconsistent,
            _ if value == Self::OutputBuffersInconsistent as i32 => Self::OutputBuffersInconsistent,
            _ if value == Self::FailedToConverge as i32 => Self::FailedToConverge,
            _ if value == Self::IntegrandEvalFailed as i32 => Self::IntegrandEvalFailed,
            _ if value == Self::IncorrectOutputBufferSize as i32 => Self::IncorrectOutputBufferSize,
            _ => Self::Unknown,
        }
    }
}
