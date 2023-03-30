// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetClusterSessionCredentials`](crate::client::fluent_builders::GetClusterSessionCredentials) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl Into<String>)`](crate::client::fluent_builders::GetClusterSessionCredentials::cluster_id) / [`set_cluster_id(Option<String>)`](crate::client::fluent_builders::GetClusterSessionCredentials::set_cluster_id): <p>The unique identifier of the cluster.</p>
    ///   - [`execution_role_arn(impl Into<String>)`](crate::client::fluent_builders::GetClusterSessionCredentials::execution_role_arn) / [`set_execution_role_arn(Option<String>)`](crate::client::fluent_builders::GetClusterSessionCredentials::set_execution_role_arn): <p>The Amazon Resource Name (ARN) of the runtime role for interactive workload submission on the cluster. The runtime role can be a cross-account IAM role. The runtime role ARN is a combination of account ID, role name, and role type using the following format: <code>arn:partition:service:region:account:resource</code>.</p>
                            /// - On success, responds with [`GetClusterSessionCredentialsOutput`](crate::output::GetClusterSessionCredentialsOutput) with field(s):
    ///   - [`credentials(Option<Credentials>)`](crate::output::GetClusterSessionCredentialsOutput::credentials): <p>The credentials that you can use to connect to cluster endpoints that support username-based and password-based authentication.</p>
    ///   - [`expires_at(Option<DateTime>)`](crate::output::GetClusterSessionCredentialsOutput::expires_at): <p>The time when the credentials that are returned by the <code>GetClusterSessionCredentials</code> API expire.</p>
                            /// - On failure, responds with [`SdkError<GetClusterSessionCredentialsError>`](crate::error::GetClusterSessionCredentialsError)
    pub fn get_cluster_session_credentials(&self) -> crate::client::fluent_builders::GetClusterSessionCredentials {
                                crate::client::fluent_builders::GetClusterSessionCredentials::new(self.handle.clone())
                            }
}

