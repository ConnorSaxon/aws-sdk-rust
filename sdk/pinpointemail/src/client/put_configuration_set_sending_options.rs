// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutConfigurationSetSendingOptions`](crate::client::fluent_builders::PutConfigurationSetSendingOptions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::PutConfigurationSetSendingOptions::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::PutConfigurationSetSendingOptions::set_configuration_set_name): <p>The name of the configuration set that you want to enable or disable email sending for.</p>
    ///   - [`sending_enabled(bool)`](crate::client::fluent_builders::PutConfigurationSetSendingOptions::sending_enabled) / [`set_sending_enabled(bool)`](crate::client::fluent_builders::PutConfigurationSetSendingOptions::set_sending_enabled): <p>If <code>true</code>, email sending is enabled for the configuration set. If <code>false</code>, email sending is disabled for the configuration set.</p>
                            /// - On success, responds with [`PutConfigurationSetSendingOptionsOutput`](crate::output::PutConfigurationSetSendingOptionsOutput)
                            /// - On failure, responds with [`SdkError<PutConfigurationSetSendingOptionsError>`](crate::error::PutConfigurationSetSendingOptionsError)
    pub fn put_configuration_set_sending_options(&self) -> crate::client::fluent_builders::PutConfigurationSetSendingOptions {
                                crate::client::fluent_builders::PutConfigurationSetSendingOptions::new(self.handle.clone())
                            }
}

