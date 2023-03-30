// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePhoneNumber`](crate::client::fluent_builders::DeletePhoneNumber) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`phone_number_id(impl Into<String>)`](crate::client::fluent_builders::DeletePhoneNumber::phone_number_id) / [`set_phone_number_id(Option<String>)`](crate::client::fluent_builders::DeletePhoneNumber::set_phone_number_id): <p>The phone number ID.</p>
                            /// - On success, responds with [`DeletePhoneNumberOutput`](crate::output::DeletePhoneNumberOutput)
                            /// - On failure, responds with [`SdkError<DeletePhoneNumberError>`](crate::error::DeletePhoneNumberError)
    pub fn delete_phone_number(&self) -> crate::client::fluent_builders::DeletePhoneNumber {
                                crate::client::fluent_builders::DeletePhoneNumber::new(self.handle.clone())
                            }
}

