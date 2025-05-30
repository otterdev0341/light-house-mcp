use crate::domain::value_object::{error_response::McpErrorResponse, json_response::{ResListAssestTypeDto, ResListAssetDto, ResListContactDto, ResListContactTypeDto, ResListExpenseDto, ResListExpenseTypeDto, ResListIncomeDto, ResListPaymentDto, ResListTransferDto, ResMeDto}};



#[async_trait::async_trait]
pub trait AuthFeatureReqImpl{
    async fn get_resme(&self, user_token: &str) -> Result<ResMeDto, McpErrorResponse>;
}


#[async_trait::async_trait]
pub trait TransactionFeatureReqimpl {
    async fn get_all_income(&self, user_token: &str) -> Result<ResListIncomeDto, McpErrorResponse>;
    async fn get_all_payment(&self, user_token: &str) -> Result<ResListPaymentDto, McpErrorResponse>;
    async fn get_all_transfer(&self, user_token: &str) -> Result<ResListTransferDto, McpErrorResponse>;
}

#[async_trait::async_trait]
pub trait ServiceTypeFeatureReqImpl {
    async fn get_all_contact_type(&self, user_token: &str) -> Result<ResListContactTypeDto, McpErrorResponse>;
    async fn get_all_expense_type(&self, user_token: &str) -> Result<ResListExpenseTypeDto, McpErrorResponse>;
    async fn get_all_asset_type(&self, user_token: &str) -> Result<ResListAssestTypeDto, McpErrorResponse>;
}


#[async_trait::async_trait]
pub trait ServiceOperationFeatureReqImpl {
    async fn get_all_contact(&self, user_token: &str) -> Result<ResListContactDto, McpErrorResponse>;
    async fn get_all_expense(&self, user_token: &str) -> Result<ResListExpenseDto, McpErrorResponse>;
    async fn get_all_asset(&self, user_token: &str) -> Result<ResListAssetDto, McpErrorResponse>;
}