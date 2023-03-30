// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CopyDBClusterParameterGroup`](crate::client::fluent_builders::CopyDBClusterParameterGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_db_cluster_parameter_group_identifier(impl Into<String>)`](crate::client::fluent_builders::CopyDBClusterParameterGroup::source_db_cluster_parameter_group_identifier) / [`set_source_db_cluster_parameter_group_identifier(Option<String>)`](crate::client::fluent_builders::CopyDBClusterParameterGroup::set_source_db_cluster_parameter_group_identifier): <p>The identifier or Amazon Resource Name (ARN) for the source cluster parameter group.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must specify a valid cluster parameter group.</p> </li>   <li> <p>If the source cluster parameter group is in the same Amazon Web Services Region as the copy, specify a valid parameter group identifier; for example, <code>my-db-cluster-param-group</code>, or a valid ARN.</p> </li>   <li> <p>If the source parameter group is in a different Amazon Web Services Region than the copy, specify a valid cluster parameter group ARN; for example, <code>arn:aws:rds:us-east-1:123456789012:sample-cluster:sample-parameter-group</code>.</p> </li>  </ul>
    ///   - [`target_db_cluster_parameter_group_identifier(impl Into<String>)`](crate::client::fluent_builders::CopyDBClusterParameterGroup::target_db_cluster_parameter_group_identifier) / [`set_target_db_cluster_parameter_group_identifier(Option<String>)`](crate::client::fluent_builders::CopyDBClusterParameterGroup::set_target_db_cluster_parameter_group_identifier): <p>The identifier for the copied cluster parameter group.</p>  <p>Constraints:</p>  <ul>   <li> <p>Cannot be null, empty, or blank.</p> </li>   <li> <p>Must contain from 1 to 255 letters, numbers, or hyphens. </p> </li>   <li> <p>The first character must be a letter.</p> </li>   <li> <p>Cannot end with a hyphen or contain two consecutive hyphens. </p> </li>  </ul>  <p>Example: <code>my-cluster-param-group1</code> </p>
    ///   - [`target_db_cluster_parameter_group_description(impl Into<String>)`](crate::client::fluent_builders::CopyDBClusterParameterGroup::target_db_cluster_parameter_group_description) / [`set_target_db_cluster_parameter_group_description(Option<String>)`](crate::client::fluent_builders::CopyDBClusterParameterGroup::set_target_db_cluster_parameter_group_description): <p>A description for the copied cluster parameter group.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CopyDBClusterParameterGroup::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CopyDBClusterParameterGroup::set_tags): <p>The tags that are to be assigned to the parameter group.</p>
                            /// - On success, responds with [`CopyDbClusterParameterGroupOutput`](crate::output::CopyDbClusterParameterGroupOutput) with field(s):
    ///   - [`db_cluster_parameter_group(Option<DbClusterParameterGroup>)`](crate::output::CopyDbClusterParameterGroupOutput::db_cluster_parameter_group): <p>Detailed information about a cluster parameter group. </p>
                            /// - On failure, responds with [`SdkError<CopyDBClusterParameterGroupError>`](crate::error::CopyDBClusterParameterGroupError)
    pub fn copy_db_cluster_parameter_group(&self) -> crate::client::fluent_builders::CopyDBClusterParameterGroup {
                                crate::client::fluent_builders::CopyDBClusterParameterGroup::new(self.handle.clone())
                            }
}

