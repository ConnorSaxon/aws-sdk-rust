// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListLoggingConfigurations`](crate::client::fluent_builders::ListLoggingConfigurations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListLoggingConfigurations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListLoggingConfigurations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListLoggingConfigurations::set_next_token): <p>The first logging configurations to retrieve. This is used for pagination; see the <code>nextToken</code> response field.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListLoggingConfigurations::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListLoggingConfigurations::set_max_results): <p>Maximum number of logging configurations to return. Default: 50.</p>
                            /// - On success, responds with [`ListLoggingConfigurationsOutput`](crate::output::ListLoggingConfigurationsOutput) with field(s):
    ///   - [`logging_configurations(Option<Vec<LoggingConfigurationSummary>>)`](crate::output::ListLoggingConfigurationsOutput::logging_configurations): <p>List of the matching logging configurations (summary information only). There is only one type of destination (<code>cloudWatchLogs</code>, <code>firehose</code>, or <code>s3</code>) in a <code>destinationConfiguration</code>.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListLoggingConfigurationsOutput::next_token): <p>If there are more logging configurations than <code>maxResults</code>, use <code>nextToken</code> in the request to get the next set.</p>
                            /// - On failure, responds with [`SdkError<ListLoggingConfigurationsError>`](crate::error::ListLoggingConfigurationsError)
    pub fn list_logging_configurations(&self) -> crate::client::fluent_builders::ListLoggingConfigurations {
                                crate::client::fluent_builders::ListLoggingConfigurations::new(self.handle.clone())
                            }
}

