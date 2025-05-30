use serde_json::Value;

use crate::{config::token_config::TokenConfig, domain::{repository::req_impl::TransactionFeatureReqimpl, value_object::{error_response::McpErrorResponse, json_response::{ResListIncomeDto, ResListPaymentDto, ResListTransferDto}}}};





#[derive(Clone)]
pub struct TransactionRepositoryImpl{
    token: TokenConfig
}

impl TransactionRepositoryImpl {
    pub fn new(token: TokenConfig) -> Self {
        TransactionRepositoryImpl {
            token
        }
    }
}

#[async_trait::async_trait]
impl TransactionFeatureReqimpl for TransactionRepositoryImpl {
    async fn get_all_income(&self, _user_token: &str) -> Result<ResListIncomeDto, McpErrorResponse> {
        // Implementation for getting all income
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let client = reqwest::Client::new();
        let url = format!("{}/v1/mcp/income", base_url);

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
            let resme_dto: ResListIncomeDto = serde_json::from_value(resme_value.clone()).map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse '': {}", e))
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


    } // end get all

    async fn get_all_payment(&self, _user_token: &str) -> Result<ResListPaymentDto, McpErrorResponse> {
        // Implementation for getting all payment
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let client = reqwest::Client::new();
        let url = format!("{}/v1/mcp/payment", base_url);

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
            let resme_dto: ResListPaymentDto = serde_json::from_value(resme_value.clone()).map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse '': {}", e))
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

    }// end get all payment

    async fn get_all_transfer(&self, _user_token: &str) -> Result<ResListTransferDto, McpErrorResponse> {
        // Implementation for getting all transfer
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let client = reqwest::Client::new();
        let url = format!("{}/v1/mcp/transfer", base_url);

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
            let resme_dto: ResListTransferDto = serde_json::from_value(resme_value.clone()).map_err(|e| {
                McpErrorResponse::new("400".to_string(), format!("Failed to parse '': {}", e))
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

    }// end
}