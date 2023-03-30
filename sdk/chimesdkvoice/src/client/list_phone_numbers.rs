// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPhoneNumbers`](crate::client::fluent_builders::ListPhoneNumbers) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPhoneNumbers::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`status(impl Into<String>)`](crate::client::fluent_builders::ListPhoneNumbers::status) / [`set_status(Option<String>)`](crate::client::fluent_builders::ListPhoneNumbers::set_status): (undocumented)
    ///   - [`product_type(PhoneNumberProductType)`](crate::client::fluent_builders::ListPhoneNumbers::product_type) / [`set_product_type(Option<PhoneNumberProductType>)`](crate::client::fluent_builders::ListPhoneNumbers::set_product_type): (undocumented)
    ///   - [`filter_name(PhoneNumberAssociationName)`](crate::client::fluent_builders::ListPhoneNumbers::filter_name) / [`set_filter_name(Option<PhoneNumberAssociationName>)`](crate::client::fluent_builders::ListPhoneNumbers::set_filter_name): (undocumented)
    ///   - [`filter_value(impl Into<String>)`](crate::client::fluent_builders::ListPhoneNumbers::filter_value) / [`set_filter_value(Option<String>)`](crate::client::fluent_builders::ListPhoneNumbers::set_filter_value): (undocumented)
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPhoneNumbers::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListPhoneNumbers::set_max_results): (undocumented)
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPhoneNumbers::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPhoneNumbers::set_next_token): (undocumented)
                            /// - On success, responds with [`ListPhoneNumbersOutput`](crate::output::ListPhoneNumbersOutput) with field(s):
    ///   - [`phone_numbers(Option<Vec<PhoneNumber>>)`](crate::output::ListPhoneNumbersOutput::phone_numbers): (undocumented)
    ///   - [`next_token(Option<String>)`](crate::output::ListPhoneNumbersOutput::next_token): (undocumented)
                            /// - On failure, responds with [`SdkError<ListPhoneNumbersError>`](crate::error::ListPhoneNumbersError)
    pub fn list_phone_numbers(&self) -> crate::client::fluent_builders::ListPhoneNumbers {
                                crate::client::fluent_builders::ListPhoneNumbers::new(self.handle.clone())
                            }
}

