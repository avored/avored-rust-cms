use crate::core::{
    application::dtos::{LoginCommand, LoginResult},
    domain::{entities::User, repositories::AuthRepository},
};

pub struct LoginUser<R>
where
    R: AuthRepository,
{
    repository: R,
}

impl<R> LoginUser<R>
where
    R: AuthRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, command: LoginCommand) -> LoginResult {
        let user = self
            .repository
            .authenticate(&command.email, &command.password)
            .unwrap_or_else(|| User::new("", "", ""));

        if user.email.is_empty() {
            return LoginResult {
                token: String::new(),
                user: User::new("", "", ""),
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
