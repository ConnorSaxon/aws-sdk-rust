// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartExecution`](crate::client::fluent_builders::StartExecution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`state_machine_arn(impl Into<String>)`](crate::client::fluent_builders::StartExecution::state_machine_arn) / [`set_state_machine_arn(Option<String>)`](crate::client::fluent_builders::StartExecution::set_state_machine_arn): <p>The Amazon Resource Name (ARN) of the state machine to execute.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::StartExecution::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::StartExecution::set_name): <p>The name of the execution. This name must be unique for your Amazon Web Services account, region, and state machine for 90 days. For more information, see <a href="https://docs.aws.amazon.com/step-functions/latest/dg/limits.html#service-limits-state-machine-executions"> Limits Related to State Machine Executions</a> in the <i>Step Functions Developer Guide</i>.</p>  <p>A name must <i>not</i> contain:</p>  <ul>   <li> <p>white space</p> </li>   <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>   <li> <p>wildcard characters <code>? *</code> </p> </li>   <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>   <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>  </ul>  <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    ///   - [`input(impl Into<String>)`](crate::client::fluent_builders::StartExecution::input) / [`set_input(Option<String>)`](crate::client::fluent_builders::StartExecution::set_input): <p>The string that contains the JSON input data for the execution, for example:</p>  <p> <code>"input": "{\"first_name\" : \"test\"}"</code> </p> <note>   <p>If you don't include any JSON input data, you still must include the two braces, for example: <code>"input": "{}"</code> </p>  </note>  <p>Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.</p>
    ///   - [`trace_header(impl Into<String>)`](crate::client::fluent_builders::StartExecution::trace_header) / [`set_trace_header(Option<String>)`](crate::client::fluent_builders::StartExecution::set_trace_header): <p>Passes the X-Ray trace header. The trace header can also be passed in the request payload.</p>
                            /// - On success, responds with [`StartExecutionOutput`](crate::output::StartExecutionOutput) with field(s):
    ///   - [`execution_arn(Option<String>)`](crate::output::StartExecutionOutput::execution_arn): <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    ///   - [`start_date(Option<DateTime>)`](crate::output::StartExecutionOutput::start_date): <p>The date the execution is started.</p>
                            /// - On failure, responds with [`SdkError<StartExecutionError>`](crate::error::StartExecutionError)
    pub fn start_execution(&self) -> crate::client::fluent_builders::StartExecution {
                                crate::client::fluent_builders::StartExecution::new(self.handle.clone())
                            }
}

