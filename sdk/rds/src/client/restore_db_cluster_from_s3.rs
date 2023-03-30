// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RestoreDBClusterFromS3`](crate::client::fluent_builders::RestoreDBClusterFromS3) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`availability_zones(Vec<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::availability_zones) / [`set_availability_zones(Option<Vec<String>>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_availability_zones): <p>A list of Availability Zones (AZs) where instances in the restored DB cluster can be created.</p>
    ///   - [`backup_retention_period(i32)`](crate::client::fluent_builders::RestoreDBClusterFromS3::backup_retention_period) / [`set_backup_retention_period(Option<i32>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_backup_retention_period): <p>The number of days for which automated backups of the restored DB cluster are retained. You must specify a minimum value of 1.</p>  <p>Default: 1</p>  <p>Constraints:</p>  <ul>   <li> <p>Must be a value from 1 to 35</p> </li>  </ul>
    ///   - [`character_set_name(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::character_set_name) / [`set_character_set_name(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_character_set_name): <p>A value that indicates that the restored DB cluster should be associated with the specified CharacterSet.</p>
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_database_name): <p>The database name for the restored DB cluster.</p>
    ///   - [`db_cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::db_cluster_identifier) / [`set_db_cluster_identifier(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_db_cluster_identifier): <p>The name of the DB cluster to create from the source data in the Amazon S3 bucket. This parameter isn't case-sensitive.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li>   <li> <p>First character must be a letter.</p> </li>   <li> <p>Can't end with a hyphen or contain two consecutive hyphens.</p> </li>  </ul>  <p>Example: <code>my-cluster1</code> </p>
    ///   - [`db_cluster_parameter_group_name(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::db_cluster_parameter_group_name) / [`set_db_cluster_parameter_group_name(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_db_cluster_parameter_group_name): <p>The name of the DB cluster parameter group to associate with the restored DB cluster. If this argument is omitted, <code>default.aurora5.6</code> is used.</p>  <p>Constraints:</p>  <ul>   <li> <p>If supplied, must match the name of an existing DBClusterParameterGroup.</p> </li>  </ul>
    ///   - [`vpc_security_group_ids(Vec<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::vpc_security_group_ids) / [`set_vpc_security_group_ids(Option<Vec<String>>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_vpc_security_group_ids): <p>A list of EC2 VPC security groups to associate with the restored DB cluster.</p>
    ///   - [`db_subnet_group_name(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::db_subnet_group_name) / [`set_db_subnet_group_name(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_db_subnet_group_name): <p>A DB subnet group to associate with the restored DB cluster.</p>  <p>Constraints: If supplied, must match the name of an existing DBSubnetGroup.</p>  <p>Example: <code>mydbsubnetgroup</code> </p>
    ///   - [`engine(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::engine) / [`set_engine(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_engine): <p>The name of the database engine to be used for this DB cluster.</p>  <p>Valid Values: <code>aurora</code> (for MySQL 5.6-compatible Aurora) and <code>aurora-mysql</code> (for MySQL 5.7-compatible and MySQL 8.0-compatible Aurora)</p>
    ///   - [`engine_version(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::engine_version) / [`set_engine_version(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_engine_version): <p>The version number of the database engine to use.</p>  <p>To list all of the available engine versions for <code>aurora</code> (for MySQL 5.6-compatible Aurora), use the following command:</p>  <p> <code>aws rds describe-db-engine-versions --engine aurora --query "DBEngineVersions[].EngineVersion"</code> </p>  <p>To list all of the available engine versions for <code>aurora-mysql</code> (for MySQL 5.7-compatible and MySQL 8.0-compatible Aurora), use the following command:</p>  <p> <code>aws rds describe-db-engine-versions --engine aurora-mysql --query "DBEngineVersions[].EngineVersion"</code> </p>  <p> <b>Aurora MySQL</b> </p>  <p>Example: <code>5.6.10a</code>, <code>5.6.mysql_aurora.1.19.2</code>, <code>5.7.mysql_aurora.2.07.1</code>, <code>8.0.mysql_aurora.3.02.0</code> </p>
    ///   - [`port(i32)`](crate::client::fluent_builders::RestoreDBClusterFromS3::port) / [`set_port(Option<i32>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_port): <p>The port number on which the instances in the restored DB cluster accept connections.</p>  <p>Default: <code>3306</code> </p>
    ///   - [`master_username(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::master_username) / [`set_master_username(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_master_username): <p>The name of the master user for the restored DB cluster.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must be 1 to 16 letters or numbers.</p> </li>   <li> <p>First character must be a letter.</p> </li>   <li> <p>Can't be a reserved word for the chosen database engine.</p> </li>  </ul>
    ///   - [`master_user_password(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::master_user_password) / [`set_master_user_password(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_master_user_password): <p>The password for the master database user. This password can contain any printable ASCII character except "/", """, or "@".</p>  <p>Constraints:</p>  <ul>   <li> <p>Must contain from 8 to 41 characters.</p> </li>   <li> <p>Can't be specified if <code>ManageMasterUserPassword</code> is turned on.</p> </li>  </ul>
    ///   - [`option_group_name(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::option_group_name) / [`set_option_group_name(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_option_group_name): <p>A value that indicates that the restored DB cluster should be associated with the specified option group.</p>  <p>Permanent options can't be removed from an option group. An option group can't be removed from a DB cluster once it is associated with a DB cluster.</p>
    ///   - [`preferred_backup_window(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::preferred_backup_window) / [`set_preferred_backup_window(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_preferred_backup_window): <p>The daily time range during which automated backups are created if automated backups are enabled using the <code>BackupRetentionPeriod</code> parameter.</p>  <p>The default is a 30-minute window selected at random from an 8-hour block of time for each Amazon Web Services Region. To view the time blocks available, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Aurora.Managing.Backups.html#Aurora.Managing.Backups.BackupWindow"> Backup window</a> in the <i>Amazon Aurora User Guide</i>.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must be in the format <code>hh24:mi-hh24:mi</code>.</p> </li>   <li> <p>Must be in Universal Coordinated Time (UTC).</p> </li>   <li> <p>Must not conflict with the preferred maintenance window.</p> </li>   <li> <p>Must be at least 30 minutes.</p> </li>  </ul>
    ///   - [`preferred_maintenance_window(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::preferred_maintenance_window) / [`set_preferred_maintenance_window(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_preferred_maintenance_window): <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>  <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p>  <p>The default is a 30-minute window selected at random from an 8-hour block of time for each Amazon Web Services Region, occurring on a random day of the week. To see the time blocks available, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_UpgradeDBInstance.Maintenance.html#AdjustingTheMaintenanceWindow.Aurora"> Adjusting the Preferred Maintenance Window</a> in the <i>Amazon Aurora User Guide</i>.</p>  <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p>  <p>Constraints: Minimum 30-minute window.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_tags): <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i> </p>
    ///   - [`storage_encrypted(bool)`](crate::client::fluent_builders::RestoreDBClusterFromS3::storage_encrypted) / [`set_storage_encrypted(Option<bool>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_storage_encrypted): <p>A value that indicates whether the restored DB cluster is encrypted.</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_kms_key_id): <p>The Amazon Web Services KMS key identifier for an encrypted DB cluster.</p>  <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN.</p>  <p>If the StorageEncrypted parameter is enabled, and you do not specify a value for the <code>KmsKeyId</code> parameter, then Amazon RDS will use your default KMS key. There is a default KMS key for your Amazon Web Services account. Your Amazon Web Services account has a different default KMS key for each Amazon Web Services Region.</p>
    ///   - [`enable_iam_database_authentication(bool)`](crate::client::fluent_builders::RestoreDBClusterFromS3::enable_iam_database_authentication) / [`set_enable_iam_database_authentication(Option<bool>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_enable_iam_database_authentication): <p>A value that indicates whether to enable mapping of Amazon Web Services Identity and Access Management (IAM) accounts to database accounts. By default, mapping isn't enabled.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/UsingWithRDS.IAMDBAuth.html"> IAM Database Authentication</a> in the <i>Amazon Aurora User Guide</i>.</p>
    ///   - [`source_engine(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::source_engine) / [`set_source_engine(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_source_engine): <p>The identifier for the database engine that was backed up to create the files stored in the Amazon S3 bucket.</p>  <p>Valid values: <code>mysql</code> </p>
    ///   - [`source_engine_version(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::source_engine_version) / [`set_source_engine_version(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_source_engine_version): <p>The version of the database that the backup files were created from.</p>  <p>MySQL versions 5.5, 5.6, and 5.7 are supported.</p>  <p>Example: <code>5.6.40</code>, <code>5.7.28</code> </p>
    ///   - [`s3_bucket_name(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::s3_bucket_name) / [`set_s3_bucket_name(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_s3_bucket_name): <p>The name of the Amazon S3 bucket that contains the data used to create the Amazon Aurora DB cluster.</p>
    ///   - [`s3_prefix(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::s3_prefix) / [`set_s3_prefix(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_s3_prefix): <p>The prefix for all of the file names that contain the data used to create the Amazon Aurora DB cluster. If you do not specify a <b>SourceS3Prefix</b> value, then the Amazon Aurora DB cluster is created by using all of the files in the Amazon S3 bucket.</p>
    ///   - [`s3_ingestion_role_arn(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::s3_ingestion_role_arn) / [`set_s3_ingestion_role_arn(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_s3_ingestion_role_arn): <p>The Amazon Resource Name (ARN) of the Amazon Web Services Identity and Access Management (IAM) role that authorizes Amazon RDS to access the Amazon S3 bucket on your behalf.</p>
    ///   - [`backtrack_window(i64)`](crate::client::fluent_builders::RestoreDBClusterFromS3::backtrack_window) / [`set_backtrack_window(Option<i64>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_backtrack_window): <p>The target backtrack window, in seconds. To disable backtracking, set this value to 0.</p> <note>   <p>Currently, Backtrack is only supported for Aurora MySQL DB clusters.</p>  </note>  <p>Default: 0</p>  <p>Constraints:</p>  <ul>   <li> <p>If specified, this value must be set to a number from 0 to 259,200 (72 hours).</p> </li>  </ul>
    ///   - [`enable_cloudwatch_logs_exports(Vec<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::enable_cloudwatch_logs_exports) / [`set_enable_cloudwatch_logs_exports(Option<Vec<String>>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_enable_cloudwatch_logs_exports): <p>The list of logs that the restored DB cluster is to export to CloudWatch Logs. The values in the list depend on the DB engine being used.</p>  <p> <b>Aurora MySQL</b> </p>  <p>Possible values are <code>audit</code>, <code>error</code>, <code>general</code>, and <code>slowquery</code>.</p>  <p> <b>Aurora PostgreSQL</b> </p>  <p>Possible value is <code>postgresql</code>.</p>  <p>For more information about exporting CloudWatch Logs for Amazon Aurora, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_LogAccess.html#USER_LogAccess.Procedural.UploadtoCloudWatch">Publishing Database Logs to Amazon CloudWatch Logs</a> in the <i>Amazon Aurora User Guide</i>.</p>
    ///   - [`deletion_protection(bool)`](crate::client::fluent_builders::RestoreDBClusterFromS3::deletion_protection) / [`set_deletion_protection(Option<bool>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_deletion_protection): <p>A value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection isn't enabled.</p>
    ///   - [`copy_tags_to_snapshot(bool)`](crate::client::fluent_builders::RestoreDBClusterFromS3::copy_tags_to_snapshot) / [`set_copy_tags_to_snapshot(Option<bool>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_copy_tags_to_snapshot): <p>A value that indicates whether to copy all tags from the restored DB cluster to snapshots of the restored DB cluster. The default is not to copy them.</p>
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_domain): <p>Specify the Active Directory directory ID to restore the DB cluster in. The domain must be created prior to this operation.</p>  <p>For Amazon Aurora DB clusters, Amazon RDS can use Kerberos Authentication to authenticate users that connect to the DB cluster. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/kerberos-authentication.html">Kerberos Authentication</a> in the <i>Amazon Aurora User Guide</i>.</p>
    ///   - [`domain_iam_role_name(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::domain_iam_role_name) / [`set_domain_iam_role_name(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_domain_iam_role_name): <p>Specify the name of the IAM role to be used when making API calls to the Directory Service.</p>
    ///   - [`serverless_v2_scaling_configuration(ServerlessV2ScalingConfiguration)`](crate::client::fluent_builders::RestoreDBClusterFromS3::serverless_v2_scaling_configuration) / [`set_serverless_v2_scaling_configuration(Option<ServerlessV2ScalingConfiguration>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_serverless_v2_scaling_configuration): <p>Contains the scaling configuration of an Aurora Serverless v2 DB cluster.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-serverless-v2.html">Using Amazon Aurora Serverless v2</a> in the <i>Amazon Aurora User Guide</i>.</p>
    ///   - [`network_type(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::network_type) / [`set_network_type(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_network_type): <p>The network type of the DB cluster.</p>  <p>Valid values:</p>  <ul>   <li> <p> <code>IPV4</code> </p> </li>   <li> <p> <code>DUAL</code> </p> </li>  </ul>  <p>The network type is determined by the <code>DBSubnetGroup</code> specified for the DB cluster. A <code>DBSubnetGroup</code> can support only the IPv4 protocol or the IPv4 and the IPv6 protocols (<code>DUAL</code>).</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_VPC.WorkingWithRDSInstanceinaVPC.html"> Working with a DB instance in a VPC</a> in the <i>Amazon Aurora User Guide.</i> </p>
    ///   - [`manage_master_user_password(bool)`](crate::client::fluent_builders::RestoreDBClusterFromS3::manage_master_user_password) / [`set_manage_master_user_password(Option<bool>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_manage_master_user_password): <p>A value that indicates whether to manage the master user password with Amazon Web Services Secrets Manager.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-secrets-manager.html">Password management with Amazon Web Services Secrets Manager</a> in the <i>Amazon RDS User Guide</i> and <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/rds-secrets-manager.html">Password management with Amazon Web Services Secrets Manager</a> in the <i>Amazon Aurora User Guide.</i> </p>  <p>Constraints:</p>  <ul>   <li> <p>Can't manage the master user password with Amazon Web Services Secrets Manager if <code>MasterUserPassword</code> is specified.</p> </li>  </ul>
    ///   - [`master_user_secret_kms_key_id(impl Into<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::master_user_secret_kms_key_id) / [`set_master_user_secret_kms_key_id(Option<String>)`](crate::client::fluent_builders::RestoreDBClusterFromS3::set_master_user_secret_kms_key_id): <p>The Amazon Web Services KMS key identifier to encrypt a secret that is automatically generated and managed in Amazon Web Services Secrets Manager.</p>  <p>This setting is valid only if the master user password is managed by RDS in Amazon Web Services Secrets Manager for the DB cluster.</p>  <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN.</p>  <p>If you don't specify <code>MasterUserSecretKmsKeyId</code>, then the <code>aws/secretsmanager</code> KMS key is used to encrypt the secret. If the secret is in a different Amazon Web Services account, then you can't use the <code>aws/secretsmanager</code> KMS key to encrypt the secret, and you must use a customer managed KMS key.</p>  <p>There is a default KMS key for your Amazon Web Services account. Your Amazon Web Services account has a different default KMS key for each Amazon Web Services Region.</p>
                            /// - On success, responds with [`RestoreDbClusterFromS3Output`](crate::output::RestoreDbClusterFromS3Output) with field(s):
    ///   - [`db_cluster(Option<DbCluster>)`](crate::output::RestoreDbClusterFromS3Output::db_cluster): <p>Contains the details of an Amazon Aurora DB cluster or Multi-AZ DB cluster.</p>  <p>For an Amazon Aurora DB cluster, this data type is used as a response element in the operations <code>CreateDBCluster</code>, <code>DeleteDBCluster</code>, <code>DescribeDBClusters</code>, <code>FailoverDBCluster</code>, <code>ModifyDBCluster</code>, <code>PromoteReadReplicaDBCluster</code>, <code>RestoreDBClusterFromS3</code>, <code>RestoreDBClusterFromSnapshot</code>, <code>RestoreDBClusterToPointInTime</code>, <code>StartDBCluster</code>, and <code>StopDBCluster</code>.</p>  <p>For a Multi-AZ DB cluster, this data type is used as a response element in the operations <code>CreateDBCluster</code>, <code>DeleteDBCluster</code>, <code>DescribeDBClusters</code>, <code>FailoverDBCluster</code>, <code>ModifyDBCluster</code>, <code>RebootDBCluster</code>, <code>RestoreDBClusterFromSnapshot</code>, and <code>RestoreDBClusterToPointInTime</code>.</p>  <p>For more information on Amazon Aurora DB clusters, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/CHAP_AuroraOverview.html"> What is Amazon Aurora?</a> in the <i>Amazon Aurora User Guide.</i> </p>  <p>For more information on Multi-AZ DB clusters, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/multi-az-db-clusters-concepts.html"> Multi-AZ deployments with two readable standby DB instances</a> in the <i>Amazon RDS User Guide.</i> </p>
                            /// - On failure, responds with [`SdkError<RestoreDBClusterFromS3Error>`](crate::error::RestoreDBClusterFromS3Error)
    pub fn restore_db_cluster_from_s3(&self) -> crate::client::fluent_builders::RestoreDBClusterFromS3 {
                                crate::client::fluent_builders::RestoreDBClusterFromS3::new(self.handle.clone())
                            }
}

