pub mod models {
    pub mod account_services {
        pub mod account_balance;
        pub mod account_inquiry;
        pub mod account_statement;
    }
    pub mod payments {
        pub mod payment;
    }
    pub mod authorization {
        pub mod auth_token;
    }
}

mod util {
    pub mod util;
}

mod authorization {
    pub mod generate_auth_token;
}

pub mod account_services {
    pub mod account_balance;
    pub mod account_inquiry;
    pub mod account_statement;
}

pub mod payments {
    pub mod payment;
}

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

use models::account_services::{
    account_balance::{AccountBalanceDataInputDetails, AccountBalanceResponseData},
    account_inquiry::{AccountInquiryDataInputDetails, AccountInquiryResponseData},
    account_statement::{AccountStatementDataInputDetails, AccountStatementResponseData},
};

use models::payments::{
    payment::{PaymentDataInputDetails, PaymentResponseData},
};

const AUTHORISATION_BEARER: &str = "Bearer";
const GRANT_TYPE: &str = "client_credentials";

const AUTH_TOKEN_URL_SANDBOX: &str = "https://";
const AUTH_TOKEN_URL_PROD: &str = "https://";

const ACCOUNT_BALANCE_URL_SANDBOX: &str = "https://developer.ecobank.com/corporateapi/merchant/accountbalance";
const ACCOUNT_BALANCE_URL_PROD: &str = "https://developer.ecobank.com/corporateapi/merchant/accountbalance";

const ACCOUNT_INQUIRY_URL_SANDBOX: &str = "https://developer.ecobank.com/corporateapi/merchant/accountinquiry";
const ACCOUNT_INQUIRY_URL_PROD: &str = "https://developer.ecobank.com/corporateapi/merchant/accountinquiry";

const ACCOUNT_STATEMENT_URL_SANDBOX: &str = "https://developer.ecobank.com/corporateapi/merchant/statement";
const ACCOUNT_STATEMENT_URL_PROD: &str = "https://developer.ecobank.com/corporateapi/merchant/statement";

const PAYMENT_URL_SANDBOX: &str = "https://developer.ecobank.com/corporateapi/merchant/payment";
const PAYMENT_URL_PROD: &str = "https://developer.ecobank.com/corporateapi/merchant/payment";

#[derive(Debug)]
pub struct EcobankPayGateway {
    grant_type: String,
    consumer_key: String,
    consumer_secret: String,
    auth_token_url: String,
    account_balance_url: String,
    account_inquiry_url: String,
    account_statement_url: String,
    payment_url: String,
}

impl EcobankPayGateway {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        _env: String,
    ) -> Result<Self, String> {
        if consumer_key.is_empty() || consumer_key.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer key is empty"));
        }

        if consumer_secret.is_empty() || consumer_secret.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer secret is empty"));
        }

        if _env.is_empty() || _env.replace(" ", "").trim().len() == 0 {
            return Err(String::from("_env is empty"));
        }

        if _env.eq_ignore_ascii_case(&String::from("sandbox"))
            || _env.eq_ignore_ascii_case(&String::from("prod"))
        {
            // valid _env
        } else {
            return Err(String::from("invalid env"));
        }

        let grant_type = GRANT_TYPE.to_string();

        let auth_token_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            AUTH_TOKEN_URL_PROD.to_string()
        } else {
            AUTH_TOKEN_URL_SANDBOX.to_string()
        };

        let account_balance_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_BALANCE_URL_PROD.to_string()
        } else {
            ACCOUNT_BALANCE_URL_SANDBOX.to_string()
        };

        let account_inquiry_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_INQUIRY_URL_PROD.to_string()
        } else {
            ACCOUNT_INQUIRY_URL_SANDBOX.to_string()
        };

        let account_statement_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_STATEMENT_URL_PROD.to_string()
        } else {
            ACCOUNT_STATEMENT_URL_SANDBOX.to_string()
        };

        let payment_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            PAYMENT_URL_PROD.to_string()
        } else {
            PAYMENT_URL_SANDBOX.to_string()
        };

        Ok(Self {
            grant_type,
            consumer_key,
            consumer_secret,
            auth_token_url,
            account_balance_url,
            account_inquiry_url,
            account_statement_url,
            payment_url,
        })
    }

    fn get_api_key(&self) -> String {
        let consumer_key = &self.consumer_key;
        let consumer_secret = &self.consumer_secret;
        let mut password: String = consumer_key.to_string();
        let k = ":"; // Separator
        password.push_str(k);
        password.push_str(&consumer_secret);
        let encodedpassword = general_purpose::STANDARD.encode(password);

        let mut api_key = String::from("Basic");
        let k = " "; // Separator
        api_key.push_str(k);
        api_key.push_str(&encodedpassword);

        api_key
    }

    fn parse_auth_token(&self, access_token_result: String) -> String {
        let access_token: String = if !access_token_result.is_empty()
            && access_token_result.replace(" ", "").trim().len() > 0
        {
            let mut access_token = AUTHORISATION_BEARER.to_string();
            let k = " "; // Separator
            access_token.push_str(k);
            access_token.push_str(&access_token_result);

            access_token
        } else {
            String::from("")
        };

        access_token
    }

    async fn get_auth_token(&self) -> std::result::Result<String, String> {
        let grant_type = &self.grant_type;
        let api_key = &self.get_api_key();
        let api_url = &self.auth_token_url;

        let _result = authorization::generate_auth_token::get_auth_token(
            grant_type.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        )
        .await;

        _result
    }

    pub async fn enquire_account_balance(
        &self,
        account_details: AccountBalanceDataInputDetails,
    ) -> std::result::Result<AccountBalanceResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_balance_url;
                
                let _result = account_services::account_balance::enquire(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_inquiry(
        &self,
        account_details: AccountInquiryDataInputDetails,
    ) -> std::result::Result<AccountInquiryResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_inquiry_url;
                
                let _result = account_services::account_inquiry::enquire(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_statement(
        &self,
        account_details: AccountStatementDataInputDetails,
    ) -> std::result::Result<AccountStatementResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_statement_url;
                
                let _result = account_services::account_statement::enquire(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn payment(
        &self,
        payment_details: PaymentDataInputDetails,
    ) -> std::result::Result<PaymentResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.payment_url;
                
                let _result = payments::payment::payment(
                    payment_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecobankpay_gateway() {
        let consumer_key = String::from("***");
        let consumer_secret = String::from("***");
        let _env = String::from("sandbox");

        let _result = EcobankPayGateway::new(consumer_key, consumer_secret, _env);
        assert_eq!(_result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_enquire_account_balance() {
        let consumer_key = String::from("***");
        let consumer_secret = String::from("***");
        let _env = String::from("sandbox");

        let _result = EcobankPayGateway::new(consumer_key, consumer_secret, _env);

        if let Ok(scb) = _result {
            let request_id = String::from("***");
            let affiliate_code = String::from("***");
            let account_no = String::from("***");
            let client_id = String::from("***");
            let company_name = String::from("***");
            let secure_hash = String::from("***");

            let _result = AccountBalanceDataInputDetails::new(request_id,
                affiliate_code,
                account_no,
                client_id,
                company_name,
                secure_hash,);

            if let Ok(account_details) = _result {
                let _output = scb.enquire_account_balance(account_details);
                let _result = _output.await;
                assert_eq!(_result.is_ok(), true);
            }
        }
    }
}
