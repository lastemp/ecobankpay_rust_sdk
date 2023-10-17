use reqwest::StatusCode;

use crate::{
    models::account_services::account_inquiry::{
        AccountInquiryDataInputDetails, AccountInquiryResponseData,
    },
    util::util::{build_account_inquiry_data, build_headers},
};

pub async fn enquire(
    account_details: AccountInquiryDataInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<AccountInquiryResponseData, String> {
    // Lets build the request params as a struct
    let account_data = build_account_inquiry_data(&account_details);

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&account_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                // 200
                match response.json::<AccountInquiryResponseData>().await {
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
