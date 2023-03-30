// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeConversionTasks`](crate::client::fluent_builders::DescribeConversionTasks) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`conversion_task_ids(Vec<String>)`](crate::client::fluent_builders::DescribeConversionTasks::conversion_task_ids) / [`set_conversion_task_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeConversionTasks::set_conversion_task_ids): <p>The conversion task IDs.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeConversionTasks::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeConversionTasks::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DescribeConversionTasksOutput`](crate::output::DescribeConversionTasksOutput) with field(s):
    ///   - [`conversion_tasks(Option<Vec<ConversionTask>>)`](crate::output::DescribeConversionTasksOutput::conversion_tasks): <p>Information about the conversion tasks.</p>
                            /// - On failure, responds with [`SdkError<DescribeConversionTasksError>`](crate::error::DescribeConversionTasksError)
    pub fn describe_conversion_tasks(&self) -> crate::client::fluent_builders::DescribeConversionTasks {
                                crate::client::fluent_builders::DescribeConversionTasks::new(self.handle.clone())
                            }
}

