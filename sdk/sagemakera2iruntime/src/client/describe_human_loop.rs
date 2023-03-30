// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeHumanLoop`](crate::client::fluent_builders::DescribeHumanLoop) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`human_loop_name(impl Into<String>)`](crate::client::fluent_builders::DescribeHumanLoop::human_loop_name) / [`set_human_loop_name(Option<String>)`](crate::client::fluent_builders::DescribeHumanLoop::set_human_loop_name): <p>The name of the human loop that you want information about.</p>
                            /// - On success, responds with [`DescribeHumanLoopOutput`](crate::output::DescribeHumanLoopOutput) with field(s):
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeHumanLoopOutput::creation_time): <p>The creation time when Amazon Augmented AI created the human loop.</p>
    ///   - [`failure_reason(Option<String>)`](crate::output::DescribeHumanLoopOutput::failure_reason): <p>The reason why a human loop failed. The failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
    ///   - [`failure_code(Option<String>)`](crate::output::DescribeHumanLoopOutput::failure_code): <p>A failure code that identifies the type of failure.</p>  <p>Possible values: <code>ValidationError</code>, <code>Expired</code>, <code>InternalError</code> </p>
    ///   - [`human_loop_status(Option<HumanLoopStatus>)`](crate::output::DescribeHumanLoopOutput::human_loop_status): <p>The status of the human loop. </p>
    ///   - [`human_loop_name(Option<String>)`](crate::output::DescribeHumanLoopOutput::human_loop_name): <p>The name of the human loop. The name must be lowercase, unique within the Region in your account, and can have up to 63 characters. Valid characters: a-z, 0-9, and - (hyphen).</p>
    ///   - [`human_loop_arn(Option<String>)`](crate::output::DescribeHumanLoopOutput::human_loop_arn): <p>The Amazon Resource Name (ARN) of the human loop.</p>
    ///   - [`flow_definition_arn(Option<String>)`](crate::output::DescribeHumanLoopOutput::flow_definition_arn): <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    ///   - [`human_loop_output(Option<HumanLoopOutput>)`](crate::output::DescribeHumanLoopOutput::human_loop_output): <p>An object that contains information about the output of the human loop.</p>
                            /// - On failure, responds with [`SdkError<DescribeHumanLoopError>`](crate::error::DescribeHumanLoopError)
    pub fn describe_human_loop(&self) -> crate::client::fluent_builders::DescribeHumanLoop {
                                crate::client::fluent_builders::DescribeHumanLoop::new(self.handle.clone())
                            }
}

