// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateFailbackReplicationConfiguration`](crate::client::fluent_builders::UpdateFailbackReplicationConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`recovery_instance_id(impl Into<String>)`](crate::client::fluent_builders::UpdateFailbackReplicationConfiguration::recovery_instance_id) / [`set_recovery_instance_id(Option<String>)`](crate::client::fluent_builders::UpdateFailbackReplicationConfiguration::set_recovery_instance_id): <p>The ID of the Recovery Instance.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateFailbackReplicationConfiguration::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateFailbackReplicationConfiguration::set_name): <p>The name of the Failback Replication Configuration.</p>
    ///   - [`bandwidth_throttling(i64)`](crate::client::fluent_builders::UpdateFailbackReplicationConfiguration::bandwidth_throttling) / [`set_bandwidth_throttling(i64)`](crate::client::fluent_builders::UpdateFailbackReplicationConfiguration::set_bandwidth_throttling): <p>Configure bandwidth throttling for the outbound data transfer rate of the Recovery Instance in Mbps.</p>
    ///   - [`use_private_ip(bool)`](crate::client::fluent_builders::UpdateFailbackReplicationConfiguration::use_private_ip) / [`set_use_private_ip(Option<bool>)`](crate::client::fluent_builders::UpdateFailbackReplicationConfiguration::set_use_private_ip): <p>Whether to use Private IP for the failback replication of the Recovery Instance.</p>
                            /// - On success, responds with [`UpdateFailbackReplicationConfigurationOutput`](crate::output::UpdateFailbackReplicationConfigurationOutput)
                            /// - On failure, responds with [`SdkError<UpdateFailbackReplicationConfigurationError>`](crate::error::UpdateFailbackReplicationConfigurationError)
    pub fn update_failback_replication_configuration(&self) -> crate::client::fluent_builders::UpdateFailbackReplicationConfiguration {
                                crate::client::fluent_builders::UpdateFailbackReplicationConfiguration::new(self.handle.clone())
                            }
}

