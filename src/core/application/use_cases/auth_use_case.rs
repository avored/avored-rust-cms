use crate::core::{
    application::dtos::{LoginCommand, LoginResult},
    domain::{entities::User, repositories::AuthRepository},
};

#[derive(Clone)]
pub struct AuthUseCase<R>
where
    R: AuthRepository,
{
    repository: R,
}

impl<R> AuthUseCase<R>
where
    R: AuthRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn auth(&self, command: LoginCommand) -> LoginResult {
        let user = self
            .repository
            .authenticate(&command.email, &command.password)
            .await;

        let user = match user {
            Some(user) => user,
            None => return LoginResult {
                token: String::new(),
                user: User::default(),
                authenticated: false,
            },
        };

        if user.email.is_empty() {
            return LoginResult {
                token: String::new(),
                user: User::default(),
                authenticated: false,
            };
        }

        LoginResult {
            token: format!("demo-token-for-{}", user.id),
            user,
            authenticated: true,
        }
    }
}
