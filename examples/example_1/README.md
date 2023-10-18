# example_1

This is a full working example which uses the [ecobankpay_rust_sdk](https://github.com/lastemp/ecobankpay_rust_sdk).
The API endpoints provided by EcobankPay API Gateway includes; QR Code Payments, Card Payments, Payments and Account Services (https://developer.ecobank.com/app/index.xhtml).

The example has below listed dependencies:
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [tokio](https://github.com/tokio-rs/tokio) A runtime for writing reliable, asynchronous applications
- [ecobankpay_rust_sdk](https://github.com/lastemp/ecobankpay_rust_sdk) an sdk to seamlessly integrate with EcobankPay Gateway

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../example_1
```

1. Using a different terminal execute requests by un-commenting code for the spefific function on main.rs. For example:

   ```rust
    pub mod account_services {
    pub mod account_balance;
	}

	// SANDBOX
	const CONSUMER_KEY_SANDBOX: &str = "***";
	const CONSUMER_SECRET_SANDBOX: &str = "***";

	const ENVIRONMENT: &str = "sandbox";

	#[tokio::main]
	async fn main() {
		let consumer_key = CONSUMER_KEY_SANDBOX.to_string();
		let consumer_secret = CONSUMER_SECRET_SANDBOX.to_string();
		let _env = ENVIRONMENT.to_string();

		// account_balance
		let x = account_services::account_balance::test_enquire_account_balance(
			consumer_key,
			consumer_secret,
			_env,
		);
		
		x.await;
	}
   ```

1. Run the application:

   ```sh
   cargo run
   ```
