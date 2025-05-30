use serde_json::Value;

use crate::{config::token_config::TokenConfig, domain::{repository::req_impl::ServiceTypeFeatureReqImpl, value_object::{error_response::McpErrorResponse, json_response::{ResListAssestTypeDto, ResListContactTypeDto, ResListExpenseTypeDto}}}};





#[derive(Clone)]
pub struct ServiceTypeRepositoryImpl{
    token: TokenConfig
}

impl ServiceTypeRepositoryImpl {
    pub fn new(token: TokenConfig) -> Self {
        ServiceTypeRepositoryImpl {
            token
        }
    }
}

#[async_trait::async_trait]
impl ServiceTypeFeatureReqImpl for ServiceTypeRepositoryImpl{
    
    async fn get_all_contact_type(&self, _user_token: &str) -> Result<ResListContactTypeDto, McpErrorResponse>
    {
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let client = reqwest::Client::new();
        let url = format!("{}/v1/mcp/contact-type", base_url);

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
                .and_then(|data| data.get("data"))
                .ok_or_else(|| McpErrorResponse::new("404".to_string(), "Missing 'data' field".to_string()))?;

            // Deserialize the "resme" field into ResMeDto
            let res_list_dto: ResListContactTypeDto = serde_json::from_value(resme_value.clone()).map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse '': {}", e))
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


    } // end of get_all_contact_type

    async fn get_all_expense_type(&self, _user_token: &str) -> Result<ResListExpenseTypeDto, McpErrorResponse>
    {
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let client = reqwest::Client::new();
        let url = format!("{}/v1/mcp/expense-type", base_url);

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
                .and_then(|data| data.get("data"))
                .ok_or_else(|| McpErrorResponse::new("404".to_string(), "Missing 'data' field".to_string()))?;

            // Deserialize the "resme" field into ResMeDto
            let res_list_dto: ResListExpenseTypeDto = serde_json::from_value(resme_value.clone()).map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse '': {}", e))
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


    } // end of get_all_expense_type


    async fn get_all_asset_type(&self, _user_token: &str) -> Result<ResListAssestTypeDto, McpErrorResponse>
    {
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let client = reqwest::Client::new();
        let url = format!("{}/v1/mcp/asset-type", base_url);

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
                .and_then(|data| data.get("data"))
                .ok_or_else(|| McpErrorResponse::new("404".to_string(), "Missing 'data' field".to_string()))?;

            // Deserialize the "resme" field into ResMeDto
            let res_list_dto: ResListAssestTypeDto = serde_json::from_value(resme_value.clone()).map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse '': {}", e))
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
    } // end of get_all_asset_type
}