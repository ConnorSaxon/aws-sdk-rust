// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateReplicationConfiguration`](crate::client::fluent_builders::UpdateReplicationConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_server_id(impl Into<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::source_server_id) / [`set_source_server_id(Option<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_source_server_id): <p>The ID of the Source Server for this Replication Configuration.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_name): <p>The name of the Replication Configuration.</p>
    ///   - [`staging_area_subnet_id(impl Into<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::staging_area_subnet_id) / [`set_staging_area_subnet_id(Option<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_staging_area_subnet_id): <p>The subnet to be used by the replication staging area.</p>
    ///   - [`associate_default_security_group(bool)`](crate::client::fluent_builders::UpdateReplicationConfiguration::associate_default_security_group) / [`set_associate_default_security_group(Option<bool>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_associate_default_security_group): <p>Whether to associate the default Elastic Disaster Recovery Security group with the Replication Configuration.</p>
    ///   - [`replication_servers_security_groups_i_ds(Vec<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::replication_servers_security_groups_i_ds) / [`set_replication_servers_security_groups_i_ds(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_replication_servers_security_groups_i_ds): <p>The security group IDs that will be used by the replication server.</p>
    ///   - [`replication_server_instance_type(impl Into<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::replication_server_instance_type) / [`set_replication_server_instance_type(Option<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_replication_server_instance_type): <p>The instance type to be used for the replication server.</p>
    ///   - [`use_dedicated_replication_server(bool)`](crate::client::fluent_builders::UpdateReplicationConfiguration::use_dedicated_replication_server) / [`set_use_dedicated_replication_server(Option<bool>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_use_dedicated_replication_server): <p>Whether to use a dedicated Replication Server in the replication staging area.</p>
    ///   - [`default_large_staging_disk_type(ReplicationConfigurationDefaultLargeStagingDiskType)`](crate::client::fluent_builders::UpdateReplicationConfiguration::default_large_staging_disk_type) / [`set_default_large_staging_disk_type(Option<ReplicationConfigurationDefaultLargeStagingDiskType>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_default_large_staging_disk_type): <p>The Staging Disk EBS volume type to be used during replication.</p>
    ///   - [`replicated_disks(Vec<ReplicationConfigurationReplicatedDisk>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::replicated_disks) / [`set_replicated_disks(Option<Vec<ReplicationConfigurationReplicatedDisk>>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_replicated_disks): <p>The configuration of the disks of the Source Server to be replicated.</p>
    ///   - [`ebs_encryption(ReplicationConfigurationEbsEncryption)`](crate::client::fluent_builders::UpdateReplicationConfiguration::ebs_encryption) / [`set_ebs_encryption(Option<ReplicationConfigurationEbsEncryption>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_ebs_encryption): <p>The type of EBS encryption to be used during replication.</p>
    ///   - [`ebs_encryption_key_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::ebs_encryption_key_arn) / [`set_ebs_encryption_key_arn(Option<String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_ebs_encryption_key_arn): <p>The ARN of the EBS encryption key to be used during replication.</p>
    ///   - [`bandwidth_throttling(i64)`](crate::client::fluent_builders::UpdateReplicationConfiguration::bandwidth_throttling) / [`set_bandwidth_throttling(i64)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_bandwidth_throttling): <p>Configure bandwidth throttling for the outbound data transfer rate of the Source Server in Mbps.</p>
    ///   - [`data_plane_routing(ReplicationConfigurationDataPlaneRouting)`](crate::client::fluent_builders::UpdateReplicationConfiguration::data_plane_routing) / [`set_data_plane_routing(Option<ReplicationConfigurationDataPlaneRouting>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_data_plane_routing): <p>The data plane routing mechanism that will be used for replication.</p>
    ///   - [`create_public_ip(bool)`](crate::client::fluent_builders::UpdateReplicationConfiguration::create_public_ip) / [`set_create_public_ip(Option<bool>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_create_public_ip): <p>Whether to create a Public IP for the Recovery Instance by default.</p>
    ///   - [`staging_area_tags(HashMap<String, String>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::staging_area_tags) / [`set_staging_area_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_staging_area_tags): <p>A set of tags to be associated with all resources created in the replication staging area: EC2 replication server, EBS volumes, EBS snapshots, etc.</p>
    ///   - [`pit_policy(Vec<PitPolicyRule>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::pit_policy) / [`set_pit_policy(Option<Vec<PitPolicyRule>>)`](crate::client::fluent_builders::UpdateReplicationConfiguration::set_pit_policy): <p>The Point in time (PIT) policy to manage snapshots taken during replication.</p>
                            /// - On success, responds with [`UpdateReplicationConfigurationOutput`](crate::output::UpdateReplicationConfigurationOutput) with field(s):
    ///   - [`source_server_id(Option<String>)`](crate::output::UpdateReplicationConfigurationOutput::source_server_id): <p>The ID of the Source Server for this Replication Configuration.</p>
    ///   - [`name(Option<String>)`](crate::output::UpdateReplicationConfigurationOutput::name): <p>The name of the Replication Configuration.</p>
    ///   - [`staging_area_subnet_id(Option<String>)`](crate::output::UpdateReplicationConfigurationOutput::staging_area_subnet_id): <p>The subnet to be used by the replication staging area.</p>
    ///   - [`associate_default_security_group(Option<bool>)`](crate::output::UpdateReplicationConfigurationOutput::associate_default_security_group): <p>Whether to associate the default Elastic Disaster Recovery Security group with the Replication Configuration.</p>
    ///   - [`replication_servers_security_groups_i_ds(Option<Vec<String>>)`](crate::output::UpdateReplicationConfigurationOutput::replication_servers_security_groups_i_ds): <p>The security group IDs that will be used by the replication server.</p>
    ///   - [`replication_server_instance_type(Option<String>)`](crate::output::UpdateReplicationConfigurationOutput::replication_server_instance_type): <p>The instance type to be used for the replication server.</p>
    ///   - [`use_dedicated_replication_server(Option<bool>)`](crate::output::UpdateReplicationConfigurationOutput::use_dedicated_replication_server): <p>Whether to use a dedicated Replication Server in the replication staging area.</p>
    ///   - [`default_large_staging_disk_type(Option<ReplicationConfigurationDefaultLargeStagingDiskType>)`](crate::output::UpdateReplicationConfigurationOutput::default_large_staging_disk_type): <p>The Staging Disk EBS volume type to be used during replication.</p>
    ///   - [`replicated_disks(Option<Vec<ReplicationConfigurationReplicatedDisk>>)`](crate::output::UpdateReplicationConfigurationOutput::replicated_disks): <p>The configuration of the disks of the Source Server to be replicated.</p>
    ///   - [`ebs_encryption(Option<ReplicationConfigurationEbsEncryption>)`](crate::output::UpdateReplicationConfigurationOutput::ebs_encryption): <p>The type of EBS encryption to be used during replication.</p>
    ///   - [`ebs_encryption_key_arn(Option<String>)`](crate::output::UpdateReplicationConfigurationOutput::ebs_encryption_key_arn): <p>The ARN of the EBS encryption key to be used during replication.</p>
    ///   - [`bandwidth_throttling(i64)`](crate::output::UpdateReplicationConfigurationOutput::bandwidth_throttling): <p>Configure bandwidth throttling for the outbound data transfer rate of the Source Server in Mbps.</p>
    ///   - [`data_plane_routing(Option<ReplicationConfigurationDataPlaneRouting>)`](crate::output::UpdateReplicationConfigurationOutput::data_plane_routing): <p>The data plane routing mechanism that will be used for replication.</p>
    ///   - [`create_public_ip(Option<bool>)`](crate::output::UpdateReplicationConfigurationOutput::create_public_ip): <p>Whether to create a Public IP for the Recovery Instance by default.</p>
    ///   - [`staging_area_tags(Option<HashMap<String, String>>)`](crate::output::UpdateReplicationConfigurationOutput::staging_area_tags): <p>A set of tags to be associated with all resources created in the replication staging area: EC2 replication server, EBS volumes, EBS snapshots, etc.</p>
    ///   - [`pit_policy(Option<Vec<PitPolicyRule>>)`](crate::output::UpdateReplicationConfigurationOutput::pit_policy): <p>The Point in time (PIT) policy to manage snapshots taken during replication.</p>
                            /// - On failure, responds with [`SdkError<UpdateReplicationConfigurationError>`](crate::error::UpdateReplicationConfigurationError)
    pub fn update_replication_configuration(&self) -> crate::client::fluent_builders::UpdateReplicationConfiguration {
                                crate::client::fluent_builders::UpdateReplicationConfiguration::new(self.handle.clone())
                            }
}

