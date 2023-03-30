// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeregisterAccount`](crate::client::fluent_builders::DeregisterAccount) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DeregisterAccount::send) it.
                            /// - On success, responds with [`DeregisterAccountOutput`](crate::output::DeregisterAccountOutput) with field(s):
    ///   - [`status(Option<AccountStatus>)`](crate::output::DeregisterAccountOutput::status): <p> The registration status of the account. </p>
                            /// - On failure, responds with [`SdkError<DeregisterAccountError>`](crate::error::DeregisterAccountError)
    pub fn deregister_account(&self) -> crate::client::fluent_builders::DeregisterAccount {
                                crate::client::fluent_builders::DeregisterAccount::new(self.handle.clone())
                            }
}

