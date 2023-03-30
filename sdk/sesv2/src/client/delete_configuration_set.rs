// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteConfigurationSet`](crate::client::fluent_builders::DeleteConfigurationSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::DeleteConfigurationSet::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::DeleteConfigurationSet::set_configuration_set_name): <p>The name of the configuration set.</p>
                            /// - On success, responds with [`DeleteConfigurationSetOutput`](crate::output::DeleteConfigurationSetOutput)
                            /// - On failure, responds with [`SdkError<DeleteConfigurationSetError>`](crate::error::DeleteConfigurationSetError)
    pub fn delete_configuration_set(&self) -> crate::client::fluent_builders::DeleteConfigurationSet {
                                crate::client::fluent_builders::DeleteConfigurationSet::new(self.handle.clone())
                            }
}

