// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`OptInPhoneNumber`](crate::client::fluent_builders::OptInPhoneNumber) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`phone_number(impl Into<String>)`](crate::client::fluent_builders::OptInPhoneNumber::phone_number) / [`set_phone_number(Option<String>)`](crate::client::fluent_builders::OptInPhoneNumber::set_phone_number): <p>The phone number to opt in. Use E.164 format.</p>
                            /// - On success, responds with [`OptInPhoneNumberOutput`](crate::output::OptInPhoneNumberOutput)
                            /// - On failure, responds with [`SdkError<OptInPhoneNumberError>`](crate::error::OptInPhoneNumberError)
    pub fn opt_in_phone_number(&self) -> crate::client::fluent_builders::OptInPhoneNumber {
                                crate::client::fluent_builders::OptInPhoneNumber::new(self.handle.clone())
                            }
}

