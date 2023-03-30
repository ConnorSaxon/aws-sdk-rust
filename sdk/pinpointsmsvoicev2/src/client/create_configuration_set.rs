// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateConfigurationSet`](crate::client::fluent_builders::CreateConfigurationSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::CreateConfigurationSet::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::CreateConfigurationSet::set_configuration_set_name): <p>The name to use for the new configuration set.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateConfigurationSet::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateConfigurationSet::set_tags): <p>An array of key and value pair tags that's associated with the new configuration set. </p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateConfigurationSet::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateConfigurationSet::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p>
                            /// - On success, responds with [`CreateConfigurationSetOutput`](crate::output::CreateConfigurationSetOutput) with field(s):
    ///   - [`configuration_set_arn(Option<String>)`](crate::output::CreateConfigurationSetOutput::configuration_set_arn): <p>The Amazon Resource Name (ARN) of the newly created configuration set.</p>
    ///   - [`configuration_set_name(Option<String>)`](crate::output::CreateConfigurationSetOutput::configuration_set_name): <p>The name of the new configuration set.</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::CreateConfigurationSetOutput::tags): <p>An array of key and value pair tags that's associated with the configuration set.</p>
    ///   - [`created_timestamp(Option<DateTime>)`](crate::output::CreateConfigurationSetOutput::created_timestamp): <p>The time when the configuration set was created, in <a href="https://www.epochconverter.com/">UNIX epoch time</a> format.</p>
                            /// - On failure, responds with [`SdkError<CreateConfigurationSetError>`](crate::error::CreateConfigurationSetError)
    pub fn create_configuration_set(&self) -> crate::client::fluent_builders::CreateConfigurationSet {
                                crate::client::fluent_builders::CreateConfigurationSet::new(self.handle.clone())
                            }
}

