// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutExternalEvaluation`](crate::client::fluent_builders::PutExternalEvaluation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`config_rule_name(impl Into<String>)`](crate::client::fluent_builders::PutExternalEvaluation::config_rule_name) / [`set_config_rule_name(Option<String>)`](crate::client::fluent_builders::PutExternalEvaluation::set_config_rule_name): <p>The name of the Config rule.</p>
    ///   - [`external_evaluation(ExternalEvaluation)`](crate::client::fluent_builders::PutExternalEvaluation::external_evaluation) / [`set_external_evaluation(Option<ExternalEvaluation>)`](crate::client::fluent_builders::PutExternalEvaluation::set_external_evaluation): <p>An <code>ExternalEvaluation</code> object that provides details about compliance.</p>
                            /// - On success, responds with [`PutExternalEvaluationOutput`](crate::output::PutExternalEvaluationOutput)
                            /// - On failure, responds with [`SdkError<PutExternalEvaluationError>`](crate::error::PutExternalEvaluationError)
    pub fn put_external_evaluation(&self) -> crate::client::fluent_builders::PutExternalEvaluation {
                                crate::client::fluent_builders::PutExternalEvaluation::new(self.handle.clone())
                            }
}

