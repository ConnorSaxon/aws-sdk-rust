// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDBCluster`](crate::client::fluent_builders::CreateDBCluster) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`availability_zones(Vec<String>)`](crate::client::fluent_builders::CreateDBCluster::availability_zones) / [`set_availability_zones(Option<Vec<String>>)`](crate::client::fluent_builders::CreateDBCluster::set_availability_zones): <p>A list of Amazon EC2 Availability Zones that instances in the cluster can be created in.</p>
    ///   - [`backup_retention_period(i32)`](crate::client::fluent_builders::CreateDBCluster::backup_retention_period) / [`set_backup_retention_period(Option<i32>)`](crate::client::fluent_builders::CreateDBCluster::set_backup_retention_period): <p>The number of days for which automated backups are retained. You must specify a minimum value of 1.</p>  <p>Default: 1</p>  <p>Constraints:</p>  <ul>   <li> <p>Must be a value from 1 to 35.</p> </li>  </ul>
    ///   - [`db_cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::db_cluster_identifier) / [`set_db_cluster_identifier(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_db_cluster_identifier): <p>The cluster identifier. This parameter is stored as a lowercase string.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens. </p> </li>   <li> <p>The first character must be a letter.</p> </li>   <li> <p>Cannot end with a hyphen or contain two consecutive hyphens. </p> </li>  </ul>  <p>Example: <code>my-cluster</code> </p>
    ///   - [`db_cluster_parameter_group_name(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::db_cluster_parameter_group_name) / [`set_db_cluster_parameter_group_name(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_db_cluster_parameter_group_name): <p>The name of the cluster parameter group to associate with this cluster.</p>
    ///   - [`vpc_security_group_ids(Vec<String>)`](crate::client::fluent_builders::CreateDBCluster::vpc_security_group_ids) / [`set_vpc_security_group_ids(Option<Vec<String>>)`](crate::client::fluent_builders::CreateDBCluster::set_vpc_security_group_ids): <p>A list of EC2 VPC security groups to associate with this cluster. </p>
    ///   - [`db_subnet_group_name(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::db_subnet_group_name) / [`set_db_subnet_group_name(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_db_subnet_group_name): <p>A subnet group to associate with this cluster.</p>  <p>Constraints: Must match the name of an existing <code>DBSubnetGroup</code>. Must not be default.</p>  <p>Example: <code>mySubnetgroup</code> </p>
    ///   - [`engine(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::engine) / [`set_engine(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_engine): <p>The name of the database engine to be used for this cluster.</p>  <p>Valid values: <code>docdb</code> </p>
    ///   - [`engine_version(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::engine_version) / [`set_engine_version(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_engine_version): <p>The version number of the database engine to use. The <code>--engine-version</code> will default to the latest major engine version. For production workloads, we recommend explicitly declaring this parameter with the intended major engine version.</p>
    ///   - [`port(i32)`](crate::client::fluent_builders::CreateDBCluster::port) / [`set_port(Option<i32>)`](crate::client::fluent_builders::CreateDBCluster::set_port): <p>The port number on which the instances in the cluster accept connections.</p>
    ///   - [`master_username(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::master_username) / [`set_master_username(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_master_username): <p>The name of the master user for the cluster.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must be from 1 to 63 letters or numbers.</p> </li>   <li> <p>The first character must be a letter.</p> </li>   <li> <p>Cannot be a reserved word for the chosen database engine. </p> </li>  </ul>
    ///   - [`master_user_password(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::master_user_password) / [`set_master_user_password(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_master_user_password): <p>The password for the master database user. This password can contain any printable ASCII character except forward slash (/), double quote ("), or the "at" symbol (@).</p>  <p>Constraints: Must contain from 8 to 100 characters.</p>
    ///   - [`preferred_backup_window(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::preferred_backup_window) / [`set_preferred_backup_window(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_preferred_backup_window): <p>The daily time range during which automated backups are created if automated backups are enabled using the <code>BackupRetentionPeriod</code> parameter. </p>  <p>The default is a 30-minute window selected at random from an 8-hour block of time for each Amazon Web Services Region. </p>  <p>Constraints:</p>  <ul>   <li> <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p> </li>   <li> <p>Must be in Universal Coordinated Time (UTC).</p> </li>   <li> <p>Must not conflict with the preferred maintenance window. </p> </li>   <li> <p>Must be at least 30 minutes.</p> </li>  </ul>
    ///   - [`preferred_maintenance_window(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::preferred_maintenance_window) / [`set_preferred_maintenance_window(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_preferred_maintenance_window): <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>  <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p>  <p>The default is a 30-minute window selected at random from an 8-hour block of time for each Amazon Web Services Region, occurring on a random day of the week.</p>  <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p>  <p>Constraints: Minimum 30-minute window.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateDBCluster::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateDBCluster::set_tags): <p>The tags to be assigned to the cluster.</p>
    ///   - [`storage_encrypted(bool)`](crate::client::fluent_builders::CreateDBCluster::storage_encrypted) / [`set_storage_encrypted(Option<bool>)`](crate::client::fluent_builders::CreateDBCluster::set_storage_encrypted): <p>Specifies whether the cluster is encrypted.</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_kms_key_id): <p>The KMS key identifier for an encrypted cluster.</p>  <p>The KMS key identifier is the Amazon Resource Name (ARN) for the KMS encryption key. If you are creating a cluster using the same Amazon Web Services account that owns the KMS encryption key that is used to encrypt the new cluster, you can use the KMS key alias instead of the ARN for the KMS encryption key.</p>  <p>If an encryption key is not specified in <code>KmsKeyId</code>: </p>  <ul>   <li> <p>If the <code>StorageEncrypted</code> parameter is <code>true</code>, Amazon DocumentDB uses your default encryption key. </p> </li>  </ul>  <p>KMS creates the default encryption key for your Amazon Web Services account. Your Amazon Web Services account has a different default encryption key for each Amazon Web Services Regions.</p>
    ///   - [`pre_signed_url(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::pre_signed_url) / [`set_pre_signed_url(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_pre_signed_url): <p>Not currently supported. </p>
    ///   - [`enable_cloudwatch_logs_exports(Vec<String>)`](crate::client::fluent_builders::CreateDBCluster::enable_cloudwatch_logs_exports) / [`set_enable_cloudwatch_logs_exports(Option<Vec<String>>)`](crate::client::fluent_builders::CreateDBCluster::set_enable_cloudwatch_logs_exports): <p>A list of log types that need to be enabled for exporting to Amazon CloudWatch Logs. You can enable audit logs or profiler logs. For more information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/event-auditing.html"> Auditing Amazon DocumentDB Events</a> and <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/profiling.html"> Profiling Amazon DocumentDB Operations</a>. </p>
    ///   - [`deletion_protection(bool)`](crate::client::fluent_builders::CreateDBCluster::deletion_protection) / [`set_deletion_protection(Option<bool>)`](crate::client::fluent_builders::CreateDBCluster::set_deletion_protection): <p>Specifies whether this cluster can be deleted. If <code>DeletionProtection</code> is enabled, the cluster cannot be deleted unless it is modified and <code>DeletionProtection</code> is disabled. <code>DeletionProtection</code> protects clusters from being accidentally deleted.</p>
    ///   - [`global_cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::CreateDBCluster::global_cluster_identifier) / [`set_global_cluster_identifier(Option<String>)`](crate::client::fluent_builders::CreateDBCluster::set_global_cluster_identifier): <p>The cluster identifier of the new global cluster.</p>
                            /// - On success, responds with [`CreateDbClusterOutput`](crate::output::CreateDbClusterOutput) with field(s):
    ///   - [`db_cluster(Option<DbCluster>)`](crate::output::CreateDbClusterOutput::db_cluster): <p>Detailed information about a cluster. </p>
                            /// - On failure, responds with [`SdkError<CreateDBClusterError>`](crate::error::CreateDBClusterError)
    pub fn create_db_cluster(&self) -> crate::client::fluent_builders::CreateDBCluster {
                                crate::client::fluent_builders::CreateDBCluster::new(self.handle.clone())
                            }
}

