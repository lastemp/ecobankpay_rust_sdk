use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::models::account_services::{
    account_balance::{AccountBalanceData, AccountBalanceDataInputDetails},
    account_inquiry::{AccountInquiryData, AccountInquiryDataInputDetails},
    account_statement::{AccountStatementData, AccountStatementDataInputDetails},
};

use crate::models::payments::payment::{
    DomesticPaymentRequestDetails, MixedTypeValue, PaymentData, PaymentDataInputDetails,
    PaymentHeaderData, PaymentRequestData, RequestParameter, TokenPaymentRequestDetails,
};

pub fn build_headers(access_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());

    headers
}

pub fn build_headers_generate_auth_token(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", api_key.parse().unwrap());

    headers
}

pub fn build_account_balance_data(
    account_details: &AccountBalanceDataInputDetails,
) -> AccountBalanceData {
    AccountBalanceData {
        requestId: account_details.get_request_id(),
        affiliateCode: account_details.get_affiliate_code(),
        accountNo: account_details.get_account_no(),
        clientId: account_details.get_client_id(),
        companyName: account_details.get_company_name(),
        secureHash: account_details.get_secure_hash(),
    }
}

pub fn build_account_inquiry_data(
    account_details: &AccountInquiryDataInputDetails,
) -> AccountInquiryData {
    AccountInquiryData {
        requestId: account_details.get_request_id(),
        affiliateCode: account_details.get_affiliate_code(),
        accountNo: account_details.get_account_no(),
        clientId: account_details.get_client_id(),
        companyName: account_details.get_company_name(),
        secureHash: account_details.get_secure_hash(),
    }
}

pub fn build_account_statement_data(
    account_details: &AccountStatementDataInputDetails,
) -> AccountStatementData {
    AccountStatementData {
        affiliateCode: account_details.get_affiliate_code(),
        corporateId: account_details.get_corporate_id(),
        accountNumber: account_details.get_account_number(),
        startDate: account_details.get_start_date(),
        endDate: account_details.get_end_date(),
        secureHash: account_details.get_secure_hash(),
    }
}

pub fn build_payment_data(payment_details: &PaymentDataInputDetails) -> PaymentData {
    let extension_domestic: &Vec<DomesticPaymentRequestDetails> =
        payment_details.get_extension_domestic();
    let extension_token: &Vec<TokenPaymentRequestDetails> = payment_details.get_extension_token();
    let x1 = if !extension_domestic.is_empty() {
        extension_domestic.len()
    } else {
        0
    };
    let x2 = if !extension_token.is_empty() {
        extension_token.len()
    } else {
        0
    };
    let transaction_count: u16 = x1 as u16 + x2 as u16;
    let batch_count = transaction_count;

    let payment_header = payment_details.get_payment_header();

    let my_payment_header = PaymentHeaderData {
        clientid: payment_header.get_client_id(),
        batchsequence: payment_header.get_batch_sequence().to_string(),
        batchamount: payment_header.get_batch_amount(),
        transactionamount: payment_header.get_transaction_amount(),
        batchid: payment_header.get_batch_id(),
        transactioncount: transaction_count,
        batchcount: batch_count,
        transactionid: payment_header.get_transaction_id(),
        debittype: payment_header.get_debit_type(),
        affiliateCode: payment_header.get_affiliate_code(),
        totalbatches: payment_header.get_total_batches(),
        execution_date: payment_header.get_execution_date(),
    };

    let mut my_extension: Vec<PaymentRequestData> = Vec::new();

    // "domestic"
    for param_request in extension_domestic {
        let mut my_param_list: Vec<RequestParameter> = Vec::new();

        let param_list_domestic = param_request.get_param_list_domestic();

        for param_list in param_list_domestic {
            let credit_account_no = RequestParameter {
                Key: String::from("creditAccountNo"),
                Value: MixedTypeValue::StringValue(param_list.get_credit_account_no()),
            };

            let debit_account_branch = RequestParameter {
                Key: String::from("debitAccountBranch"),
                Value: MixedTypeValue::StringValue(param_list.get_debit_account_branch()),
            };

            let debit_account_type = RequestParameter {
                Key: String::from("debitAccountType"),
                Value: MixedTypeValue::StringValue(param_list.get_debit_account_type()),
            };

            let credit_account_branch = RequestParameter {
                Key: String::from("creditAccountBranch"),
                Value: MixedTypeValue::StringValue(param_list.get_credit_account_branch()),
            };

            let credit_account_type = RequestParameter {
                Key: String::from("creditAccountType"),
                Value: MixedTypeValue::StringValue(param_list.get_credit_account_type()),
            };

            let _amount = RequestParameter {
                Key: String::from("amount"),
                Value: MixedTypeValue::FloatValue(param_list.get_amount()),
            };

            let _ccy = RequestParameter {
                Key: String::from("ccy"),
                Value: MixedTypeValue::StringValue(param_list.get_ccy()),
            };

            my_param_list.push(credit_account_no);
            my_param_list.push(debit_account_branch);
            my_param_list.push(debit_account_type);
            my_param_list.push(credit_account_branch);
            my_param_list.push(credit_account_type);
            my_param_list.push(_amount);
            my_param_list.push(_ccy);
        }

        let my_param_request = PaymentRequestData {
            request_id: param_request.get_request_id(),
            request_type: String::from("domestic"),
            param_list: my_param_list,
            amount: param_request.get_amount(),
            currency: param_request.get_currency(),
            status: param_request.get_status(),
            rate_type: param_request.get_rate_type(),
        };

        my_extension.push(my_param_request);
    }

    // "token"
    for param_request in extension_token {
        let mut my_param_list: Vec<RequestParameter> = Vec::new();

        let param_list_token = param_request.get_param_list_token();

        for param_list in param_list_token {
            let transaction_description = RequestParameter {
                Key: String::from("transactionDescription"),
                Value: MixedTypeValue::StringValue(param_list.get_transaction_description()),
            };

            let secret_code = RequestParameter {
                Key: String::from("secretCode"),
                Value: MixedTypeValue::StringValue(param_list.get_secret_code()),
            };

            let source_account = RequestParameter {
                Key: String::from("sourceAccount"),
                Value: MixedTypeValue::StringValue(param_list.get_source_account()),
            };

            let source_account_currency = RequestParameter {
                Key: String::from("sourceAccountCurrency"),
                Value: MixedTypeValue::StringValue(param_list.get_source_account_currency()),
            };

            let source_account_type = RequestParameter {
                Key: String::from("sourceAccountType"),
                Value: MixedTypeValue::StringValue(param_list.get_source_account_type()),
            };

            let sender_name = RequestParameter {
                Key: String::from("senderName"),
                Value: MixedTypeValue::StringValue(param_list.get_sender_name()),
            };

            let _ccy = RequestParameter {
                Key: String::from("ccy"),
                Value: MixedTypeValue::StringValue(param_list.get_ccy()),
            };

            let sender_mobile_no = RequestParameter {
                Key: String::from("senderMobileNo"),
                Value: MixedTypeValue::StringValue(param_list.get_sender_mobile_no()),
            };

            let _amount = RequestParameter {
                Key: String::from("amount"),
                Value: MixedTypeValue::FloatValue(param_list.get_amount()),
            };

            let sender_id = RequestParameter {
                Key: String::from("senderId"),
                Value: MixedTypeValue::StringValue(param_list.get_sender_id()),
            };

            let beneficiary_name = RequestParameter {
                Key: String::from("beneficiaryName"),
                Value: MixedTypeValue::StringValue(param_list.get_beneficiary_name()),
            };

            let beneficiary_mobile_no = RequestParameter {
                Key: String::from("beneficiaryMobileNo"),
                Value: MixedTypeValue::StringValue(param_list.get_beneficiary_mobile_no()),
            };

            let withdrawal_channel = RequestParameter {
                Key: String::from("withdrawalChannel"),
                Value: MixedTypeValue::StringValue(param_list.get_withdrawal_channel()),
            };

            my_param_list.push(transaction_description);
            my_param_list.push(secret_code);
            my_param_list.push(source_account);
            my_param_list.push(source_account_currency);
            my_param_list.push(source_account_type);
            my_param_list.push(sender_name);
            my_param_list.push(_ccy);
            my_param_list.push(sender_mobile_no);
            my_param_list.push(_amount);
            my_param_list.push(sender_id);
            my_param_list.push(beneficiary_name);
            my_param_list.push(beneficiary_mobile_no);
            my_param_list.push(withdrawal_channel);
        }

        let my_param_request = PaymentRequestData {
            request_id: param_request.get_request_id(),
            request_type: String::from("token"),
            param_list: my_param_list,
            amount: param_request.get_amount(),
            currency: param_request.get_currency(),
            status: param_request.get_status(),
            rate_type: param_request.get_rate_type(),
        };

        my_extension.push(my_param_request);
    }

    let secure_hash = payment_details.get_secure_hash(); //payment_details.payment_header.get_secure_hash();

    PaymentData {
        paymentHeader: my_payment_header,
        extension: my_extension,
        secureHash: secure_hash,
    }
}
