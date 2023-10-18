pub mod account_services {
    pub mod account_balance;
    pub mod account_inquiry;
    pub mod account_statement;
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

    /*
    // account_inquiry
    let x = account_services::account_inquiry::test_enquire_account_inquiry(
        consumer_key,
        consumer_secret,
        _env,
    );

    // account_statement
    let x = account_services::account_statement::test_enquire_account_inquiry(
        consumer_key,
        consumer_secret,
        _env,
    );
    */

    x.await;
}
