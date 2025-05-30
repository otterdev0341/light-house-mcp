use std::env;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub mcp_token: String,
}

pub fn load() -> Result<TokenConfig> {
    dotenvy::dotenv().ok();

    let mcp_token = match dotenvy::var("MCP_TOKEN") {
        Ok(token) => token,
        Err(_) => env::args()
            .nth(1)
            .expect("MCP token not found in environment variables or command line arguments"),
    };

    Ok(TokenConfig { mcp_token })
}