use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AccountBalanceDataInputDetails {
    request_id: String,
    affiliate_code: String,
    account_no: String,
    client_id: String,
    company_name: String,
    secure_hash: String,
}

impl AccountBalanceDataInputDetails {
    pub fn new(
        request_id: String,
        affiliate_code: String,
        account_no: String,
        client_id: String,
        company_name: String,
        secure_hash: String,
    ) -> Result<Self, String> {
        if request_id.is_empty() || request_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("request id is empty"));
        }
        // request_id has a length between 10 and 13 characters
        else if request_id.trim().len() >= 10 && request_id.trim().len() <= 13 {
            // request_id is valid
        } else {
            return Err(String::from("request id has invalid length"));
        }

        if affiliate_code.is_empty() || affiliate_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("affiliate code is empty"));
        }
        // affiliate_code has a length of 3 characters
        else if affiliate_code.len() == 3 {
            // affiliate_code is valid
        } else {
            return Err(String::from("affiliate code has invalid length"));
        }

        if account_no.is_empty() || account_no.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account no is empty"));
        }
        // account_no has a length between 9 and 40 digits
        else if account_no.trim().len() >= 9 && account_no.trim().len() <= 40 {
            // account_no is valid
        } else {
            return Err(String::from("account no has invalid length"));
        }

        if client_id.is_empty() || client_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("client id is empty"));
        }
        // client_id has a length between 10 and 20 characters
        else if client_id.trim().len() >= 10 && client_id.trim().len() <= 20 {
            // client_id is valid
        } else {
            return Err(String::from("client id has invalid length"));
        }

        if company_name.is_empty() || company_name.replace(" ", "").trim().len() == 0 {
            return Err(String::from("company name is empty"));
        }
        // company_name has a max length of 50 characters
        else if company_name.trim().len() > 0 && company_name.trim().len() <= 50 {
            // company_name is valid
        } else {
            return Err(String::from("company name has invalid length"));
        }

        if secure_hash.is_empty() || secure_hash.replace(" ", "").trim().len() == 0 {
            return Err(String::from("secure hash is empty"));
        }
        // secure_hash has a max length of 128 characters
        else if secure_hash.trim().len() > 0 && secure_hash.trim().len() <= 128 {
            // secure_hash is valid
        } else {
            return Err(String::from("secure hash has invalid length"));
        }

        Ok(Self {
            request_id,
            affiliate_code,
            account_no,
            client_id,
            company_name,
            secure_hash,
        })
    }

    pub fn get_request_id(&self) -> String {
        let request_id = &self.request_id;
        request_id.to_string()
    }

    pub fn get_affiliate_code(&self) -> String {
        let affiliate_code = &self.affiliate_code;
        affiliate_code.to_string()
    }

    pub fn get_account_no(&self) -> String {
        let account_no = &self.account_no;
        account_no.to_string()
    }

    pub fn get_client_id(&self) -> String {
        let client_id = &self.client_id;
        client_id.to_string()
    }

    pub fn get_company_name(&self) -> String {
        let company_name = &self.company_name;
        company_name.to_string()
    }

    pub fn get_secure_hash(&self) -> String {
        let secure_hash = &self.secure_hash;
        secure_hash.to_string()
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountBalanceData {
    pub requestId: String,
    pub affiliateCode: String,
    pub accountNo: String,
    pub clientId: String,
    pub companyName: String,
    pub secureHash: String,
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct HostHeaderInfoData {
    pub sourceCode: Option<String>,
    pub requestId: Option<String>,
    pub affiliateCode: Option<String>,
    pub responseCode: Option<String>,
    pub responseMessage: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ResponseContentData {
    pub hostHeaderInfo: HostHeaderInfoData,
    pub accountNo: Option<String>,
    pub responseCode: Option<i16>,
    pub responseMessage: Option<String>,
    pub accountName: Option<String>,
    pub ccy: Option<String>,
    pub branchCode: Option<String>,
    pub customerID: Option<String>,
    pub availableBalance: Option<f32>,
    pub currentBalance: Option<f32>,
    pub odlimit: Option<f32>,
    pub accountType: Option<String>,
    pub accountClass: Option<String>,
    pub accountStatus: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountBalanceResponseData {
    pub response_code: Option<i16>,
    pub response_message: Option<String>,
    pub response_content: ResponseContentData,
    pub response_timestamp: Option<String>,
}
