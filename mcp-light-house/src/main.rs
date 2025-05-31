use std::sync::Arc;

use dotenvy::dotenv;
use anyhow::Result;
use mcp_light_house::{config::{token_config::load}, infrastructure::{mcp_handler::init_handler::MCPHandler, repository::{auth_repo::AuthRepositoryImpl, service_operation_repo::ServiceOperationRepositoryImpl, service_type_repo::ServiceTypeRepositoryImpl, transaction_repo::TransactionRepositoryImpl}}, service::{auth::AuthUsecase, service_operation::ServiceOperationUsecase, service_type::ServiceTypeUsecase, transaction::TransactionUsecase}};
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::EnvFilter;



#[tokio::main]
async fn main() -> Result<()> {
    let config = load().expect("Failed to load configuration");
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP Light House...");
    
    let auth_usecase = {
        let auth_repo = AuthRepositoryImpl::new(config.clone());
        AuthUsecase::new(Arc::new(auth_repo))
    };

    let transaction_usecase = {
        let transaction_repo = TransactionRepositoryImpl::new(config.clone());
        TransactionUsecase::new(Arc::new(transaction_repo))
    };

    let service_operation_usecase = {
        let service_operation_repo = ServiceOperationRepositoryImpl::new(config.clone());
        ServiceOperationUsecase::new(Arc::new(service_operation_repo))
    };

    let service_type_usecase = {
        let service_type_repo = ServiceTypeRepositoryImpl::new(config.clone());
        ServiceTypeUsecase::new(Arc::new(service_type_repo))
    };

    let service = MCPHandler::new(
        Arc::new(auth_usecase), 
        Arc::new(transaction_usecase), 
        Arc::new(service_type_usecase), 
        Arc::new(service_operation_usecase))
        .serve(stdio())
        .await
        .inspect_err(|e| tracing::error!("Failed to start MCP Light House service: {}", e))?;
    service.waiting().await.inspect_err(|e| tracing::error!("Failed to start MCP Light House service: {}", e))?;
    tracing::info!("MCP Light House service started successfully");
    

        
        
        
    
    


    Ok(())
}
