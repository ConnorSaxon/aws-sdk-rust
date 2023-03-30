// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteParameterGroup`](crate::client::fluent_builders::DeleteParameterGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`parameter_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteParameterGroup::parameter_group_name) / [`set_parameter_group_name(Option<String>)`](crate::client::fluent_builders::DeleteParameterGroup::set_parameter_group_name): <p>The name of the parameter group to delete.</p>
                            /// - On success, responds with [`DeleteParameterGroupOutput`](crate::output::DeleteParameterGroupOutput) with field(s):
    ///   - [`parameter_group(Option<ParameterGroup>)`](crate::output::DeleteParameterGroupOutput::parameter_group): <p>The parameter group that has been deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteParameterGroupError>`](crate::error::DeleteParameterGroupError)
    pub fn delete_parameter_group(&self) -> crate::client::fluent_builders::DeleteParameterGroup {
                                crate::client::fluent_builders::DeleteParameterGroup::new(self.handle.clone())
                            }
}

