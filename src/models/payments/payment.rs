use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct PaymentHeaderDetails {
    client_id: String,
    batch_sequence: u8,
    batch_amount: f32,
    transaction_amount: f32,
    batch_id: String,
    //transaction_count: u16,
    //batch_count: u16,
    transaction_id: String,
    debit_type: String,
    affiliate_code: String,
    total_batches: u16,
    execution_date: String,
}

impl PaymentHeaderDetails {
    pub fn new(
        client_id: String,
        batch_sequence: u8,
        batch_amount: f32,
        transaction_amount: f32,
        batch_id: String,
        //transaction_count: u16,
        //batch_count: u16,
        transaction_id: String,
        debit_type: String,
        affiliate_code: String,
        total_batches: u16,
        execution_date: String,
    ) -> Result<Self, String> {
        if client_id.is_empty() || client_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("client id is empty"));
        }
        // client_id has a length of 15 characters
        else if client_id.len() == 15 {
            // client_id is valid
        } else {
            return Err(String::from("client id has invalid length"));
        }

        if batch_sequence > 0 && batch_sequence < 10 {
            // batch_sequence is valid
        } else {
            return Err(String::from("batch sequence has invalid value"));
        }

        if batch_amount > 0.0 {
            // batch_amount is valid
        } else {
            return Err(String::from("batch amount has invalid value"));
        }

        if transaction_amount > 0.0 {
            // transaction_amount is valid
        } else {
            return Err(String::from("transaction amount has invalid value"));
        }

        if batch_id.is_empty() || batch_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("batch id is empty"));
        }
        // batch_id has a length of 15 characters
        else if batch_id.len() == 15 {
            // batch_id is valid
        } else {
            return Err(String::from("batch id has invalid length"));
        }

        if transaction_id.is_empty() || transaction_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("transaction id is empty"));
        }
        // transaction_id has a length of 15 characters
        else if transaction_id.len() == 15 {
            // transaction_id is valid
        } else {
            return Err(String::from("transaction id has invalid length"));
        }

        if debit_type.is_empty() || debit_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("debit type is empty"));
        }
        // debit_type has a value of Multiple
        else if debit_type.eq_ignore_ascii_case(&String::from("multiple")) {
            // debit_type is valid
        } else {
            return Err(String::from("debit type has invalid value"));
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

        if total_batches == 1 || total_batches == 2 {
            // total_batches is valid
        } else {
            return Err(String::from("total batches has invalid value"));
        }

        if execution_date.is_empty() || execution_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("execution date is empty"));
        }
        // execution_date has a length of 19 characters
        else if execution_date.len() == 19 {
            // execution_date is valid
        } else {
            return Err(String::from("execution date has invalid length"));
        }

        Ok(Self {
            client_id,
            batch_sequence,
            batch_amount,
            transaction_amount,
            batch_id,
            //transaction_count,
            //batch_count,
            transaction_id,
            debit_type,
            affiliate_code,
            total_batches,
            execution_date,
        })
    }

    pub fn get_client_id(&self) -> String {
        let client_id = &self.client_id;
        client_id.to_string()
    }

    pub fn get_batch_sequence(&self) -> u8 {
        let batch_sequence = &self.batch_sequence;
        *batch_sequence
    }

    pub fn get_batch_amount(&self) -> f32 {
        let batch_amount = &self.batch_amount;
        *batch_amount
    }

    pub fn get_transaction_amount(&self) -> f32 {
        let transaction_amount = &self.transaction_amount;
        *transaction_amount
    }

    pub fn get_batch_id(&self) -> String {
        let batch_id = &self.batch_id;
        batch_id.to_string()
    }
    /*
    pub fn get_transaction_count(&self) -> u16 {
        let transaction_count = &self.transaction_count;
        *transaction_count
    }

    pub fn get_batch_count(&self) -> u16 {
        let batch_count = &self.batch_count;
        *batch_count
    }
    */
    pub fn get_transaction_id(&self) -> String {
        let transaction_id = &self.transaction_id;
        transaction_id.to_string()
    }

    pub fn get_debit_type(&self) -> String {
        let debit_type = &self.debit_type;
        debit_type.to_string()
    }

    pub fn get_affiliate_code(&self) -> String {
        let affiliate_code = &self.affiliate_code;
        affiliate_code.to_string()
    }

    pub fn get_total_batches(&self) -> u16 {
        let total_batches = &self.total_batches;
        *total_batches
    }

    pub fn get_execution_date(&self) -> String {
        let execution_date = &self.execution_date;
        execution_date.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DomesticParameterDetails {
    credit_account_no: String,
    debit_account_branch: String,
    debit_account_type: String,
    credit_account_branch: String,
    credit_account_type: String,
    _amount: f32,
    _ccy: String,
}

impl DomesticParameterDetails {
    pub fn new(
        credit_account_no: String,
        debit_account_branch: String,  // optional
        debit_account_type: String,    // optional
        credit_account_branch: String, // optional
        credit_account_type: String,   // optional
        _amount: f32,
        _ccy: String,
    ) -> Result<Self, String> {
        if credit_account_no.is_empty() || credit_account_no.replace(" ", "").trim().len() == 0 {
            return Err(String::from("credit account no is empty"));
        }
        // credit_account_no has a length of 16 characters
        else if credit_account_no.len() == 16 {
            // credit_account_no is valid
        } else {
            return Err(String::from("credit account no has invalid length"));
        }

        if debit_account_branch.is_empty()
            || debit_account_branch.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("debit account branch is empty"));
        }
        // debit_account_branch has a max length of 20 characters
        else if debit_account_branch.len() > 0 && debit_account_branch.len() <= 20 {
            // debit_account_branch is valid
        } else {
            return Err(String::from("debit account branch has invalid length"));
        }

        if debit_account_type.is_empty() || debit_account_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("debit account type is empty"));
        }
        // debit_account_type has a value of Corporate
        else if debit_account_type.eq_ignore_ascii_case(&String::from("corporate")) {
            // debit_account_type is valid
        } else {
            return Err(String::from("debit account type has invalid value"));
        }

        if credit_account_branch.is_empty()
            || credit_account_branch.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("credit account branch is empty"));
        }
        // credit_account_branch has a max length of 20 characters
        else if credit_account_branch.len() > 0 && credit_account_branch.len() <= 20 {
            // credit_account_branch is valid
        } else {
            return Err(String::from("credit account branch has invalid length"));
        }

        if credit_account_type.is_empty() || credit_account_type.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("credit account type is empty"));
        }
        // credit_account_type has a value of Corporate
        else if credit_account_type.eq_ignore_ascii_case(&String::from("corporate")) {
            // credit_account_type is valid
        } else {
            return Err(String::from("credit account type has invalid value"));
        }

        if _amount > 0.0 {
            // _amount is valid
        } else {
            return Err(String::from("amount has invalid value"));
        }

        if _ccy.is_empty() || _ccy.replace(" ", "").trim().len() == 0 {
            return Err(String::from("ccy is empty"));
        }
        // _ccy has a length of 3 characters
        else if _ccy.len() == 3 {
            // _ccy is valid
        } else {
            return Err(String::from("ccy has invalid length"));
        }

        Ok(Self {
            credit_account_no,
            debit_account_branch,
            debit_account_type,
            credit_account_branch,
            credit_account_type,
            _amount,
            _ccy,
        })
    }

    pub fn get_credit_account_no(&self) -> String {
        let credit_account_no = &self.credit_account_no;
        credit_account_no.to_string()
    }

    pub fn get_debit_account_branch(&self) -> String {
        let debit_account_branch = &self.debit_account_branch;
        debit_account_branch.to_string()
    }

    pub fn get_debit_account_type(&self) -> String {
        let debit_account_type = &self.debit_account_type;
        debit_account_type.to_string()
    }
    pub fn get_credit_account_branch(&self) -> String {
        let credit_account_branch = &self.credit_account_branch;
        credit_account_branch.to_string()
    }
    pub fn get_credit_account_type(&self) -> String {
        let credit_account_type = &self.credit_account_type;
        credit_account_type.to_string()
    }

    pub fn get_amount(&self) -> f32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_ccy(&self) -> String {
        let _ccy = &self._ccy;
        _ccy.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DomesticPaymentRequestDetails {
    request_id: String,
    param_list_domestic: Vec<DomesticParameterDetails>,
    _amount: f32,
    _currency: String,
    _status: String,
    rate_type: String,
}

impl DomesticPaymentRequestDetails {
    pub fn new(
        request_id: String,
        param_list_domestic: Vec<DomesticParameterDetails>,
        _amount: f32,
        _currency: String,
        _status: String, // optional
        rate_type: String,
    ) -> Result<Self, String> {
        if request_id.is_empty() || request_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("request id is empty"));
        }
        // request_id has a length of 15 characters
        else if request_id.len() == 15 {
            // request_id is valid
        } else {
            return Err(String::from("request id has invalid length"));
        }

        if param_list_domestic.is_empty() {
            return Err(String::from("param list domestic is empty"));
        }
        // param_list_domestic has a min length of 1 item
        else if param_list_domestic.len() > 0 {
            // param_list_domestic is valid
        } else {
            return Err(String::from("param list domestic has invalid length"));
        }

        if _amount > 0.0 {
            // _amount is valid
        } else {
            return Err(String::from("amount has invalid value"));
        }

        if _currency.is_empty() || _currency.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency is empty"));
        }
        // _currency has a length of 3 characters
        else if _currency.len() == 3 {
            // _currency is valid
        } else {
            return Err(String::from("currency has invalid length"));
        }

        if _status.is_empty() || _status.replace(" ", "").trim().len() == 0 {
            return Err(String::from("status is empty"));
        }
        // _status has a value of NEW
        else if _status.eq_ignore_ascii_case(&String::from("new")) {
            // _status is valid
        } else {
            return Err(String::from("status has invalid value"));
        }

        if rate_type.is_empty() || rate_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("rate type is empty"));
        }
        // rate_type has a value of Spot
        else if rate_type.eq_ignore_ascii_case(&String::from("spot ")) {
            // rate_type is valid
        } else {
            return Err(String::from("rate type has invalid value"));
        }

        Ok(Self {
            request_id,
            param_list_domestic,
            _amount,
            _currency,
            _status,
            rate_type,
        })
    }

    pub fn get_request_id(&self) -> String {
        let request_id = &self.request_id;
        request_id.to_string()
    }

    pub fn get_param_list_domestic(&self) -> &Vec<DomesticParameterDetails> {
        let param_list_domestic = &self.param_list_domestic;
        param_list_domestic
    }

    pub fn get_amount(&self) -> f32 {
        let _amount = &self._amount;
        *_amount
    }
    pub fn get_currency(&self) -> String {
        let _currency = &self._currency;
        _currency.to_string()
    }
    pub fn get_status(&self) -> String {
        let _status = &self._status;
        _status.to_string()
    }

    pub fn get_rate_type(&self) -> String {
        let rate_type = &self.rate_type;
        rate_type.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct TokenParameterDetails {
    transaction_description: String,
    secret_code: String,
    source_account: String,
    source_account_currency: String,
    source_account_type: String,
    sender_name: String,
    _ccy: String,
    sender_mobile_no: String,
    _amount: f32,
    sender_id: String,
    beneficiary_name: String,
    beneficiary_mobile_no: String,
    withdrawal_channel: String,
}

impl TokenParameterDetails {
    pub fn new(
        transaction_description: String,
        secret_code: String,
        source_account: String,
        source_account_currency: String,
        source_account_type: String,
        sender_name: String,
        _ccy: String,
        sender_mobile_no: String,
        _amount: f32,
        sender_id: String,
        beneficiary_name: String,
        beneficiary_mobile_no: String,
        withdrawal_channel: String,
    ) -> Result<Self, String> {
        if _ccy.is_empty() || _ccy.replace(" ", "").trim().len() == 0 {
            return Err(String::from("ccy is empty"));
        }
        // _ccy has a length of 3 characters
        else if _ccy.len() == 3 {
            // _ccy is valid
        } else {
            return Err(String::from("ccy has invalid length"));
        }

        if _amount > 0.0 {
            // _amount is valid
        } else {
            return Err(String::from("amount has invalid value"));
        }

        Ok(Self {
            transaction_description,
            secret_code,
            source_account,
            source_account_currency,
            source_account_type,
            sender_name,
            _ccy,
            sender_mobile_no,
            _amount,
            sender_id,
            beneficiary_name,
            beneficiary_mobile_no,
            withdrawal_channel,
        })
    }

    pub fn get_transaction_description(&self) -> String {
        let transaction_description = &self.transaction_description;
        transaction_description.to_string()
    }

    pub fn get_secret_code(&self) -> String {
        let secret_code = &self.secret_code;
        secret_code.to_string()
    }

    pub fn get_source_account(&self) -> String {
        let source_account = &self.source_account;
        source_account.to_string()
    }
    pub fn get_source_account_currency(&self) -> String {
        let source_account_currency = &self.source_account_currency;
        source_account_currency.to_string()
    }
    pub fn get_source_account_type(&self) -> String {
        let source_account_type = &self.source_account_type;
        source_account_type.to_string()
    }

    pub fn get_sender_name(&self) -> String {
        let sender_name = &self.sender_name;
        sender_name.to_string()
    }

    pub fn get_ccy(&self) -> String {
        let _ccy = &self._ccy;
        _ccy.to_string()
    }

    pub fn get_sender_mobile_no(&self) -> String {
        let sender_mobile_no = &self.sender_mobile_no;
        sender_mobile_no.to_string()
    }

    pub fn get_amount(&self) -> f32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_sender_id(&self) -> String {
        let sender_id = &self.sender_id;
        sender_id.to_string()
    }

    pub fn get_beneficiary_name(&self) -> String {
        let beneficiary_name = &self.beneficiary_name;
        beneficiary_name.to_string()
    }

    pub fn get_beneficiary_mobile_no(&self) -> String {
        let beneficiary_mobile_no = &self.beneficiary_mobile_no;
        beneficiary_mobile_no.to_string()
    }

    pub fn get_withdrawal_channel(&self) -> String {
        let withdrawal_channel = &self.withdrawal_channel;
        withdrawal_channel.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct TokenPaymentRequestDetails {
    request_id: String,
    param_list_token: Vec<TokenParameterDetails>,
    _amount: f32,
    _currency: String,
    _status: String,
    rate_type: String,
}

impl TokenPaymentRequestDetails {
    pub fn new(
        request_id: String,
        param_list_token: Vec<TokenParameterDetails>,
        _amount: f32,
        _currency: String,
        _status: String, // optional
        rate_type: String,
    ) -> Result<Self, String> {
        if request_id.is_empty() || request_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("request id is empty"));
        }
        // request_id has a length of 15 characters
        else if request_id.len() == 15 {
            // request_id is valid
        } else {
            return Err(String::from("request id has invalid length"));
        }

        if param_list_token.is_empty() {
            return Err(String::from("param list token is empty"));
        }
        // param_list_token has a min length of 1 item
        else if param_list_token.len() > 0 {
            // param_list_token is valid
        } else {
            return Err(String::from("param list token has invalid length"));
        }

        if _amount > 0.0 {
            // _amount is valid
        } else {
            return Err(String::from("amount has invalid value"));
        }

        if _currency.is_empty() || _currency.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency is empty"));
        }
        // _currency has a length of 3 characters
        else if _currency.len() == 3 {
            // _currency is valid
        } else {
            return Err(String::from("currency has invalid length"));
        }

        if _status.is_empty() || _status.replace(" ", "").trim().len() == 0 {
            return Err(String::from("status is empty"));
        }
        // _status has a value of NEW
        else if _status.eq_ignore_ascii_case(&String::from("new")) {
            // _status is valid
        } else {
            return Err(String::from("status has invalid value"));
        }

        if rate_type.is_empty() || rate_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("rate type is empty"));
        }
        // rate_type has a value of Spot
        else if rate_type.eq_ignore_ascii_case(&String::from("spot ")) {
            // rate_type is valid
        } else {
            return Err(String::from("rate type has invalid value"));
        }

        Ok(Self {
            request_id,
            param_list_token,
            _amount,
            _currency,
            _status,
            rate_type,
        })
    }

    pub fn get_request_id(&self) -> String {
        let request_id = &self.request_id;
        request_id.to_string()
    }

    pub fn get_param_list_token(&self) -> &Vec<TokenParameterDetails> {
        let param_list_token = &self.param_list_token;
        param_list_token
    }

    pub fn get_amount(&self) -> f32 {
        let _amount = &self._amount;
        *_amount
    }
    pub fn get_currency(&self) -> String {
        let _currency = &self._currency;
        _currency.to_string()
    }
    pub fn get_status(&self) -> String {
        let _status = &self._status;
        _status.to_string()
    }

    pub fn get_rate_type(&self) -> String {
        let rate_type = &self.rate_type;
        rate_type.to_string()
    }
}

#[derive(Debug)]
pub struct PaymentDataInputDetails {
    payment_header: PaymentHeaderDetails,
    extension_domestic: Vec<DomesticPaymentRequestDetails>,
    extension_token: Vec<TokenPaymentRequestDetails>,
    secure_hash: String,
}

impl PaymentDataInputDetails {
    pub fn new(
        payment_header: PaymentHeaderDetails,
        extension_domestic: Vec<DomesticPaymentRequestDetails>,
        extension_token: Vec<TokenPaymentRequestDetails>,
        secure_hash: String,
    ) -> Result<Self, String> {
        if extension_domestic.is_empty() {
            return Err(String::from("extension is empty"));
        }
        // _extension has a min length of 1 item
        else if extension_domestic.len() > 0 {
            // _extension is valid
        } else {
            return Err(String::from("extension has invalid length"));
        }

        if extension_token.is_empty() {
            return Err(String::from("extension token is empty"));
        }
        // extension_token has a min length of 1 item
        else if extension_token.len() > 0 {
            // extension_token is valid
        } else {
            return Err(String::from("extension token has invalid length"));
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
            payment_header,
            extension_domestic,
            extension_token,
            secure_hash,
        })
    }

    pub fn get_payment_header(&self) -> &PaymentHeaderDetails {
        let payment_header = &self.payment_header;
        payment_header
    }

    pub fn get_extension_domestic(&self) -> &Vec<DomesticPaymentRequestDetails> {
        let extension_domestic = &self.extension_domestic;
        extension_domestic
    }

    pub fn get_extension_token(&self) -> &Vec<TokenPaymentRequestDetails> {
        let extension_token = &self.extension_token;
        extension_token
    }

    pub fn get_secure_hash(&self) -> String {
        let secure_hash = &self.secure_hash;
        secure_hash.to_string()
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PaymentHeaderData {
    pub clientid: String,
    pub batchsequence: String,
    pub batchamount: f32,
    pub transactionamount: f32,
    pub batchid: String,
    pub transactioncount: u16,
    pub batchcount: u16,
    pub transactionid: String,
    pub debittype: String,
    pub affiliateCode: String,
    pub totalbatches: u16,
    pub execution_date: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum MixedTypeValue {
    StringValue(String),
    IntegerValue(u32),
    FloatValue(f32),
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct RequestParameter {
    pub Key: String,
    pub Value: MixedTypeValue,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PaymentRequestData {
    pub request_id: String,
    pub request_type: String,
    pub param_list: Vec<RequestParameter>,
    pub amount: f32,
    pub currency: String,
    pub status: String,
    pub rate_type: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PaymentData {
    pub paymentHeader: PaymentHeaderData,
    pub extension: Vec<PaymentRequestData>,
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
pub struct PaymentResponseData {
    pub response_code: Option<i16>,
    pub response_message: Option<String>,
    pub response_content: Vec<ResponseContentData>,
    pub response_timestamp: Option<String>,
}
