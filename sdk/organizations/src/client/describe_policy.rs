// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribePolicy`](crate::client::fluent_builders::DescribePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`policy_id(impl Into<String>)`](crate::client::fluent_builders::DescribePolicy::policy_id) / [`set_policy_id(Option<String>)`](crate::client::fluent_builders::DescribePolicy::set_policy_id): <p>The unique identifier (ID) of the policy that you want details about. You can get the ID from the <code>ListPolicies</code> or <code>ListPoliciesForTarget</code> operations.</p>  <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lowercase or uppercase letters, digits, or the underscore character (_).</p>
                            /// - On success, responds with [`DescribePolicyOutput`](crate::output::DescribePolicyOutput) with field(s):
    ///   - [`policy(Option<Policy>)`](crate::output::DescribePolicyOutput::policy): <p>A structure that contains details about the specified policy.</p>
                            /// - On failure, responds with [`SdkError<DescribePolicyError>`](crate::error::DescribePolicyError)
    pub fn describe_policy(&self) -> crate::client::fluent_builders::DescribePolicy {
                                crate::client::fluent_builders::DescribePolicy::new(self.handle.clone())
                            }
}

