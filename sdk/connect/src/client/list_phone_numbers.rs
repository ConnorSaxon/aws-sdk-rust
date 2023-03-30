// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPhoneNumbers`](crate::client::fluent_builders::ListPhoneNumbers) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPhoneNumbers::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::ListPhoneNumbers::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::ListPhoneNumbers::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`phone_number_types(Vec<PhoneNumberType>)`](crate::client::fluent_builders::ListPhoneNumbers::phone_number_types) / [`set_phone_number_types(Option<Vec<PhoneNumberType>>)`](crate::client::fluent_builders::ListPhoneNumbers::set_phone_number_types): <p>The type of phone number.</p>
    ///   - [`phone_number_country_codes(Vec<PhoneNumberCountryCode>)`](crate::client::fluent_builders::ListPhoneNumbers::phone_number_country_codes) / [`set_phone_number_country_codes(Option<Vec<PhoneNumberCountryCode>>)`](crate::client::fluent_builders::ListPhoneNumbers::set_phone_number_country_codes): <p>The ISO country code.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPhoneNumbers::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPhoneNumbers::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPhoneNumbers::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListPhoneNumbers::set_max_results): <p>The maximum number of results to return per page. The default MaxResult size is 100.</p>
                            /// - On success, responds with [`ListPhoneNumbersOutput`](crate::output::ListPhoneNumbersOutput) with field(s):
    ///   - [`phone_number_summary_list(Option<Vec<PhoneNumberSummary>>)`](crate::output::ListPhoneNumbersOutput::phone_number_summary_list): <p>Information about the phone numbers.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPhoneNumbersOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListPhoneNumbersError>`](crate::error::ListPhoneNumbersError)
    pub fn list_phone_numbers(&self) -> crate::client::fluent_builders::ListPhoneNumbers {
                                crate::client::fluent_builders::ListPhoneNumbers::new(self.handle.clone())
                            }
}

