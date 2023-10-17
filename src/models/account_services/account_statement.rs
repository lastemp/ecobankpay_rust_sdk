use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AccountStatementDataInputDetails {
    affiliate_code: String,
    corporate_id: String,
    account_number: String,
    start_date: String,
    end_date: String,
    secure_hash: String,
}

impl AccountStatementDataInputDetails {
    pub fn new(
        affiliate_code: String,
        corporate_id: String,
        account_number: String,
        start_date: String,
        end_date: String,
        secure_hash: String,
    ) -> Result<Self, String> {
        if affiliate_code.is_empty() || affiliate_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("affiliate code is empty"));
        }
        // affiliate_code has a length of 3 characters
        else if affiliate_code.len() == 3 {
            // affiliate_code is valid
        } else {
            return Err(String::from("affiliate code has invalid length"));
        }

        if corporate_id.is_empty() || corporate_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("corporate id is empty"));
        }
        // corporate_id has a length of 4 characters
        else if corporate_id.trim().len() == 4 {
            // corporate_id is valid
        } else {
            return Err(String::from("corporate id has invalid length"));
        }

        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a length between 9 and 40 digits
        else if account_number.trim().len() >= 9 && account_number.trim().len() <= 40 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        if start_date.is_empty() || start_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("start date is empty"));
        }
        // start_date has a length of 8 characters i.e YYYYMMDD, 20200301
        else if start_date.trim().len() == 8 {
            // start_date is valid
        } else {
            return Err(String::from("start date has invalid length"));
        }

        if end_date.is_empty() || end_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("end date is empty"));
        }
        // end_date has a length of 8 characters i.e YYYYMMDD, 20200316
        else if end_date.trim().len() == 8 {
            // end_date is valid
        } else {
            return Err(String::from("end date has invalid length"));
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
            affiliate_code,
            corporate_id,
            account_number,
            start_date,
            end_date,
            secure_hash,
        })
    }

    pub fn get_affiliate_code(&self) -> String {
        let affiliate_code = &self.affiliate_code;
        affiliate_code.to_string()
    }

    pub fn get_corporate_id(&self) -> String {
        let corporate_id = &self.corporate_id;
        corporate_id.to_string()
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }

    pub fn get_start_date(&self) -> String {
        let start_date = &self.start_date;
        start_date.to_string()
    }

    pub fn get_end_date(&self) -> String {
        let end_date = &self.end_date;
        end_date.to_string()
    }

    pub fn get_secure_hash(&self) -> String {
        let secure_hash = &self.secure_hash;
        secure_hash.to_string()
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountStatementData {
    pub affiliateCode: String,
    pub corporateId: String,
    pub accountNumber: String,
    pub startDate: String,
    pub endDate: String,
    pub secureHash: String,
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ResponseContentData {
    pub acccy: Option<String>,
    pub drcrind: Option<String>,
    pub trnrefno: Option<String>,
    pub paidin: Option<String>,
    pub paidout: Option<String>,
    pub valuedate: Option<String>,
    pub lcyamount1: Option<String>,
    pub narrative: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountStatementResponseData {
    pub response_code: Option<i16>,
    pub response_message: Option<String>,
    pub response_content: Vec<ResponseContentData>,
    pub response_timestamp: Option<String>,
}
