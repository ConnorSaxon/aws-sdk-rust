// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutLoggingConfiguration`](crate::client::fluent_builders::PutLoggingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`logging_configuration(LoggingConfiguration)`](crate::client::fluent_builders::PutLoggingConfiguration::logging_configuration) / [`set_logging_configuration(Option<LoggingConfiguration>)`](crate::client::fluent_builders::PutLoggingConfiguration::set_logging_configuration): <p>The Amazon Kinesis Data Firehose that contains the inspected traffic information, the redacted fields details, and the Amazon Resource Name (ARN) of the web ACL to monitor.</p> <note>   <p>When specifying <code>Type</code> in <code>RedactedFields</code>, you must use one of the following values: <code>URI</code>, <code>QUERY_STRING</code>, <code>HEADER</code>, or <code>METHOD</code>.</p>  </note>
                            /// - On success, responds with [`PutLoggingConfigurationOutput`](crate::output::PutLoggingConfigurationOutput) with field(s):
    ///   - [`logging_configuration(Option<LoggingConfiguration>)`](crate::output::PutLoggingConfigurationOutput::logging_configuration): <p>The <code>LoggingConfiguration</code> that you submitted in the request.</p>
                            /// - On failure, responds with [`SdkError<PutLoggingConfigurationError>`](crate::error::PutLoggingConfigurationError)
    pub fn put_logging_configuration(&self) -> crate::client::fluent_builders::PutLoggingConfiguration {
                                crate::client::fluent_builders::PutLoggingConfiguration::new(self.handle.clone())
                            }
}

