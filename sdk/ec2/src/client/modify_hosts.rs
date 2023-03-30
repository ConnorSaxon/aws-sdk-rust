// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyHosts`](crate::client::fluent_builders::ModifyHosts) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`auto_placement(AutoPlacement)`](crate::client::fluent_builders::ModifyHosts::auto_placement) / [`set_auto_placement(Option<AutoPlacement>)`](crate::client::fluent_builders::ModifyHosts::set_auto_placement): <p>Specify whether to enable or disable auto-placement.</p>
    ///   - [`host_ids(Vec<String>)`](crate::client::fluent_builders::ModifyHosts::host_ids) / [`set_host_ids(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyHosts::set_host_ids): <p>The IDs of the Dedicated Hosts to modify.</p>
    ///   - [`host_recovery(HostRecovery)`](crate::client::fluent_builders::ModifyHosts::host_recovery) / [`set_host_recovery(Option<HostRecovery>)`](crate::client::fluent_builders::ModifyHosts::set_host_recovery): <p>Indicates whether to enable or disable host recovery for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    ///   - [`instance_type(impl Into<String>)`](crate::client::fluent_builders::ModifyHosts::instance_type) / [`set_instance_type(Option<String>)`](crate::client::fluent_builders::ModifyHosts::set_instance_type): <p>Specifies the instance type to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support only a specific instance type.</p>  <p>If you want to modify a Dedicated Host to support multiple instance types in its current instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    ///   - [`instance_family(impl Into<String>)`](crate::client::fluent_builders::ModifyHosts::instance_family) / [`set_instance_family(Option<String>)`](crate::client::fluent_builders::ModifyHosts::set_instance_family): <p>Specifies the instance family to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support multiple instance types within its current instance family.</p>  <p>If you want to modify a Dedicated Host to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
                            /// - On success, responds with [`ModifyHostsOutput`](crate::output::ModifyHostsOutput) with field(s):
    ///   - [`successful(Option<Vec<String>>)`](crate::output::ModifyHostsOutput::successful): <p>The IDs of the Dedicated Hosts that were successfully modified.</p>
    ///   - [`unsuccessful(Option<Vec<UnsuccessfulItem>>)`](crate::output::ModifyHostsOutput::unsuccessful): <p>The IDs of the Dedicated Hosts that could not be modified. Check whether the setting you requested can be used.</p>
                            /// - On failure, responds with [`SdkError<ModifyHostsError>`](crate::error::ModifyHostsError)
    pub fn modify_hosts(&self) -> crate::client::fluent_builders::ModifyHosts {
                                crate::client::fluent_builders::ModifyHosts::new(self.handle.clone())
                            }
}

