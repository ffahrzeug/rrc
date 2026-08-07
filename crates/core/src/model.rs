use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ServiceName(String);

#[derive(Debug, thiserror::Error)]
#[error("invalid service name: {0:?}")]
pub struct ServiceNameError(String);

// Size of the kernel's `task_struct.comm` buffer (`include/linux/sched.h`).
const TASK_COMM_LEN: usize = 16;
const MAX_SERVICE_NAME_LEN: usize = TASK_COMM_LEN - 1;

impl ServiceName {
    pub fn new(s: impl Into<String>) -> Result<Self, ServiceNameError> {
        let s = s.into();
        let ok = !s.is_empty() && s.len() <= MAX_SERVICE_NAME_LEN
            && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_'));
        if ok {
            Ok(Self(s))
        } else {
            Err(ServiceNameError(s))
        }
    }
}

impl TryFrom<String> for ServiceName {
    type Error = ServiceNameError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl From<ServiceName> for String {
    fn from(s: ServiceName) -> Self {
        s.0
    }
}