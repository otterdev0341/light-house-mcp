use serde::Serialize;



#[derive(Serialize, Debug)]
pub struct McpErrorResponse{
    pub code: String,
    pub message: String,
}

impl McpErrorResponse {
    pub fn new(code: String, message: String) -> Self {
        McpErrorResponse { code, message }
    }
}


impl std::fmt::Display for McpErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "McpErrorResponse {{ code: {}, message: {} }}", self.code, self.message)
    }
}

