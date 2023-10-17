use reqwest::StatusCode;

use crate::{
    models::payments::payment::{PaymentDataInputDetails, PaymentResponseData},
    util::util::{build_account_statement_data, build_headers, build_payment_data},
};

pub async fn payment(
    payment_details: PaymentDataInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<PaymentResponseData, String> {
    // Lets build the request params as a struct
    let payment_data = build_payment_data(&payment_details);

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&payment_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                // 200
                match response.json::<PaymentResponseData>().await {
                    Ok(account_response_data) => {
                        // Handle success case

                        return Ok(account_response_data);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
