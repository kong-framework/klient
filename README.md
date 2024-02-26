# Klient

`kong` __HTTP__ client

## Example

``` Rust
let client = Klient::new_client().unwrap();
let klient = Klient {
	client,
	accounts_endpoint: "http://localhost:3000/accounts".to_string(),
};

// create admin new account
let account = AccountCreationInput {
	username: "admin".to_string(),
	email: Some("admin@example.com".to_string()),
	password: "1234567890".to_string(),
};

klient.create_account(account);
```
