// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetKeyPolicy`](crate::client::fluent_builders::GetKeyPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::client::fluent_builders::GetKeyPolicy::key_id) / [`set_key_id(Option<String>)`](crate::client::fluent_builders::GetKeyPolicy::set_key_id): <p>Gets the key policy for the specified KMS key.</p>  <p>Specify the key ID or key ARN of the KMS key.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    ///   - [`policy_name(impl Into<String>)`](crate::client::fluent_builders::GetKeyPolicy::policy_name) / [`set_policy_name(Option<String>)`](crate::client::fluent_builders::GetKeyPolicy::set_policy_name): <p>Specifies the name of the key policy. The only valid name is <code>default</code>. To get the names of key policies, use <code>ListKeyPolicies</code>.</p>
                            /// - On success, responds with [`GetKeyPolicyOutput`](crate::output::GetKeyPolicyOutput) with field(s):
    ///   - [`policy(Option<String>)`](crate::output::GetKeyPolicyOutput::policy): <p>A key policy document in JSON format.</p>
                            /// - On failure, responds with [`SdkError<GetKeyPolicyError>`](crate::error::GetKeyPolicyError)
    pub fn get_key_policy(&self) -> crate::client::fluent_builders::GetKeyPolicy {
                                crate::client::fluent_builders::GetKeyPolicy::new(self.handle.clone())
                            }
}

