// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SignalWorkflowExecution`](crate::client::fluent_builders::SignalWorkflowExecution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::set_domain): <p>The name of the domain containing the workflow execution to signal.</p>
    ///   - [`workflow_id(impl Into<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::workflow_id) / [`set_workflow_id(Option<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::set_workflow_id): <p>The workflowId of the workflow execution to signal.</p>
    ///   - [`run_id(impl Into<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::run_id) / [`set_run_id(Option<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::set_run_id): <p>The runId of the workflow execution to signal.</p>
    ///   - [`signal_name(impl Into<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::signal_name) / [`set_signal_name(Option<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::set_signal_name): <p>The name of the signal. This name must be meaningful to the target workflow.</p>
    ///   - [`input(impl Into<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::input) / [`set_input(Option<String>)`](crate::client::fluent_builders::SignalWorkflowExecution::set_input): <p>Data to attach to the <code>WorkflowExecutionSignaled</code> event in the target workflow execution's history.</p>
                            /// - On success, responds with [`SignalWorkflowExecutionOutput`](crate::output::SignalWorkflowExecutionOutput)
                            /// - On failure, responds with [`SdkError<SignalWorkflowExecutionError>`](crate::error::SignalWorkflowExecutionError)
    pub fn signal_workflow_execution(&self) -> crate::client::fluent_builders::SignalWorkflowExecution {
                                crate::client::fluent_builders::SignalWorkflowExecution::new(self.handle.clone())
                            }
}

