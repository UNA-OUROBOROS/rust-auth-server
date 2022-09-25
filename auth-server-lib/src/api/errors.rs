use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorDetails {
    pub http_code: u16,
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
            http_code: self.http_code,
            code_name: self.code_name,
            message: self.message.clone(),
            internal_error: Some(internal_error),
        }
    }
}

/// error list
// unknown internal error
pub const ERR_UNKNOWN_INTERNAL_ERROR: ErrorDetails = ErrorDetails {
    http_code: 500,
    code_name: "ERR_UNKNOWN_INTERNAL_ERROR",
    message: "An unknown internal error occurred",
    internal_error: None,
};
// invalid data/malformed request
pub const ERR_INVALID_DATA: ErrorDetails = ErrorDetails {
    http_code: 400,
    code_name: "ERR_INVALID_DATA",
    message: "The data provided is invalid",
    internal_error: None,
};

// authentication failed(invalid credentials|authentication)
pub const ERR_AUTHENTICATION_FAILED: ErrorDetails = ErrorDetails {
    http_code: 401,
    code_name: "ERR_AUTHENTICATION_FAILED",
    message: "Authentication failed",
    internal_error: None,
};
// database connection string not found
pub const ERR_BACKEND_CONNECTION_STRING_NOT_FOUND: ErrorDetails = ErrorDetails {
    http_code: 500,
    code_name: "ERR-BACKEND-CONNECTION-OFFLINE",
    message: "Could not connect to the internal backend",
    internal_error: None,
};
/// could not connect to the SQL database
pub const ERR_BACKEND_CONNECTION_FAILED: ErrorDetails = ErrorDetails {
    http_code: 503,
    code_name: "ERR-BACKEND-CONNECTION-OFFLINE",
    message: "Could not connect to the internal backend",
    internal_error: None,
};
// database query failed
pub const ERR_BACKEND_QUERY_FAILED: ErrorDetails = ErrorDetails {
    http_code: 500,
    code_name: "ERR-COULD-NOT-PROCESS-REQUEST",
    message: "Could not query the internal backend",
    internal_error: None,
};
// database resource not found
pub const ERR_DATABASE_RESOURCE_NOT_FOUND: ErrorDetails = ErrorDetails {
    http_code: 404,
    code_name: "ERR-RESOURCE-NOT-FOUND",
    message: "Could not find the requested resource",
    internal_error: None,
};
// database failed transaction
pub const ERR_DATABASE_TRANSACTION_FAILED: ErrorDetails = ErrorDetails {
    http_code: 500,
    code_name: "ERR-COULD-NOT-PROCESS-REQUEST",
    message: "the backend could not process the request",
    internal_error: None,
};
// existing record found
pub const ERR_DATABASE_RECORD_EXISTS: ErrorDetails = ErrorDetails {
    http_code: 409,
    code_name: "ERR-RECORD-ALREADY-EXISTS",
    message: "The record already exists",
    internal_error: None,
};
