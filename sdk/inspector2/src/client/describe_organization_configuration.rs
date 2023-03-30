// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeOrganizationConfiguration`](crate::client::fluent_builders::DescribeOrganizationConfiguration) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DescribeOrganizationConfiguration::send) it.
                            /// - On success, responds with [`DescribeOrganizationConfigurationOutput`](crate::output::DescribeOrganizationConfigurationOutput) with field(s):
    ///   - [`auto_enable(Option<AutoEnable>)`](crate::output::DescribeOrganizationConfigurationOutput::auto_enable): <p>The scan types are automatically enabled for new members of your organization.</p>
    ///   - [`max_account_limit_reached(Option<bool>)`](crate::output::DescribeOrganizationConfigurationOutput::max_account_limit_reached): <p>Represents whether your organization has reached the maximum Amazon Web Services account limit for Amazon Inspector.</p>
                            /// - On failure, responds with [`SdkError<DescribeOrganizationConfigurationError>`](crate::error::DescribeOrganizationConfigurationError)
    pub fn describe_organization_configuration(&self) -> crate::client::fluent_builders::DescribeOrganizationConfiguration {
                                crate::client::fluent_builders::DescribeOrganizationConfiguration::new(self.handle.clone())
                            }
}

