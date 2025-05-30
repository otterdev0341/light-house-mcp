use std::sync::Arc;

use crate::domain::repository::req_impl::AuthFeatureReqImpl;





#[derive(Clone)]
pub struct AuthUsecase{
    auth_repository: Arc<dyn AuthFeatureReqImpl + Send + Sync + 'static>,
}

impl AuthUsecase{
    pub fn new(auth_repository: Arc<dyn AuthFeatureReqImpl + Send + Sync + 'static>) -> Self {
        AuthUsecase {
            auth_repository,
        }
    }

    pub async fn get_resme(&self, user_token: &str) -> Result<crate::domain::value_object::json_response::ResMeDto, crate::domain::value_object::error_response::McpErrorResponse> {
        self.auth_repository.get_resme(user_token).await
    }
}