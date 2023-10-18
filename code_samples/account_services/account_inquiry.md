# account_inquiry

This API invokes account inquiry.

## main.rs

This should contain below code:

```rust
pub mod account_services {
    pub mod account_inquiry;
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

    // account_inquiry
    let x = account_services::account_inquiry::test_enquire_account_inquiry(
        consumer_key,
        consumer_secret,
        _env,
    );
	
    x.await;
}
```

## account_inquiry.rs

This module contains the function test_enquire_account_inquiry:

```rust
use ecobankpay_rust_sdk::models::account_services::account_inquiry::{
    AccountInquiryDataInputDetails, AccountInquiryResponseData,
};
use ecobankpay_rust_sdk::EcobankPayGateway;

pub async fn test_enquire_account_inquiry(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = EcobankPayGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(ecobankpay) = _result {
        let request_id = String::from("***");
        let affiliate_code = String::from("***");
        let account_no = String::from("***");
        let client_id = String::from("***");
        let company_name = String::from("***");
        let secure_hash = String::from("***");

        let _result = AccountInquiryDataInputDetails::new(
            request_id,
            affiliate_code,
            account_no,
            client_id,
            company_name,
            secure_hash,
        );

        if let Ok(account_details) = _result {
            let _output = ecobankpay.enquire_account_inquiry(account_details);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
            } else if let Err(e) = _result {
                println!("{:?}", e);
            } else {
                println!("Unexpected error occured during processing");
            }
        } else if let Err(e) = _result {
            println!("{:?}", e);
        } else {
            println!("Unexpected error occured during processing");
        }
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
```
