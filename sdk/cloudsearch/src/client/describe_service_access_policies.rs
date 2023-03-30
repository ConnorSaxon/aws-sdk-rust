// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeServiceAccessPolicies`](crate::client::fluent_builders::DescribeServiceAccessPolicies) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::DescribeServiceAccessPolicies::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::DescribeServiceAccessPolicies::set_domain_name): <p>The name of the domain you want to describe.</p>
    ///   - [`deployed(bool)`](crate::client::fluent_builders::DescribeServiceAccessPolicies::deployed) / [`set_deployed(Option<bool>)`](crate::client::fluent_builders::DescribeServiceAccessPolicies::set_deployed): <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
                            /// - On success, responds with [`DescribeServiceAccessPoliciesOutput`](crate::output::DescribeServiceAccessPoliciesOutput) with field(s):
    ///   - [`access_policies(Option<AccessPoliciesStatus>)`](crate::output::DescribeServiceAccessPoliciesOutput::access_policies): <p>The access rules configured for the domain specified in the request.</p>
                            /// - On failure, responds with [`SdkError<DescribeServiceAccessPoliciesError>`](crate::error::DescribeServiceAccessPoliciesError)
    pub fn describe_service_access_policies(&self) -> crate::client::fluent_builders::DescribeServiceAccessPolicies {
                                crate::client::fluent_builders::DescribeServiceAccessPolicies::new(self.handle.clone())
                            }
}

