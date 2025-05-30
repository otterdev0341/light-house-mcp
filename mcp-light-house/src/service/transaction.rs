use std::sync::Arc;

use crate::domain::repository::req_impl::TransactionFeatureReqimpl;





#[derive(Clone)]
pub struct TransactionUsecase{
    transaction_repository: Arc<dyn TransactionFeatureReqimpl + Send + Sync + 'static>,
}

impl TransactionUsecase {
    pub fn new(transaction_repository: Arc<dyn TransactionFeatureReqimpl + Send + Sync + 'static>) -> Self {
        TransactionUsecase {
            transaction_repository,
        }
    }

    pub async fn get_all_income(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResListIncomeDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.transaction_repository.get_all_income(user_token).await
    }

    pub async fn get_all_payment(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResListPaymentDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.transaction_repository.get_all_payment(user_token).await
    }

    pub async fn get_all_transfer(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResListTransferDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.transaction_repository.get_all_transfer(user_token).await
    }
}