// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListSupportedPhoneNumberCountries`](crate::client::fluent_builders::ListSupportedPhoneNumberCountries) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`product_type(PhoneNumberProductType)`](crate::client::fluent_builders::ListSupportedPhoneNumberCountries::product_type) / [`set_product_type(Option<PhoneNumberProductType>)`](crate::client::fluent_builders::ListSupportedPhoneNumberCountries::set_product_type): <p>The phone number product type.</p>
                            /// - On success, responds with [`ListSupportedPhoneNumberCountriesOutput`](crate::output::ListSupportedPhoneNumberCountriesOutput) with field(s):
    ///   - [`phone_number_countries(Option<Vec<PhoneNumberCountry>>)`](crate::output::ListSupportedPhoneNumberCountriesOutput::phone_number_countries): <p>The supported phone number countries.</p>
                            /// - On failure, responds with [`SdkError<ListSupportedPhoneNumberCountriesError>`](crate::error::ListSupportedPhoneNumberCountriesError)
    pub fn list_supported_phone_number_countries(&self) -> crate::client::fluent_builders::ListSupportedPhoneNumberCountries {
                                crate::client::fluent_builders::ListSupportedPhoneNumberCountries::new(self.handle.clone())
                            }
}

