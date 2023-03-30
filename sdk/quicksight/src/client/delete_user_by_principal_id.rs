// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteUserByPrincipalId`](crate::client::fluent_builders::DeleteUserByPrincipalId) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`principal_id(impl Into<String>)`](crate::client::fluent_builders::DeleteUserByPrincipalId::principal_id) / [`set_principal_id(Option<String>)`](crate::client::fluent_builders::DeleteUserByPrincipalId::set_principal_id): <p>The principal ID of the user.</p>
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::DeleteUserByPrincipalId::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::DeleteUserByPrincipalId::set_aws_account_id): <p>The ID for the Amazon Web Services account that the user is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    ///   - [`namespace(impl Into<String>)`](crate::client::fluent_builders::DeleteUserByPrincipalId::namespace) / [`set_namespace(Option<String>)`](crate::client::fluent_builders::DeleteUserByPrincipalId::set_namespace): <p>The namespace. Currently, you should set this to <code>default</code>.</p>
                            /// - On success, responds with [`DeleteUserByPrincipalIdOutput`](crate::output::DeleteUserByPrincipalIdOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::DeleteUserByPrincipalIdOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::output::DeleteUserByPrincipalIdOutput::status): <p>The HTTP status of the request.</p>
                            /// - On failure, responds with [`SdkError<DeleteUserByPrincipalIdError>`](crate::error::DeleteUserByPrincipalIdError)
    pub fn delete_user_by_principal_id(&self) -> crate::client::fluent_builders::DeleteUserByPrincipalId {
                                crate::client::fluent_builders::DeleteUserByPrincipalId::new(self.handle.clone())
                            }
}

