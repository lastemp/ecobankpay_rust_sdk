# ecobankpay_rust_sdk

This is an sdk that will be used by developers to seamlessly integrate with EcobankPay APIs' Gateway.
Ecobank is a pan-African banking conglomerate, with banking operations in 33 African countries. It is the leading independent regional banking group in West Africa and Central Africa.
The API endpoints provided by EcobankPay Gateway includes; QR Code Payments, Card Payments, Payments and Account Services (https://developer.ecobank.com/app/index.xhtml). 

The sdk has below listed dependencies:
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client
- [serde_json](https://github.com/serde-rs/json) for serializing and deserializing Rust data structures
- [serde_urlencoded](https://github.com/nox/serde_urlencoded) for serialising to and deserialising from the application/x-www-form-urlencoded format
- [chrono](https://github.com/chronotope/chrono) provides all functionality needed to do correct operations on dates and times
- [base64](https://github.com/marshallpierce/rust-base64) Decode from Base64 format or encode into it
- [tokio](https://github.com/tokio-rs/tokio) A runtime for writing reliable, asynchronous applications

## installation

```
cargo install --git https://github.com/lastemp/ecobankpay_rust_sdk
```

## Usage

Please find below code samples and full working examples:

   - See [the code samples](./code_samples/) for more info.	
   - See [the examples](./examples/) for full working examples.
