// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateWorkgroup`](crate::client::fluent_builders::UpdateWorkgroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workgroup_name(impl Into<String>)`](crate::client::fluent_builders::UpdateWorkgroup::workgroup_name) / [`set_workgroup_name(Option<String>)`](crate::client::fluent_builders::UpdateWorkgroup::set_workgroup_name): <p>The name of the workgroup to update.</p>
    ///   - [`base_capacity(i32)`](crate::client::fluent_builders::UpdateWorkgroup::base_capacity) / [`set_base_capacity(Option<i32>)`](crate::client::fluent_builders::UpdateWorkgroup::set_base_capacity): <p>The new base data warehouse capacity in Redshift Processing Units (RPUs).</p>
    ///   - [`enhanced_vpc_routing(bool)`](crate::client::fluent_builders::UpdateWorkgroup::enhanced_vpc_routing) / [`set_enhanced_vpc_routing(Option<bool>)`](crate::client::fluent_builders::UpdateWorkgroup::set_enhanced_vpc_routing): <p>The value that specifies whether to turn on enhanced virtual private cloud (VPC) routing, which forces Amazon Redshift Serverless to route traffic through your VPC.</p>
    ///   - [`config_parameters(Vec<ConfigParameter>)`](crate::client::fluent_builders::UpdateWorkgroup::config_parameters) / [`set_config_parameters(Option<Vec<ConfigParameter>>)`](crate::client::fluent_builders::UpdateWorkgroup::set_config_parameters): <p>An array of parameters to set for advanced control over a database. The options are <code>datestyle</code>, <code>enable_user_activity_logging</code>, <code>query_group</code>, <code>search_path</code>, and <code>max_query_execution_time</code>.</p>
    ///   - [`publicly_accessible(bool)`](crate::client::fluent_builders::UpdateWorkgroup::publicly_accessible) / [`set_publicly_accessible(Option<bool>)`](crate::client::fluent_builders::UpdateWorkgroup::set_publicly_accessible): <p>A value that specifies whether the workgroup can be accessible from a public network.</p>
    ///   - [`subnet_ids(Vec<String>)`](crate::client::fluent_builders::UpdateWorkgroup::subnet_ids) / [`set_subnet_ids(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateWorkgroup::set_subnet_ids): <p>An array of VPC subnet IDs to associate with the workgroup.</p>
    ///   - [`security_group_ids(Vec<String>)`](crate::client::fluent_builders::UpdateWorkgroup::security_group_ids) / [`set_security_group_ids(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateWorkgroup::set_security_group_ids): <p>An array of security group IDs to associate with the workgroup.</p>
    ///   - [`port(i32)`](crate::client::fluent_builders::UpdateWorkgroup::port) / [`set_port(Option<i32>)`](crate::client::fluent_builders::UpdateWorkgroup::set_port): <p>The custom port to use when connecting to a workgroup. Valid port ranges are 5431-5455 and 8191-8215. The default is 5439.</p>
                            /// - On success, responds with [`UpdateWorkgroupOutput`](crate::output::UpdateWorkgroupOutput) with field(s):
    ///   - [`workgroup(Option<Workgroup>)`](crate::output::UpdateWorkgroupOutput::workgroup): <p>The updated workgroup object.</p>
                            /// - On failure, responds with [`SdkError<UpdateWorkgroupError>`](crate::error::UpdateWorkgroupError)
    pub fn update_workgroup(&self) -> crate::client::fluent_builders::UpdateWorkgroup {
                                crate::client::fluent_builders::UpdateWorkgroup::new(self.handle.clone())
                            }
}

