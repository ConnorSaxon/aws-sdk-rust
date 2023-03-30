// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAccountAssignment`](crate::client::fluent_builders::CreateAccountAssignment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_arn(impl Into<String>)`](crate::client::fluent_builders::CreateAccountAssignment::instance_arn) / [`set_instance_arn(Option<String>)`](crate::client::fluent_builders::CreateAccountAssignment::set_instance_arn): <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    ///   - [`target_id(impl Into<String>)`](crate::client::fluent_builders::CreateAccountAssignment::target_id) / [`set_target_id(Option<String>)`](crate::client::fluent_builders::CreateAccountAssignment::set_target_id): <p>TargetID is an AWS account identifier, typically a 10-12 digit string (For example, 123456789012).</p>
    ///   - [`target_type(TargetType)`](crate::client::fluent_builders::CreateAccountAssignment::target_type) / [`set_target_type(Option<TargetType>)`](crate::client::fluent_builders::CreateAccountAssignment::set_target_type): <p>The entity type for which the assignment will be created.</p>
    ///   - [`permission_set_arn(impl Into<String>)`](crate::client::fluent_builders::CreateAccountAssignment::permission_set_arn) / [`set_permission_set_arn(Option<String>)`](crate::client::fluent_builders::CreateAccountAssignment::set_permission_set_arn): <p>The ARN of the permission set that the admin wants to grant the principal access to.</p>
    ///   - [`principal_type(PrincipalType)`](crate::client::fluent_builders::CreateAccountAssignment::principal_type) / [`set_principal_type(Option<PrincipalType>)`](crate::client::fluent_builders::CreateAccountAssignment::set_principal_type): <p>The entity type for which the assignment will be created.</p>
    ///   - [`principal_id(impl Into<String>)`](crate::client::fluent_builders::CreateAccountAssignment::principal_id) / [`set_principal_id(Option<String>)`](crate::client::fluent_builders::CreateAccountAssignment::set_principal_id): <p>An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the <a href="/singlesignon/latest/IdentityStoreAPIReference/welcome.html">IAM Identity Center Identity Store API Reference</a>.</p>
                            /// - On success, responds with [`CreateAccountAssignmentOutput`](crate::output::CreateAccountAssignmentOutput) with field(s):
    ///   - [`account_assignment_creation_status(Option<AccountAssignmentOperationStatus>)`](crate::output::CreateAccountAssignmentOutput::account_assignment_creation_status): <p>The status object for the account assignment creation operation.</p>
                            /// - On failure, responds with [`SdkError<CreateAccountAssignmentError>`](crate::error::CreateAccountAssignmentError)
    pub fn create_account_assignment(&self) -> crate::client::fluent_builders::CreateAccountAssignment {
                                crate::client::fluent_builders::CreateAccountAssignment::new(self.handle.clone())
                            }
}

