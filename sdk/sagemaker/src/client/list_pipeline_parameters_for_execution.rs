// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPipelineParametersForExecution`](crate::client::fluent_builders::ListPipelineParametersForExecution) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPipelineParametersForExecution::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`pipeline_execution_arn(impl Into<String>)`](crate::client::fluent_builders::ListPipelineParametersForExecution::pipeline_execution_arn) / [`set_pipeline_execution_arn(Option<String>)`](crate::client::fluent_builders::ListPipelineParametersForExecution::set_pipeline_execution_arn): <p>The Amazon Resource Name (ARN) of the pipeline execution.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPipelineParametersForExecution::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPipelineParametersForExecution::set_next_token): <p>If the result of the previous <code>ListPipelineParametersForExecution</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of parameters, use the token in the next request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPipelineParametersForExecution::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListPipelineParametersForExecution::set_max_results): <p>The maximum number of parameters to return in the response.</p>
                            /// - On success, responds with [`ListPipelineParametersForExecutionOutput`](crate::output::ListPipelineParametersForExecutionOutput) with field(s):
    ///   - [`pipeline_parameters(Option<Vec<Parameter>>)`](crate::output::ListPipelineParametersForExecutionOutput::pipeline_parameters): <p>Contains a list of pipeline parameters. This list can be empty. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPipelineParametersForExecutionOutput::next_token): <p>If the result of the previous <code>ListPipelineParametersForExecution</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of parameters, use the token in the next request.</p>
                            /// - On failure, responds with [`SdkError<ListPipelineParametersForExecutionError>`](crate::error::ListPipelineParametersForExecutionError)
    pub fn list_pipeline_parameters_for_execution(&self) -> crate::client::fluent_builders::ListPipelineParametersForExecution {
                                crate::client::fluent_builders::ListPipelineParametersForExecution::new(self.handle.clone())
                            }
}

