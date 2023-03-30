// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteParameters`](crate::client::fluent_builders::DeleteParameters) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`names(Vec<String>)`](crate::client::fluent_builders::DeleteParameters::names) / [`set_names(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteParameters::set_names): <p>The names of the parameters to delete. After deleting a parameter, wait for at least 30 seconds to create a parameter with the same name.</p>
                            /// - On success, responds with [`DeleteParametersOutput`](crate::output::DeleteParametersOutput) with field(s):
    ///   - [`deleted_parameters(Option<Vec<String>>)`](crate::output::DeleteParametersOutput::deleted_parameters): <p>The names of the deleted parameters.</p>
    ///   - [`invalid_parameters(Option<Vec<String>>)`](crate::output::DeleteParametersOutput::invalid_parameters): <p>The names of parameters that weren't deleted because the parameters aren't valid.</p>
                            /// - On failure, responds with [`SdkError<DeleteParametersError>`](crate::error::DeleteParametersError)
    pub fn delete_parameters(&self) -> crate::client::fluent_builders::DeleteParameters {
                                crate::client::fluent_builders::DeleteParameters::new(self.handle.clone())
                            }
}

