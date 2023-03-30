// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchCreateVariable`](crate::client::fluent_builders::BatchCreateVariable) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`variable_entries(Vec<VariableEntry>)`](crate::client::fluent_builders::BatchCreateVariable::variable_entries) / [`set_variable_entries(Option<Vec<VariableEntry>>)`](crate::client::fluent_builders::BatchCreateVariable::set_variable_entries): <p>The list of variables for the batch create variable request.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::BatchCreateVariable::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::BatchCreateVariable::set_tags): <p>A collection of key and value pairs.</p>
                            /// - On success, responds with [`BatchCreateVariableOutput`](crate::output::BatchCreateVariableOutput) with field(s):
    ///   - [`errors(Option<Vec<BatchCreateVariableError>>)`](crate::output::BatchCreateVariableOutput::errors): <p>Provides the errors for the <code>BatchCreateVariable</code> request.</p>
                            /// - On failure, responds with [`SdkError<BatchCreateVariableError>`](crate::error::BatchCreateVariableError)
    pub fn batch_create_variable(&self) -> crate::client::fluent_builders::BatchCreateVariable {
                                crate::client::fluent_builders::BatchCreateVariable::new(self.handle.clone())
                            }
}

