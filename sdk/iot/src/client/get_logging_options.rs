// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetLoggingOptions`](crate::client::fluent_builders::GetLoggingOptions) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::GetLoggingOptions::send) it.
                            /// - On success, responds with [`GetLoggingOptionsOutput`](crate::output::GetLoggingOptionsOutput) with field(s):
    ///   - [`role_arn(Option<String>)`](crate::output::GetLoggingOptionsOutput::role_arn): <p>The ARN of the IAM role that grants access.</p>
    ///   - [`log_level(Option<LogLevel>)`](crate::output::GetLoggingOptionsOutput::log_level): <p>The logging level.</p>
                            /// - On failure, responds with [`SdkError<GetLoggingOptionsError>`](crate::error::GetLoggingOptionsError)
    pub fn get_logging_options(&self) -> crate::client::fluent_builders::GetLoggingOptions {
                                crate::client::fluent_builders::GetLoggingOptions::new(self.handle.clone())
                            }
}

