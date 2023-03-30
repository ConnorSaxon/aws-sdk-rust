// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeOrganizationConfiguration`](crate::client::fluent_builders::DescribeOrganizationConfiguration) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DescribeOrganizationConfiguration::send) it.
                            /// - On success, responds with [`DescribeOrganizationConfigurationOutput`](crate::output::DescribeOrganizationConfigurationOutput) with field(s):
    ///   - [`auto_enable(bool)`](crate::output::DescribeOrganizationConfigurationOutput::auto_enable): <p>Specifies whether Amazon Macie is enabled automatically for accounts that are added to the organization.</p>
    ///   - [`max_account_limit_reached(bool)`](crate::output::DescribeOrganizationConfigurationOutput::max_account_limit_reached): <p>Specifies whether the maximum number of Amazon Macie member accounts are part of the organization.</p>
                            /// - On failure, responds with [`SdkError<DescribeOrganizationConfigurationError>`](crate::error::DescribeOrganizationConfigurationError)
    pub fn describe_organization_configuration(&self) -> crate::client::fluent_builders::DescribeOrganizationConfiguration {
                                crate::client::fluent_builders::DescribeOrganizationConfiguration::new(self.handle.clone())
                            }
}

