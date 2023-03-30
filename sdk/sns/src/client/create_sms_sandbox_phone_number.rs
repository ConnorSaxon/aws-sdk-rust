// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateSMSSandboxPhoneNumber`](crate::client::fluent_builders::CreateSMSSandboxPhoneNumber) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`phone_number(impl Into<String>)`](crate::client::fluent_builders::CreateSMSSandboxPhoneNumber::phone_number) / [`set_phone_number(Option<String>)`](crate::client::fluent_builders::CreateSMSSandboxPhoneNumber::set_phone_number): <p>The destination phone number to verify. On verification, Amazon SNS adds this phone number to the list of verified phone numbers that you can send SMS messages to.</p>
    ///   - [`language_code(LanguageCodeString)`](crate::client::fluent_builders::CreateSMSSandboxPhoneNumber::language_code) / [`set_language_code(Option<LanguageCodeString>)`](crate::client::fluent_builders::CreateSMSSandboxPhoneNumber::set_language_code): <p>The language to use for sending the OTP. The default value is <code>en-US</code>.</p>
                            /// - On success, responds with [`CreateSmsSandboxPhoneNumberOutput`](crate::output::CreateSmsSandboxPhoneNumberOutput)
                            /// - On failure, responds with [`SdkError<CreateSMSSandboxPhoneNumberError>`](crate::error::CreateSMSSandboxPhoneNumberError)
    pub fn create_sms_sandbox_phone_number(&self) -> crate::client::fluent_builders::CreateSMSSandboxPhoneNumber {
                                crate::client::fluent_builders::CreateSMSSandboxPhoneNumber::new(self.handle.clone())
                            }
}

