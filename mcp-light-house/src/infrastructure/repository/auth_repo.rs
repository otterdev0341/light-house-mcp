use reqwest::Client;
use serde_json::Value;

use crate::{config::token_config::TokenConfig, domain::{repository::req_impl::AuthFeatureReqImpl, value_object::{error_response::McpErrorResponse, json_response::ResMeDto}}};




#[derive(Clone)]
pub struct AuthRepositoryImpl {
    token: TokenConfig,
}

impl AuthRepositoryImpl {
    pub fn new(token: TokenConfig) -> Self {
        AuthRepositoryImpl {
            token,
        }
    }
}



#[async_trait::async_trait]
impl AuthFeatureReqImpl for AuthRepositoryImpl {
    async fn get_resme(&self, _user_token: &str) -> Result<ResMeDto, McpErrorResponse> {
        // Get base URL from environment variable or default to localhost
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        
        // Create a reqwest client
        let client = Client::new();

        // Construct the request URL
        let url = format!("{}/v1/mcp/resme", base_url);

        // Make the GET request with the user token in the Authorization header
        let response = client
            .get(&url)
            .header("Mcp Authorization", format!("MCP {}", self.token.mcp_token))
            .send()
            .await
            .map_err(|e| McpErrorResponse::new("400".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Deserialize the JSON response into a serde_json::Value
            let json_value: Value = response.json().await.map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse response: {}", e))
            })?;

            // Navigate the JSON structure to extract the "resme" field
            let resme_value = json_value
                .get("data")
                .and_then(|data| data.get("resme"))
                .ok_or_else(|| McpErrorResponse::new("404".to_string(), "Missing 'resme' field".to_string()))?;

            // Deserialize the "resme" field into ResMeDto
            let resme_dto: ResMeDto = serde_json::from_value(resme_value.clone()).map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse 'resme': {}", e))
            })?;
            
            Ok(resme_dto)
        } else {
            // Handle non-success status codes
            let status_code = response.status().to_string();
            let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(McpErrorResponse::new(
                status_code,
                error_message,
            ))
        }
    }
}