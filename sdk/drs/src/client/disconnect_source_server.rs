// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisconnectSourceServer`](crate::client::fluent_builders::DisconnectSourceServer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_server_id(impl Into<String>)`](crate::client::fluent_builders::DisconnectSourceServer::source_server_id) / [`set_source_server_id(Option<String>)`](crate::client::fluent_builders::DisconnectSourceServer::set_source_server_id): <p>The ID of the Source Server to disconnect.</p>
                            /// - On success, responds with [`DisconnectSourceServerOutput`](crate::output::DisconnectSourceServerOutput) with field(s):
    ///   - [`source_server_id(Option<String>)`](crate::output::DisconnectSourceServerOutput::source_server_id): <p>The ID of the Source Server.</p>
    ///   - [`arn(Option<String>)`](crate::output::DisconnectSourceServerOutput::arn): <p>The ARN of the Source Server.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::DisconnectSourceServerOutput::tags): <p>The tags associated with the Source Server.</p>
    ///   - [`recovery_instance_id(Option<String>)`](crate::output::DisconnectSourceServerOutput::recovery_instance_id): <p>The ID of the Recovery Instance associated with this Source Server.</p>
    ///   - [`last_launch_result(Option<LastLaunchResult>)`](crate::output::DisconnectSourceServerOutput::last_launch_result): <p>The status of the last recovery launch of this Source Server.</p>
    ///   - [`data_replication_info(Option<DataReplicationInfo>)`](crate::output::DisconnectSourceServerOutput::data_replication_info): <p>The Data Replication Info of the Source Server.</p>
    ///   - [`life_cycle(Option<LifeCycle>)`](crate::output::DisconnectSourceServerOutput::life_cycle): <p>The lifecycle information of this Source Server.</p>
    ///   - [`source_properties(Option<SourceProperties>)`](crate::output::DisconnectSourceServerOutput::source_properties): <p>The source properties of the Source Server.</p>
    ///   - [`staging_area(Option<StagingArea>)`](crate::output::DisconnectSourceServerOutput::staging_area): <p>The staging area of the source server.</p>
    ///   - [`source_cloud_properties(Option<SourceCloudProperties>)`](crate::output::DisconnectSourceServerOutput::source_cloud_properties): <p>Source cloud properties of the Source Server.</p>
    ///   - [`replication_direction(Option<ReplicationDirection>)`](crate::output::DisconnectSourceServerOutput::replication_direction): <p>Replication direction of the Source Server.</p>
    ///   - [`reversed_direction_source_server_arn(Option<String>)`](crate::output::DisconnectSourceServerOutput::reversed_direction_source_server_arn): <p>For EC2-originated Source Servers which have been failed over and then failed back, this value will mean the ARN of the Source Server on the opposite replication direction.</p>
                            /// - On failure, responds with [`SdkError<DisconnectSourceServerError>`](crate::error::DisconnectSourceServerError)
    pub fn disconnect_source_server(&self) -> crate::client::fluent_builders::DisconnectSourceServer {
                                crate::client::fluent_builders::DisconnectSourceServer::new(self.handle.clone())
                            }
}

