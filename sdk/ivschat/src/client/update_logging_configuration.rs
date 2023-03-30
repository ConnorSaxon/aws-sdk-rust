// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateLoggingConfiguration`](crate::client::fluent_builders::UpdateLoggingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::client::fluent_builders::UpdateLoggingConfiguration::identifier) / [`set_identifier(Option<String>)`](crate::client::fluent_builders::UpdateLoggingConfiguration::set_identifier): <p>Identifier of the logging configuration to be updated.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateLoggingConfiguration::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateLoggingConfiguration::set_name): <p>Logging-configuration name. The value does not need to be unique.</p>
    ///   - [`destination_configuration(DestinationConfiguration)`](crate::client::fluent_builders::UpdateLoggingConfiguration::destination_configuration) / [`set_destination_configuration(Option<DestinationConfiguration>)`](crate::client::fluent_builders::UpdateLoggingConfiguration::set_destination_configuration): <p>A complex type that contains a destination configuration for where chat content will be logged. There can be only one type of destination (<code>cloudWatchLogs</code>, <code>firehose</code>, or <code>s3</code>) in a <code>destinationConfiguration</code>.</p>
                            /// - On success, responds with [`UpdateLoggingConfigurationOutput`](crate::output::UpdateLoggingConfigurationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::UpdateLoggingConfigurationOutput::arn): <p>Logging-configuration ARN, from the request (if <code>identifier</code> was an ARN).</p>
    ///   - [`id(Option<String>)`](crate::output::UpdateLoggingConfigurationOutput::id): <p>Logging-configuration ID, generated by the system. This is a relative identifier, the part of the ARN that uniquely identifies the room.</p>
    ///   - [`create_time(Option<DateTime>)`](crate::output::UpdateLoggingConfigurationOutput::create_time): <p>Time when the logging configuration was created. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>.</p>
    ///   - [`update_time(Option<DateTime>)`](crate::output::UpdateLoggingConfigurationOutput::update_time): <p>Time of the logging configuration’s last update. This is an ISO 8601 timestamp; <i>note that this is returned as a string</i>.</p>
    ///   - [`name(Option<String>)`](crate::output::UpdateLoggingConfigurationOutput::name): <p>Logging-configuration name, from the request (if specified).</p>
    ///   - [`destination_configuration(Option<DestinationConfiguration>)`](crate::output::UpdateLoggingConfigurationOutput::destination_configuration): <p>A complex type that contains a destination configuration for where chat content will be logged, from the request. There is only one type of destination (<code>cloudWatchLogs</code>, <code>firehose</code>, or <code>s3</code>) in a <code>destinationConfiguration</code>.</p>
    ///   - [`state(Option<UpdateLoggingConfigurationState>)`](crate::output::UpdateLoggingConfigurationOutput::state): <p>The state of the logging configuration. When the state is <code>ACTIVE</code>, the configuration is ready to log chat content.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::UpdateLoggingConfigurationOutput::tags): <p>Tags attached to the resource. Array of maps, each of the form <code>string:string (key:value)</code>. </p>
                            /// - On failure, responds with [`SdkError<UpdateLoggingConfigurationError>`](crate::error::UpdateLoggingConfigurationError)
    pub fn update_logging_configuration(&self) -> crate::client::fluent_builders::UpdateLoggingConfiguration {
                                crate::client::fluent_builders::UpdateLoggingConfiguration::new(self.handle.clone())
                            }
}

