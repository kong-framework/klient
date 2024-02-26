//! Kong HTTP client
#![doc(html_favicon_url = "https://kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub use error::KlientError;
use kong_kontrollers::accounts::inputs::AccountCreationInput;
use kong_kontrollers::login::inputs::AccountLoginInput;
use reqwest::{
    blocking::{multipart, Client},
    StatusCode,
};

mod error;

/// Kong HTTP client
pub struct Klient {
    /// HTTP client
    pub client: Client,
    #[cfg(feature = "accounts")]
    /// Accounts route
    pub accounts_endpoint: String,
    #[cfg(feature = "login")]
    /// Login route
    pub login_endpoint: String,
}

impl Klient {
    /// Create new klient
    pub fn new_client() -> Result<Client, KlientError> {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| KlientError::Client)?;
        Ok(client)
    }

    /// Create a new user account
    #[cfg(feature = "accounts")]
    pub fn create_account(&self, account: AccountCreationInput) -> Result<(), KlientError> {
        let res = self
            .client
            .post(&self.accounts_endpoint)
            .json(&account)
            .send()
            .map_err(|_| KlientError::APIConnection)?;

        match res.status() {
            StatusCode::CREATED => Ok(()),
            StatusCode::BAD_REQUEST => Err(KlientError::InvalidInput),
            _ => Err(KlientError::InternalServerError),
        }
    }

    /// Account login
    #[cfg(feature = "login")]
    pub fn login(&self, login_input: AccountLoginInput) -> Result<(), KlientError> {
        let res = self
            .client
            .post(&self.login_endpoint)
            .json(&login_input)
            .send()
            .map_err(|_| KlientError::APIConnection)?;

        match res.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => Err(KlientError::InvalidInput),
            _ => Err(KlientError::InternalServerError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "accounts")]
    #[cfg(feature = "login")]
    fn test_create_account() {
        let client = Klient::new_client().unwrap();
        let klient = Klient {
            client,
            accounts_endpoint: "http://localhost:3000/accounts".to_string(),
            login_endpoint: "http://localhost:3000/login".to_string(),
        };

        // create admin new account
        let account = AccountCreationInput {
            username: "admin".to_string(),
            email: Some("admin@example.com".to_string()),
            password: "1234567890".to_string(),
        };

        if let Ok(_res) = klient.create_account(account) {
            assert!(true);
        } else {
            panic!("Error creating account");
        }

        let login_info = AccountLoginInput {
            username: "admin".to_string(),
            password: "1234567890".to_string(),
        };

        if let Ok(_res) = klient.login(login_info) {
            assert!(true);
        } else {
            panic!("Error creating account");
        }
    }
}
