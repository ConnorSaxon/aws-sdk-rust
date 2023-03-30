// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListLoggingConfigurations`](crate::client::fluent_builders::ListLoggingConfigurations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_marker(impl Into<String>)`](crate::client::fluent_builders::ListLoggingConfigurations::next_marker) / [`set_next_marker(Option<String>)`](crate::client::fluent_builders::ListLoggingConfigurations::set_next_marker): <p>If you specify a value for <code>Limit</code> and you have more <code>LoggingConfigurations</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>LoggingConfigurations</code>. For the second and subsequent <code>ListLoggingConfigurations</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ListLoggingConfigurations</code>.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListLoggingConfigurations::limit) / [`set_limit(i32)`](crate::client::fluent_builders::ListLoggingConfigurations::set_limit): <p>Specifies the number of <code>LoggingConfigurations</code> that you want AWS WAF to return for this request. If you have more <code>LoggingConfigurations</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>LoggingConfigurations</code>.</p>
                            /// - On success, responds with [`ListLoggingConfigurationsOutput`](crate::output::ListLoggingConfigurationsOutput) with field(s):
    ///   - [`logging_configurations(Option<Vec<LoggingConfiguration>>)`](crate::output::ListLoggingConfigurationsOutput::logging_configurations): <p>An array of <code>LoggingConfiguration</code> objects.</p>
    ///   - [`next_marker(Option<String>)`](crate::output::ListLoggingConfigurationsOutput::next_marker): <p>If you have more <code>LoggingConfigurations</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>LoggingConfigurations</code>, submit another <code>ListLoggingConfigurations</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
                            /// - On failure, responds with [`SdkError<ListLoggingConfigurationsError>`](crate::error::ListLoggingConfigurationsError)
    pub fn list_logging_configurations(&self) -> crate::client::fluent_builders::ListLoggingConfigurations {
                                crate::client::fluent_builders::ListLoggingConfigurations::new(self.handle.clone())
                            }
}

