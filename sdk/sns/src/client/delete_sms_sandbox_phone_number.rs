// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteSMSSandboxPhoneNumber`](crate::client::fluent_builders::DeleteSMSSandboxPhoneNumber) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`phone_number(impl Into<String>)`](crate::client::fluent_builders::DeleteSMSSandboxPhoneNumber::phone_number) / [`set_phone_number(Option<String>)`](crate::client::fluent_builders::DeleteSMSSandboxPhoneNumber::set_phone_number): <p>The destination phone number to delete.</p>
                            /// - On success, responds with [`DeleteSmsSandboxPhoneNumberOutput`](crate::output::DeleteSmsSandboxPhoneNumberOutput)
                            /// - On failure, responds with [`SdkError<DeleteSMSSandboxPhoneNumberError>`](crate::error::DeleteSMSSandboxPhoneNumberError)
    pub fn delete_sms_sandbox_phone_number(&self) -> crate::client::fluent_builders::DeleteSMSSandboxPhoneNumber {
                                crate::client::fluent_builders::DeleteSMSSandboxPhoneNumber::new(self.handle.clone())
                            }
}

