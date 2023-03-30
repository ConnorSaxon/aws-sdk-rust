// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutInlinePolicyToPermissionSet`](crate::client::fluent_builders::PutInlinePolicyToPermissionSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_arn(impl Into<String>)`](crate::client::fluent_builders::PutInlinePolicyToPermissionSet::instance_arn) / [`set_instance_arn(Option<String>)`](crate::client::fluent_builders::PutInlinePolicyToPermissionSet::set_instance_arn): <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    ///   - [`permission_set_arn(impl Into<String>)`](crate::client::fluent_builders::PutInlinePolicyToPermissionSet::permission_set_arn) / [`set_permission_set_arn(Option<String>)`](crate::client::fluent_builders::PutInlinePolicyToPermissionSet::set_permission_set_arn): <p>The ARN of the permission set.</p>
    ///   - [`inline_policy(impl Into<String>)`](crate::client::fluent_builders::PutInlinePolicyToPermissionSet::inline_policy) / [`set_inline_policy(Option<String>)`](crate::client::fluent_builders::PutInlinePolicyToPermissionSet::set_inline_policy): <p>The inline policy to attach to a <code>PermissionSet</code>.</p>
                            /// - On success, responds with [`PutInlinePolicyToPermissionSetOutput`](crate::output::PutInlinePolicyToPermissionSetOutput)
                            /// - On failure, responds with [`SdkError<PutInlinePolicyToPermissionSetError>`](crate::error::PutInlinePolicyToPermissionSetError)
    pub fn put_inline_policy_to_permission_set(&self) -> crate::client::fluent_builders::PutInlinePolicyToPermissionSet {
                                crate::client::fluent_builders::PutInlinePolicyToPermissionSet::new(self.handle.clone())
                            }
}

