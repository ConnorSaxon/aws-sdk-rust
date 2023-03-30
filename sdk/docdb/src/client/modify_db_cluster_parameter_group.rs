// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyDBClusterParameterGroup`](crate::client::fluent_builders::ModifyDBClusterParameterGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_cluster_parameter_group_name(impl Into<String>)`](crate::client::fluent_builders::ModifyDBClusterParameterGroup::db_cluster_parameter_group_name) / [`set_db_cluster_parameter_group_name(Option<String>)`](crate::client::fluent_builders::ModifyDBClusterParameterGroup::set_db_cluster_parameter_group_name): <p>The name of the cluster parameter group to modify.</p>
    ///   - [`parameters(Vec<Parameter>)`](crate::client::fluent_builders::ModifyDBClusterParameterGroup::parameters) / [`set_parameters(Option<Vec<Parameter>>)`](crate::client::fluent_builders::ModifyDBClusterParameterGroup::set_parameters): <p>A list of parameters in the cluster parameter group to modify.</p>
                            /// - On success, responds with [`ModifyDbClusterParameterGroupOutput`](crate::output::ModifyDbClusterParameterGroupOutput) with field(s):
    ///   - [`db_cluster_parameter_group_name(Option<String>)`](crate::output::ModifyDbClusterParameterGroupOutput::db_cluster_parameter_group_name): <p>The name of a cluster parameter group.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must be from 1 to 255 letters or numbers.</p> </li>   <li> <p>The first character must be a letter.</p> </li>   <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li>  </ul> <note>   <p>This value is stored as a lowercase string.</p>  </note>
                            /// - On failure, responds with [`SdkError<ModifyDBClusterParameterGroupError>`](crate::error::ModifyDBClusterParameterGroupError)
    pub fn modify_db_cluster_parameter_group(&self) -> crate::client::fluent_builders::ModifyDBClusterParameterGroup {
                                crate::client::fluent_builders::ModifyDBClusterParameterGroup::new(self.handle.clone())
                            }
}

