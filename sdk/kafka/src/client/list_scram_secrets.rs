// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListScramSecrets`](crate::client::fluent_builders::ListScramSecrets) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListScramSecrets::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_arn(impl Into<String>)`](crate::client::fluent_builders::ListScramSecrets::cluster_arn) / [`set_cluster_arn(Option<String>)`](crate::client::fluent_builders::ListScramSecrets::set_cluster_arn): <p>The arn of the cluster.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListScramSecrets::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListScramSecrets::set_max_results): <p>The maxResults of the query.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListScramSecrets::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListScramSecrets::set_next_token): <p>The nextToken of the query.</p>
                            /// - On success, responds with [`ListScramSecretsOutput`](crate::output::ListScramSecretsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListScramSecretsOutput::next_token): <p>Paginated results marker.</p>
    ///   - [`secret_arn_list(Option<Vec<String>>)`](crate::output::ListScramSecretsOutput::secret_arn_list): <p>The list of scram secrets associated with the cluster.</p>
                            /// - On failure, responds with [`SdkError<ListScramSecretsError>`](crate::error::ListScramSecretsError)
    pub fn list_scram_secrets(&self) -> crate::client::fluent_builders::ListScramSecrets {
                                crate::client::fluent_builders::ListScramSecrets::new(self.handle.clone())
                            }
}

