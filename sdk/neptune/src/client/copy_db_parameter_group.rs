// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CopyDBParameterGroup`](crate::client::fluent_builders::CopyDBParameterGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_db_parameter_group_identifier(impl Into<String>)`](crate::client::fluent_builders::CopyDBParameterGroup::source_db_parameter_group_identifier) / [`set_source_db_parameter_group_identifier(Option<String>)`](crate::client::fluent_builders::CopyDBParameterGroup::set_source_db_parameter_group_identifier): <p>The identifier or ARN for the source DB parameter group. For information about creating an ARN, see <a href="https://docs.aws.amazon.com/neptune/latest/UserGuide/tagging.ARN.html#tagging.ARN.Constructing"> Constructing an Amazon Resource Name (ARN)</a>.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must specify a valid DB parameter group.</p> </li>   <li> <p>Must specify a valid DB parameter group identifier, for example <code>my-db-param-group</code>, or a valid ARN.</p> </li>  </ul>
    ///   - [`target_db_parameter_group_identifier(impl Into<String>)`](crate::client::fluent_builders::CopyDBParameterGroup::target_db_parameter_group_identifier) / [`set_target_db_parameter_group_identifier(Option<String>)`](crate::client::fluent_builders::CopyDBParameterGroup::set_target_db_parameter_group_identifier): <p>The identifier for the copied DB parameter group.</p>  <p>Constraints:</p>  <ul>   <li> <p>Cannot be null, empty, or blank.</p> </li>   <li> <p>Must contain from 1 to 255 letters, numbers, or hyphens.</p> </li>   <li> <p>First character must be a letter.</p> </li>   <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li>  </ul>  <p>Example: <code>my-db-parameter-group</code> </p>
    ///   - [`target_db_parameter_group_description(impl Into<String>)`](crate::client::fluent_builders::CopyDBParameterGroup::target_db_parameter_group_description) / [`set_target_db_parameter_group_description(Option<String>)`](crate::client::fluent_builders::CopyDBParameterGroup::set_target_db_parameter_group_description): <p>A description for the copied DB parameter group.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CopyDBParameterGroup::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CopyDBParameterGroup::set_tags): <p>The tags to be assigned to the copied DB parameter group.</p>
                            /// - On success, responds with [`CopyDbParameterGroupOutput`](crate::output::CopyDbParameterGroupOutput) with field(s):
    ///   - [`db_parameter_group(Option<DbParameterGroup>)`](crate::output::CopyDbParameterGroupOutput::db_parameter_group): <p>Contains the details of an Amazon Neptune DB parameter group.</p>  <p>This data type is used as a response element in the <code>DescribeDBParameterGroups</code> action.</p>
                            /// - On failure, responds with [`SdkError<CopyDBParameterGroupError>`](crate::error::CopyDBParameterGroupError)
    pub fn copy_db_parameter_group(&self) -> crate::client::fluent_builders::CopyDBParameterGroup {
                                crate::client::fluent_builders::CopyDBParameterGroup::new(self.handle.clone())
                            }
}

