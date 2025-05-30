use crate::{config::token_config::TokenConfig, domain::{repository::req_impl::ServiceOperationFeatureReqImpl, value_object::{error_response::McpErrorResponse, json_response::{ResListAssetDto, ResListContactDto, ResListExpenseDto}}}};





#[derive(Clone)]
pub struct ServiceOperationRepositoryImpl{
    token: TokenConfig
}

impl ServiceOperationRepositoryImpl {
    pub fn new(token: TokenConfig) -> Self {
        ServiceOperationRepositoryImpl {
            token
        }
    }
}

#[async_trait::async_trait]
impl ServiceOperationFeatureReqImpl for ServiceOperationRepositoryImpl {
    async fn get_all_contact(&self, _user_token: &str) -> Result<ResListContactDto, McpErrorResponse>
    {
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let client = reqwest::Client::new();
        let url = format!("{}/v1/mcp/contact", base_url);

        let response = client
            .get(&url)
            .header("Mcp Authorization", format!("MCP {}", self.token.mcp_token))
            .send()
            .await
            .map_err(|e| McpErrorResponse::new("400".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Deserialize the JSON response into ResListContactDto
            let res_list_dto: ResListContactDto = response.json().await.map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse response: {}", e))
            })?;
            
            Ok(res_list_dto)
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

    async fn get_all_expense(&self, _user_token: &str) -> Result<ResListExpenseDto, McpErrorResponse>
    {
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let client = reqwest::Client::new();
        let url = format!("{}/v1/mcp/expense", base_url);

        let response = client
            .get(&url)
            .header("Mcp Authorization", format!("MCP {}", self.token.mcp_token))
            .send()
            .await
            .map_err(|e| McpErrorResponse::new("400".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Deserialize the JSON response into ResListExpenseDto
            let res_list_dto: ResListExpenseDto = response.json().await.map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse response: {}", e))
            })?;
            
            Ok(res_list_dto)
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

    async fn get_all_asset(&self, _user_token: &str) -> Result<ResListAssetDto, McpErrorResponse>
    {
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let client = reqwest::Client::new();
        let url = format!("{}/v1/mcp/asset", base_url);

        let response = client
            .get(&url)
            .header("Mcp Authorization", format!("MCP {}", self.token.mcp_token))
            .send()
            .await
            .map_err(|e| McpErrorResponse::new("400".to_string(), e.to_string()))?;

        // Check if the response status is successful
        if response.status().is_success() {
            // Deserialize the JSON response into ResListAssetDto
            let res_list_dto: ResListAssetDto = response.json().await.map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse response: {}", e))
            })?;
            
            Ok(res_list_dto)
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
// end of ServiceOperationFeatureReqImpl implementation