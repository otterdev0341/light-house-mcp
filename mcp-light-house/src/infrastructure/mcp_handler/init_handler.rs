use std::sync::Arc;

use rmcp::{
    Error as McpError, RoleServer, ServerHandler, const_string, model::*, service::RequestContext,
    tool,
};
use serde_json::json;

use crate::service::{auth::AuthUsecase, service_operation::ServiceOperationUsecase, service_type::ServiceTypeUsecase, transaction::TransactionUsecase};





#[derive(Clone)]
pub struct MCPHandler {
    auth_usecase: Arc<AuthUsecase>,
    transaction_usecase: Arc<TransactionUsecase>,
    service_type_usecase: Arc<ServiceTypeUsecase>,
    service_operation_usecase: Arc<ServiceOperationUsecase>, // Assuming this is the correct usecase for service operations
}


#[tool(tool_box)]
impl MCPHandler {

    pub fn new(
        auth_usecase: Arc<AuthUsecase>,
        transaction_usecase: Arc<TransactionUsecase>,
        service_type_usecase: Arc<ServiceTypeUsecase>,
        service_operation_usecase: Arc<ServiceOperationUsecase>,
    ) -> Self {
        MCPHandler {
            auth_usecase,
            transaction_usecase,
            service_type_usecase,
            service_operation_usecase,
        }
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }


    #[tool(description = "Get all income")]
    pub async fn get_resme(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.auth_usecase.get_resme(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_resme

    #[tool(description = "Get all income")]
    pub async fn get_all_income(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.transaction_usecase.get_all_income(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_all_income



    #[tool(description = "Get all payment")]
    pub async fn get_all_payment(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.transaction_usecase.get_all_payment(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_all_payment



    #[tool(description = "Get all transfer")]
    pub async fn get_all_transfer(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.transaction_usecase.get_all_transfer(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_all_transfer

    #[tool(description = "Get all contact type")]
    pub async fn get_all_contact_type(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.service_type_usecase.get_all_contact_type(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_all_contact_type

    #[tool(description = "Get all expense type")]
    pub async fn get_all_expense_type(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.service_type_usecase.get_all_expense_type(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_all_expense_type



    #[tool(description = "Get all asset type")]
    pub async fn get_all_asset_type(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.service_type_usecase.get_all_asset_type(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_all_asset_type



    #[tool(description = "Get all contact")]
    pub async fn get_all_contact(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.service_operation_usecase.get_all_contact(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_all_contact


    #[tool(description = "Get all expense")]
    pub async fn get_all_expense(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.service_operation_usecase.get_all_expense(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_all_expense


    #[tool(description = "Get all asset")]
    pub async fn get_all_asset(
        &self, 
        #[tool(aggr)] user_token: String) -> Result<CallToolResult, McpError> {
        match self.service_operation_usecase.get_all_asset(&user_token).await {
            Ok(res) => {
                if let Ok(res_json) = Content::json(res) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            },
            Err(e) => Err(McpError::internal_error(e.message, None)),
        }
    }// end get_all_asset

}// end impl MCPHandler





const_string!(Echo = "echo");
#[tool(tool_box)]
impl ServerHandler for MCPHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Light - House project that represent data of income expense".to_string()),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                self._create_resource_text("str:////Users/to/some/path/", "cwd"),
                self._create_resource_text("memo://insights", "memo-name"),
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "str:////Users/to/some/path/" => {
                let cwd = "/Users/to/some/path/";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(cwd, uri)],
                })
            }
            "memo://insights" => {
                let memo = "Business Intelligence Memo\n\nAnalysis has revealed 5 key insights ...";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(memo, uri)],
                })
            }
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult {
            next_cursor: None,
            prompts: vec![Prompt::new(
                "example_prompt",
                Some("This is an example prompt that takes one required argument, message"),
                Some(vec![PromptArgument {
                    name: "message".to_string(),
                    description: Some("A message to put in the prompt".to_string()),
                    required: Some(true),
                }]),
            )],
        })
    }

    async fn get_prompt(
        &self,
        GetPromptRequestParam { name, arguments }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        match name.as_str() {
            "example_prompt" => {
                let message = arguments
                    .and_then(|json| json.get("message")?.as_str().map(|s| s.to_string()))
                    .ok_or_else(|| {
                        McpError::invalid_params("No message provided to example_prompt", None)
                    })?;

                let prompt =
                    format!("This is an example prompt with your message here: '{message}'");
                Ok(GetPromptResult {
                    description: None,
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(prompt),
                    }],
                })
            }
            _ => Err(McpError::invalid_params("prompt not found", None)),
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }
}