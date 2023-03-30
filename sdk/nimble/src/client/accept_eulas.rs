// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AcceptEulas`](crate::client::fluent_builders::AcceptEulas) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::AcceptEulas::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::AcceptEulas::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    ///   - [`eula_ids(Vec<String>)`](crate::client::fluent_builders::AcceptEulas::eula_ids) / [`set_eula_ids(Option<Vec<String>>)`](crate::client::fluent_builders::AcceptEulas::set_eula_ids): <p>The EULA ID.</p>
    ///   - [`studio_id(impl Into<String>)`](crate::client::fluent_builders::AcceptEulas::studio_id) / [`set_studio_id(Option<String>)`](crate::client::fluent_builders::AcceptEulas::set_studio_id): <p>The studio ID.</p>
                            /// - On success, responds with [`AcceptEulasOutput`](crate::output::AcceptEulasOutput) with field(s):
    ///   - [`eula_acceptances(Option<Vec<EulaAcceptance>>)`](crate::output::AcceptEulasOutput::eula_acceptances): <p>A collection of EULA acceptances.</p>
                            /// - On failure, responds with [`SdkError<AcceptEulasError>`](crate::error::AcceptEulasError)
    pub fn accept_eulas(&self) -> crate::client::fluent_builders::AcceptEulas {
                                crate::client::fluent_builders::AcceptEulas::new(self.handle.clone())
                            }
}

