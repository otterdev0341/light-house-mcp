use std::sync::Arc;

use crate::domain::repository::req_impl::ServiceTypeFeatureReqImpl;



#[derive(Clone)]
pub struct ServiceTypeUsecase {
    service_type_repository: Arc<dyn ServiceTypeFeatureReqImpl + Send + Sync + 'static>,
}
impl ServiceTypeUsecase {
    pub fn new(service_type_repository: Arc<dyn ServiceTypeFeatureReqImpl + Send + Sync + 'static>) -> Self {
        ServiceTypeUsecase {
            service_type_repository,
        }
    }

    pub async fn get_all_contact_type(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResListContactTypeDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.service_type_repository.get_all_contact_type(user_token).await
    }

    pub async fn get_all_expense_type(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResListExpenseTypeDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.service_type_repository.get_all_expense_type(user_token).await
    }

    pub async fn get_all_asset_type(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResListAssestTypeDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.service_type_repository.get_all_asset_type(user_token).await
    }
}