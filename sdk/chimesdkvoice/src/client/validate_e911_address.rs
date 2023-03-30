// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ValidateE911Address`](crate::client::fluent_builders::ValidateE911Address) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::ValidateE911Address::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::ValidateE911Address::set_aws_account_id): (undocumented)
    ///   - [`street_number(impl Into<String>)`](crate::client::fluent_builders::ValidateE911Address::street_number) / [`set_street_number(Option<String>)`](crate::client::fluent_builders::ValidateE911Address::set_street_number): (undocumented)
    ///   - [`street_info(impl Into<String>)`](crate::client::fluent_builders::ValidateE911Address::street_info) / [`set_street_info(Option<String>)`](crate::client::fluent_builders::ValidateE911Address::set_street_info): (undocumented)
    ///   - [`city(impl Into<String>)`](crate::client::fluent_builders::ValidateE911Address::city) / [`set_city(Option<String>)`](crate::client::fluent_builders::ValidateE911Address::set_city): (undocumented)
    ///   - [`state(impl Into<String>)`](crate::client::fluent_builders::ValidateE911Address::state) / [`set_state(Option<String>)`](crate::client::fluent_builders::ValidateE911Address::set_state): (undocumented)
    ///   - [`country(impl Into<String>)`](crate::client::fluent_builders::ValidateE911Address::country) / [`set_country(Option<String>)`](crate::client::fluent_builders::ValidateE911Address::set_country): (undocumented)
    ///   - [`postal_code(impl Into<String>)`](crate::client::fluent_builders::ValidateE911Address::postal_code) / [`set_postal_code(Option<String>)`](crate::client::fluent_builders::ValidateE911Address::set_postal_code): (undocumented)
                            /// - On success, responds with [`ValidateE911AddressOutput`](crate::output::ValidateE911AddressOutput) with field(s):
    ///   - [`validation_result(i32)`](crate::output::ValidateE911AddressOutput::validation_result): (undocumented)
    ///   - [`address_external_id(Option<String>)`](crate::output::ValidateE911AddressOutput::address_external_id): (undocumented)
    ///   - [`address(Option<Address>)`](crate::output::ValidateE911AddressOutput::address): (undocumented)
    ///   - [`candidate_address_list(Option<Vec<CandidateAddress>>)`](crate::output::ValidateE911AddressOutput::candidate_address_list): (undocumented)
                            /// - On failure, responds with [`SdkError<ValidateE911AddressError>`](crate::error::ValidateE911AddressError)
    pub fn validate_e911_address(&self) -> crate::client::fluent_builders::ValidateE911Address {
                                crate::client::fluent_builders::ValidateE911Address::new(self.handle.clone())
                            }
}

