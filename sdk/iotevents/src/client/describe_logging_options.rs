// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeLoggingOptions`](crate::client::fluent_builders::DescribeLoggingOptions) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DescribeLoggingOptions::send) it.
                            /// - On success, responds with [`DescribeLoggingOptionsOutput`](crate::output::DescribeLoggingOptionsOutput) with field(s):
    ///   - [`logging_options(Option<LoggingOptions>)`](crate::output::DescribeLoggingOptionsOutput::logging_options): <p>The current settings of the AWS IoT Events logging options.</p>
                            /// - On failure, responds with [`SdkError<DescribeLoggingOptionsError>`](crate::error::DescribeLoggingOptionsError)
    pub fn describe_logging_options(&self) -> crate::client::fluent_builders::DescribeLoggingOptions {
                                crate::client::fluent_builders::DescribeLoggingOptions::new(self.handle.clone())
                            }
}

