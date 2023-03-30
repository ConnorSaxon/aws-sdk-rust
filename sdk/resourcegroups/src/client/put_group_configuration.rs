// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutGroupConfiguration`](crate::client::fluent_builders::PutGroupConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`group(impl Into<String>)`](crate::client::fluent_builders::PutGroupConfiguration::group) / [`set_group(Option<String>)`](crate::client::fluent_builders::PutGroupConfiguration::set_group): <p>The name or ARN of the resource group with the configuration that you want to update.</p>
    ///   - [`configuration(Vec<GroupConfigurationItem>)`](crate::client::fluent_builders::PutGroupConfiguration::configuration) / [`set_configuration(Option<Vec<GroupConfigurationItem>>)`](crate::client::fluent_builders::PutGroupConfiguration::set_configuration): <p>The new configuration to associate with the specified group. A configuration associates the resource group with an Amazon Web Services service and specifies how the service can interact with the resources in the group. A configuration is an array of <code>GroupConfigurationItem</code> elements.</p>  <p>For information about the syntax of a service configuration, see <a href="https://docs.aws.amazon.com/ARG/latest/APIReference/about-slg.html">Service configurations for Resource Groups</a>.</p> <note>   <p>A resource group can contain either a <code>Configuration</code> or a <code>ResourceQuery</code>, but not both.</p>  </note>
                            /// - On success, responds with [`PutGroupConfigurationOutput`](crate::output::PutGroupConfigurationOutput)
                            /// - On failure, responds with [`SdkError<PutGroupConfigurationError>`](crate::error::PutGroupConfigurationError)
    pub fn put_group_configuration(&self) -> crate::client::fluent_builders::PutGroupConfiguration {
                                crate::client::fluent_builders::PutGroupConfiguration::new(self.handle.clone())
                            }
}

