use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorDetails {
    pub code: u16,
    pub code_name: &'static str,
    pub message: &'static str,
    // do not serialize this field
    #[serde(skip_serializing)]
    pub internal_error: Option<String>,
}

impl ErrorDetails {
    /// returns a copy of the error details with the internal error set
    pub fn with_internal_error(&self, internal_error: String) -> Self {
        ErrorDetails {
            code: self.code,
            code_name: self.code_name,
            message: self.message.clone(),
            internal_error: Some(internal_error),
        }
    }
}

/// error list

/// authentication failed(invalid credentials|authentication)
pub const ERR_AUTHENTICATION_FAILED: ErrorDetails = ErrorDetails {
    code: 401,
    code_name: "ERR_AUTHENTICATION_FAILED",
    message: "Authentication failed",
    internal_error: None,
};
// database connection string not found
pub const ERR_BACKEND_CONNECTION_STRING_NOT_FOUND: ErrorDetails = ErrorDetails {
    code: 500,
    code_name: "ERR-BACKEND-CONNECTION-OFFLINE",
    message: "Could not connect to the internal backend",
    internal_error: None,
};
/// could not connect to the SQL database
pub const ERR_BACKEND_CONNECTION_FAILED: ErrorDetails = ErrorDetails {
    code: 503,
    code_name: "ERR-BACKEND-CONNECTION-OFFLINE",
    message: "Could not connect to the internal backend",
    internal_error: None,
};
// database query failed
pub const ERR_BACKEND_QUERY_FAILED: ErrorDetails = ErrorDetails {
    code: 500,
    code_name: "ERR-COULD-NOT-PROCESS-REQUEST",
    message: "Could not query the internal backend",
    internal_error: None,
};
