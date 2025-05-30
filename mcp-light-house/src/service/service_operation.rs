use std::sync::Arc;

use crate::domain::repository::req_impl::ServiceOperationFeatureReqImpl;






pub struct ServiceOperationUsecase{
    service_operation_repository: Arc<dyn ServiceOperationFeatureReqImpl + Send + Sync + 'static>
}



impl ServiceOperationUsecase {
    pub fn new(service_operation_repository: Arc<dyn ServiceOperationFeatureReqImpl + Send + Sync + 'static>) -> Self {
        ServiceOperationUsecase {
            service_operation_repository,
        }
    }

    pub async fn get_all_contact(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResListContactDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.service_operation_repository.get_all_contact(user_token).await
    }

    pub async fn get_all_expense(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResListExpenseDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.service_operation_repository.get_all_expense(user_token).await
    }

    pub async fn get_all_asset(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResListAssetDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.service_operation_repository.get_all_asset(user_token).await
    }
}