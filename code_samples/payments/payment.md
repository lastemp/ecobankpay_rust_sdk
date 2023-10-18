# payment

This API enables partners to transfer money from their account to various beneficiaries through divers methods.

## main.rs

This should contain below code:

```rust
pub mod payments {
    pub mod payment;
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

    // payment
    let x = payments::payment::test_payment(consumer_key, consumer_secret, _env);
	
    x.await;
}
```

## payment.rs

This module contains the function test_payment:

```rust
use ecobankpay_rust_sdk::models::payments::payment::{
    BillPaymentPaymentRequestDetails, DomesticParameterDetails, DomesticPaymentRequestDetails,
    InterBankPaymentRequestDetails, PaymentDataInputDetails, PaymentHeaderDetails,
    PaymentResponseData, TokenPaymentRequestDetails,
};
use ecobankpay_rust_sdk::EcobankPayGateway;

pub async fn test_payment(consumer_key: String, consumer_secret: String, _env: String) {
    let _result = EcobankPayGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(ecobankpay) = _result {
        let client_id = String::from("***");
        let batch_sequence: u8 = 0;
        let batch_amount: f32 = 0.0;
        let transaction_amount: f32 = 0.0;
        let batch_id = String::from("***");
        let transaction_id = String::from("***");
        let debit_type = String::from("***");
        let affiliate_code = String::from("***");
        let total_batches: u16 = 0;
        let execution_date = String::from("***");

        let _result = PaymentHeaderDetails::new(
            client_id,
            batch_sequence,
            batch_amount,
            transaction_amount,
            batch_id,
            transaction_id,
            debit_type,
            affiliate_code,
            total_batches,
            execution_date,
        );

        if let Ok(payment_header) = _result {
            let credit_account_no = String::from("***");
            let debit_account_branch = String::from("***");
            let debit_account_type = String::from("***");
            let credit_account_branch = String::from("***");
            let credit_account_type = String::from("***");
            let _amount: f32 = 0.0;
            let _ccy = String::from("***");

            let _result = DomesticParameterDetails::new(
                credit_account_no,
                debit_account_branch,
                debit_account_type,
                credit_account_branch,
                credit_account_type,
                _amount,
                _ccy,
            );

            if let Ok(param_domestic) = _result {
                let request_id = String::from("***");
                let mut param_list_domestic: Vec<DomesticParameterDetails> = Vec::new();
                let _amount: f32 = 0.0;
                let _currency = String::from("***");
                let _status = String::from("***");
                let rate_type = String::from("***");

                param_list_domestic.push(param_domestic);

                let _result = DomesticPaymentRequestDetails::new(
                    request_id,
                    param_list_domestic,
                    _amount,
                    _currency,
                    _status,
                    rate_type,
                );

                if let Ok(_domestic) = _result {
                    let mut extension_domestic: Vec<DomesticPaymentRequestDetails> = Vec::new();
                    let extension_token: Vec<TokenPaymentRequestDetails> = Vec::new();
                    let extension_interbank: Vec<InterBankPaymentRequestDetails> = Vec::new();
                    let extension_billpayment: Vec<BillPaymentPaymentRequestDetails> = Vec::new();
                    let secure_hash = String::from("***");

                    extension_domestic.push(_domestic);

                    let _result = PaymentDataInputDetails::new(
                        payment_header,
                        extension_domestic,
                        extension_token,
                        extension_interbank,
                        extension_billpayment,
                        secure_hash,
                    );

                    if let Ok(payment_details) = _result {
                        let _output = ecobankpay.payment(payment_details);
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