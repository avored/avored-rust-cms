use crate::infra::repositories::surreal_user_repository::SurrealUserRepository;
use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MiscRepository: Send + Sync {
    async fn setup(&self) -> Result<bool>;
}


#[async_trait]
impl MiscRepository for SurrealUserRepository  {
    async fn setup(&self) -> Result<bool> 
    {    
        println!("Setting up database");   
        Ok(false)
    }
}
