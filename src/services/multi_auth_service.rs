use crate::error::Result;
use crate::models::admin_user_model::AdminUserModel;
use crate::providers::auth_provider::{AuthProvider, AuthenticationResult};
use crate::providers::avored_database_provider::DB;
use std::sync::Arc;
use tracing::{error, info, warn};

pub struct MultiAuthService {
    providers: Vec<Arc<dyn AuthProvider>>,
}

impl MultiAuthService {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Arc<dyn AuthProvider>) {
        if provider.is_enabled() {
            info!(
                "Adding authentication provider: {}",
                provider.provider_name()
            );
            self.providers.push(provider);
        } else {
            info!(
                "Skipping disabled authentication provider: {}",
                provider.provider_name()
            );
        }
    }

    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
        db: &DB,
        preferred_provider: Option<&str>,
    ) -> Result<AdminUserModel> {
        if self.providers.is_empty() {
            error!("No authentication providers configured");
            return Err(crate::error::Error::Generic(
                "No authentication providers available".to_string(),
            ));
        }

        // If a preferred provider is specified, try it first
        if let Some(provider_name) = preferred_provider {
            if let Some(provider) = self
                .providers
                .iter()
                .find(|p| p.provider_name() == provider_name)
            {
                info!(
                    "Attempting authentication with preferred provider: {}",
                    provider_name
                );
                match provider.authenticate(username, password, db).await? {
                    AuthenticationResult::Success(user) => {
                        info!(
                            "Authentication successful with preferred provider: {}",
                            provider_name
                        );
                        return Ok(user);
                    }
                    AuthenticationResult::Failed(reason) => {
                        warn!(
                            "Authentication failed with preferred provider {}: {}",
                            provider_name, reason
                        );
                    }
                    AuthenticationResult::UserNotFound => {
                        info!("User not found in preferred provider: {}", provider_name);
                    }
                }
            } else {
                warn!(
                    "Preferred provider '{}' not found or not enabled",
                    provider_name
                );
            }
        }

        // Try all providers in order
        let mut last_error = String::new();
        let mut user_found_somewhere = false;

        for provider in &self.providers {
            // Skip the preferred provider if we already tried it
            if let Some(preferred) = preferred_provider {
                if provider.provider_name() == preferred {
                    continue;
                }
            }

            info!(
                "Attempting authentication with provider: {}",
                provider.provider_name()
            );

            match provider.authenticate(username, password, db).await? {
                AuthenticationResult::Success(user) => {
                    info!(
                        "Authentication successful with provider: {}",
                        provider.provider_name()
                    );
                    return Ok(user);
                }
                AuthenticationResult::Failed(reason) => {
                    warn!(
                        "Authentication failed with provider {}: {}",
                        provider.provider_name(),
                        reason
                    );
                    last_error = reason;
                    user_found_somewhere = true; // User exists but auth failed
                }
                AuthenticationResult::UserNotFound => {
                    info!("User not found in provider: {}", provider.provider_name());
                }
            }
        }

        // If we get here, authentication failed with all providers
        if user_found_somewhere {
            error!(
                "Authentication failed for user {} with all providers. Last error: {}",
                username, last_error
            );
            Err(crate::error::Error::Unauthenticated(last_error))
        } else {
            error!("User {} not found in any authentication provider", username);
            Err(crate::error::Error::Unauthenticated(
                "User not found".to_string(),
            ))
        }
    }

    pub fn get_enabled_providers(&self) -> Vec<&str> {
        self.providers.iter().map(|p| p.provider_name()).collect()
    }

    pub fn has_provider(&self, provider_name: &str) -> bool {
        self.providers
            .iter()
            .any(|p| p.provider_name() == provider_name)
    }
}

impl Default for MultiAuthService {
    fn default() -> Self {
        Self::new()
    }
}
