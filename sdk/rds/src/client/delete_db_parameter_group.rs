// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDBParameterGroup`](crate::client::fluent_builders::DeleteDBParameterGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_parameter_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteDBParameterGroup::db_parameter_group_name) / [`set_db_parameter_group_name(Option<String>)`](crate::client::fluent_builders::DeleteDBParameterGroup::set_db_parameter_group_name): <p>The name of the DB parameter group.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must be the name of an existing DB parameter group</p> </li>   <li> <p>You can't delete a default DB parameter group</p> </li>   <li> <p>Can't be associated with any DB instances</p> </li>  </ul>
                            /// - On success, responds with [`DeleteDbParameterGroupOutput`](crate::output::DeleteDbParameterGroupOutput)
                            /// - On failure, responds with [`SdkError<DeleteDBParameterGroupError>`](crate::error::DeleteDBParameterGroupError)
    pub fn delete_db_parameter_group(&self) -> crate::client::fluent_builders::DeleteDBParameterGroup {
                                crate::client::fluent_builders::DeleteDBParameterGroup::new(self.handle.clone())
                            }
}

