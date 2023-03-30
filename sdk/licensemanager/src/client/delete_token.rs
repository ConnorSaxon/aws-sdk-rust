// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteToken`](crate::client::fluent_builders::DeleteToken) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`token_id(impl Into<String>)`](crate::client::fluent_builders::DeleteToken::token_id) / [`set_token_id(Option<String>)`](crate::client::fluent_builders::DeleteToken::set_token_id): <p>Token ID.</p>
                            /// - On success, responds with [`DeleteTokenOutput`](crate::output::DeleteTokenOutput)
                            /// - On failure, responds with [`SdkError<DeleteTokenError>`](crate::error::DeleteTokenError)
    pub fn delete_token(&self) -> crate::client::fluent_builders::DeleteToken {
                                crate::client::fluent_builders::DeleteToken::new(self.handle.clone())
                            }
}

