// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDBInstanceReadReplica`](crate::client::fluent_builders::CreateDBInstanceReadReplica) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_instance_identifier(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::db_instance_identifier) / [`set_db_instance_identifier(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_db_instance_identifier): <p>The DB instance identifier of the read replica. This identifier is the unique key that identifies a DB instance. This parameter is stored as a lowercase string.</p>
    ///   - [`source_db_instance_identifier(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::source_db_instance_identifier) / [`set_source_db_instance_identifier(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_source_db_instance_identifier): <p>The identifier of the DB instance that will act as the source for the read replica. Each DB instance can have up to five read replicas.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must be the identifier of an existing MySQL, MariaDB, Oracle, PostgreSQL, or SQL Server DB instance.</p> </li>   <li> <p>Can specify a DB instance that is a MySQL read replica only if the source is running MySQL 5.6 or later.</p> </li>   <li> <p>For the limitations of Oracle read replicas, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/oracle-read-replicas.html">Read Replica Limitations with Oracle</a> in the <i>Amazon RDS User Guide</i>.</p> </li>   <li> <p>For the limitations of SQL Server read replicas, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/SQLServer.ReadReplicas.Limitations.html">Read Replica Limitations with Microsoft SQL Server</a> in the <i>Amazon RDS User Guide</i>.</p> </li>   <li> <p>Can specify a PostgreSQL DB instance only if the source is running PostgreSQL 9.3.5 or later (9.4.7 and higher for cross-Region replication).</p> </li>   <li> <p>The specified DB instance must have automatic backups enabled, that is, its backup retention period must be greater than 0.</p> </li>   <li> <p>If the source DB instance is in the same Amazon Web Services Region as the read replica, specify a valid DB instance identifier.</p> </li>   <li> <p>If the source DB instance is in a different Amazon Web Services Region from the read replica, specify a valid DB instance ARN. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.ARN.html#USER_Tagging.ARN.Constructing">Constructing an ARN for Amazon RDS</a> in the <i>Amazon RDS User Guide</i>. This doesn't apply to SQL Server or RDS Custom, which don't support cross-Region replicas.</p> </li>  </ul>
    ///   - [`db_instance_class(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::db_instance_class) / [`set_db_instance_class(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_db_instance_class): <p>The compute and memory capacity of the read replica, for example db.m4.large. Not all DB instance classes are available in all Amazon Web Services Regions, or for all database engines. For the full list of DB instance classes, and availability for your engine, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html">DB Instance Class</a> in the <i>Amazon RDS User Guide</i>.</p>  <p>Default: Inherits from the source DB instance.</p>
    ///   - [`availability_zone(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::availability_zone) / [`set_availability_zone(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_availability_zone): <p>The Availability Zone (AZ) where the read replica will be created.</p>  <p>Default: A random, system-chosen Availability Zone in the endpoint's Amazon Web Services Region.</p>  <p>Example: <code>us-east-1d</code> </p>
    ///   - [`port(i32)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::port) / [`set_port(Option<i32>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_port): <p>The port number that the DB instance uses for connections.</p>  <p>Default: Inherits from the source DB instance</p>  <p>Valid Values: <code>1150-65535</code> </p>
    ///   - [`multi_az(bool)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::multi_az) / [`set_multi_az(Option<bool>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_multi_az): <p>A value that indicates whether the read replica is in a Multi-AZ deployment.</p>  <p>You can create a read replica as a Multi-AZ DB instance. RDS creates a standby of your replica in another Availability Zone for failover support for the replica. Creating your read replica as a Multi-AZ DB instance is independent of whether the source database is a Multi-AZ DB instance.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`auto_minor_version_upgrade(bool)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::auto_minor_version_upgrade) / [`set_auto_minor_version_upgrade(Option<bool>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_auto_minor_version_upgrade): <p>A value that indicates whether minor engine upgrades are applied automatically to the read replica during the maintenance window.</p>  <p>This setting doesn't apply to RDS Custom.</p>  <p>Default: Inherits from the source DB instance</p>
    ///   - [`iops(i32)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::iops) / [`set_iops(Option<i32>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_iops): <p>The amount of Provisioned IOPS (input/output operations per second) to be initially allocated for the DB instance.</p>
    ///   - [`option_group_name(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::option_group_name) / [`set_option_group_name(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_option_group_name): <p>The option group the DB instance is associated with. If omitted, the option group associated with the source instance is used.</p> <note>   <p>For SQL Server, you must use the option group associated with the source instance.</p>  </note>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`db_parameter_group_name(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::db_parameter_group_name) / [`set_db_parameter_group_name(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_db_parameter_group_name): <p>The name of the DB parameter group to associate with this DB instance.</p>  <p>If you do not specify a value for <code>DBParameterGroupName</code>, then Amazon RDS uses the <code>DBParameterGroup</code> of source DB instance for a same Region read replica, or the default <code>DBParameterGroup</code> for the specified DB engine for a cross-Region read replica.</p>  <p>Specifying a parameter group for this operation is only supported for MySQL and Oracle DB instances. It isn't supported for RDS Custom.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must be 1 to 255 letters, numbers, or hyphens.</p> </li>   <li> <p>First character must be a letter</p> </li>   <li> <p>Can't end with a hyphen or contain two consecutive hyphens</p> </li>  </ul>
    ///   - [`publicly_accessible(bool)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::publicly_accessible) / [`set_publicly_accessible(Option<bool>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_publicly_accessible): <p>A value that indicates whether the DB instance is publicly accessible.</p>  <p>When the DB cluster is publicly accessible, its Domain Name System (DNS) endpoint resolves to the private IP address from within the DB cluster's virtual private cloud (VPC). It resolves to the public IP address from outside of the DB cluster's VPC. Access to the DB cluster is ultimately controlled by the security group it uses. That public access isn't permitted if the security group assigned to the DB cluster doesn't permit it.</p>  <p>When the DB instance isn't publicly accessible, it is an internal DB instance with a DNS name that resolves to a private IP address.</p>  <p>For more information, see <code>CreateDBInstance</code>.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_tags): <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i> </p>
    ///   - [`db_subnet_group_name(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::db_subnet_group_name) / [`set_db_subnet_group_name(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_db_subnet_group_name): <p>Specifies a DB subnet group for the DB instance. The new DB instance is created in the VPC associated with the DB subnet group. If no DB subnet group is specified, then the new DB instance isn't created in a VPC.</p>  <p>Constraints:</p>  <ul>   <li> <p>Can only be specified if the source DB instance identifier specifies a DB instance in another Amazon Web Services Region.</p> </li>   <li> <p>If supplied, must match the name of an existing DBSubnetGroup.</p> </li>   <li> <p>The specified DB subnet group must be in the same Amazon Web Services Region in which the operation is running.</p> </li>   <li> <p>All read replicas in one Amazon Web Services Region that are created from the same source DB instance must either:&gt;</p>    <ul>     <li> <p>Specify DB subnet groups from the same VPC. All these read replicas are created in the same VPC.</p> </li>     <li> <p>Not specify a DB subnet group. All these read replicas are created outside of any VPC.</p> </li>    </ul> </li>  </ul>  <p>Example: <code>mydbsubnetgroup</code> </p>
    ///   - [`vpc_security_group_ids(Vec<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::vpc_security_group_ids) / [`set_vpc_security_group_ids(Option<Vec<String>>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_vpc_security_group_ids): <p>A list of Amazon EC2 VPC security groups to associate with the read replica.</p>  <p>This setting doesn't apply to RDS Custom.</p>  <p>Default: The default EC2 VPC security group for the DB subnet group's VPC.</p>
    ///   - [`storage_type(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::storage_type) / [`set_storage_type(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_storage_type): <p>Specifies the storage type to be associated with the read replica.</p>  <p>Valid values: <code>gp2 | gp3 | io1 | standard</code> </p>  <p>If you specify <code>io1</code> or <code>gp3</code>, you must also include a value for the <code>Iops</code> parameter.</p>  <p>Default: <code>io1</code> if the <code>Iops</code> parameter is specified, otherwise <code>gp2</code> </p>
    ///   - [`copy_tags_to_snapshot(bool)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::copy_tags_to_snapshot) / [`set_copy_tags_to_snapshot(Option<bool>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_copy_tags_to_snapshot): <p>A value that indicates whether to copy all tags from the read replica to snapshots of the read replica. By default, tags are not copied.</p>
    ///   - [`monitoring_interval(i32)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::monitoring_interval) / [`set_monitoring_interval(Option<i32>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_monitoring_interval): <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the read replica. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0.</p>  <p>If <code>MonitoringRoleArn</code> is specified, then you must also set <code>MonitoringInterval</code> to a value other than 0.</p>  <p>This setting doesn't apply to RDS Custom.</p>  <p>Valid Values: <code>0, 1, 5, 10, 15, 30, 60</code> </p>
    ///   - [`monitoring_role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::monitoring_role_arn) / [`set_monitoring_role_arn(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_monitoring_role_arn): <p>The ARN for the IAM role that permits RDS to send enhanced monitoring metrics to Amazon CloudWatch Logs. For example, <code>arn:aws:iam:123456789012:role/emaccess</code>. For information on creating a monitoring role, go to <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Monitoring.html#USER_Monitoring.OS.IAMRole">To create an IAM role for Amazon RDS Enhanced Monitoring</a> in the <i>Amazon RDS User Guide</i>.</p>  <p>If <code>MonitoringInterval</code> is set to a value other than 0, then you must supply a <code>MonitoringRoleArn</code> value.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_kms_key_id): <p>The Amazon Web Services KMS key identifier for an encrypted read replica.</p>  <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>  <p>If you create an encrypted read replica in the same Amazon Web Services Region as the source DB instance, then do not specify a value for this parameter. A read replica in the same Amazon Web Services Region is always encrypted with the same KMS key as the source DB instance.</p>  <p>If you create an encrypted read replica in a different Amazon Web Services Region, then you must specify a KMS key identifier for the destination Amazon Web Services Region. KMS keys are specific to the Amazon Web Services Region that they are created in, and you can't use KMS keys from one Amazon Web Services Region in another Amazon Web Services Region.</p>  <p>You can't create an encrypted read replica from an unencrypted DB instance.</p>  <p>This setting doesn't apply to RDS Custom, which uses the same KMS key as the primary replica.</p>
    ///   - [`pre_signed_url(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::pre_signed_url) / [`set_pre_signed_url(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_pre_signed_url): <p>When you are creating a read replica from one Amazon Web Services GovCloud (US) Region to another or from one China Amazon Web Services Region to another, the URL that contains a Signature Version 4 signed request for the <code>CreateDBInstanceReadReplica</code> API operation in the source Amazon Web Services Region that contains the source DB instance.</p>  <p>This setting applies only to Amazon Web Services GovCloud (US) Regions and China Amazon Web Services Regions. It's ignored in other Amazon Web Services Regions.</p>  <p>You must specify this parameter when you create an encrypted read replica from another Amazon Web Services Region by using the Amazon RDS API. Don't specify <code>PreSignedUrl</code> when you are creating an encrypted read replica in the same Amazon Web Services Region.</p>  <p>The presigned URL must be a valid request for the <code>CreateDBInstanceReadReplica</code> API operation that can run in the source Amazon Web Services Region that contains the encrypted source DB instance. The presigned URL request must contain the following parameter values:</p>  <ul>   <li> <p> <code>DestinationRegion</code> - The Amazon Web Services Region that the encrypted read replica is created in. This Amazon Web Services Region is the same one where the <code>CreateDBInstanceReadReplica</code> operation is called that contains this presigned URL.</p> <p>For example, if you create an encrypted DB instance in the us-west-1 Amazon Web Services Region, from a source DB instance in the us-east-2 Amazon Web Services Region, then you call the <code>CreateDBInstanceReadReplica</code> operation in the us-east-1 Amazon Web Services Region and provide a presigned URL that contains a call to the <code>CreateDBInstanceReadReplica</code> operation in the us-west-2 Amazon Web Services Region. For this example, the <code>DestinationRegion</code> in the presigned URL must be set to the us-east-1 Amazon Web Services Region.</p> </li>   <li> <p> <code>KmsKeyId</code> - The KMS key identifier for the key to use to encrypt the read replica in the destination Amazon Web Services Region. This is the same identifier for both the <code>CreateDBInstanceReadReplica</code> operation that is called in the destination Amazon Web Services Region, and the operation contained in the presigned URL.</p> </li>   <li> <p> <code>SourceDBInstanceIdentifier</code> - The DB instance identifier for the encrypted DB instance to be replicated. This identifier must be in the Amazon Resource Name (ARN) format for the source Amazon Web Services Region. For example, if you are creating an encrypted read replica from a DB instance in the us-west-2 Amazon Web Services Region, then your <code>SourceDBInstanceIdentifier</code> looks like the following example: <code>arn:aws:rds:us-west-2:123456789012:instance:mysql-instance1-20161115</code>.</p> </li>  </ul>  <p>To learn how to generate a Signature Version 4 signed request, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html">Authenticating Requests: Using Query Parameters (Amazon Web Services Signature Version 4)</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 Signing Process</a>.</p> <note>   <p>If you are using an Amazon Web Services SDK tool or the CLI, you can specify <code>SourceRegion</code> (or <code>--source-region</code> for the CLI) instead of specifying <code>PreSignedUrl</code> manually. Specifying <code>SourceRegion</code> autogenerates a presigned URL that is a valid request for the operation that can run in the source Amazon Web Services Region.</p>   <p> <code>SourceRegion</code> isn't supported for SQL Server, because Amazon RDS for SQL Server doesn't support cross-Region read replicas.</p>  </note>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`enable_iam_database_authentication(bool)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::enable_iam_database_authentication) / [`set_enable_iam_database_authentication(Option<bool>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_enable_iam_database_authentication): <p>A value that indicates whether to enable mapping of Amazon Web Services Identity and Access Management (IAM) accounts to database accounts. By default, mapping isn't enabled.</p>  <p>For more information about IAM database authentication, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.IAMDBAuth.html"> IAM Database Authentication for MySQL and PostgreSQL</a> in the <i>Amazon RDS User Guide</i>.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`enable_performance_insights(bool)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::enable_performance_insights) / [`set_enable_performance_insights(Option<bool>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_enable_performance_insights): <p>A value that indicates whether to enable Performance Insights for the read replica.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.html">Using Amazon Performance Insights</a> in the <i>Amazon RDS User Guide</i>.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`performance_insights_kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::performance_insights_kms_key_id) / [`set_performance_insights_kms_key_id(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_performance_insights_kms_key_id): <p>The Amazon Web Services KMS key identifier for encryption of Performance Insights data.</p>  <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>  <p>If you do not specify a value for <code>PerformanceInsightsKMSKeyId</code>, then Amazon RDS uses your default KMS key. There is a default KMS key for your Amazon Web Services account. Your Amazon Web Services account has a different default KMS key for each Amazon Web Services Region.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`performance_insights_retention_period(i32)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::performance_insights_retention_period) / [`set_performance_insights_retention_period(Option<i32>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_performance_insights_retention_period): <p>The number of days to retain Performance Insights data. The default is 7 days. The following values are valid:</p>  <ul>   <li> <p>7</p> </li>   <li> <p> <i>month</i> * 31, where <i>month</i> is a number of months from 1-23</p> </li>   <li> <p>731</p> </li>  </ul>  <p>For example, the following values are valid:</p>  <ul>   <li> <p>93 (3 months * 31)</p> </li>   <li> <p>341 (11 months * 31)</p> </li>   <li> <p>589 (19 months * 31)</p> </li>   <li> <p>731</p> </li>  </ul>  <p>If you specify a retention period such as 94, which isn't a valid value, RDS issues an error.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`enable_cloudwatch_logs_exports(Vec<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::enable_cloudwatch_logs_exports) / [`set_enable_cloudwatch_logs_exports(Option<Vec<String>>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_enable_cloudwatch_logs_exports): <p>The list of logs that the new DB instance is to export to CloudWatch Logs. The values in the list depend on the DB engine being used. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_LogAccess.html#USER_LogAccess.Procedural.UploadtoCloudWatch">Publishing Database Logs to Amazon CloudWatch Logs </a> in the <i>Amazon RDS User Guide</i>.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`processor_features(Vec<ProcessorFeature>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::processor_features) / [`set_processor_features(Option<Vec<ProcessorFeature>>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_processor_features): <p>The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`use_default_processor_features(bool)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::use_default_processor_features) / [`set_use_default_processor_features(Option<bool>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_use_default_processor_features): <p>A value that indicates whether the DB instance class of the DB instance uses its default processor features.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`deletion_protection(bool)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::deletion_protection) / [`set_deletion_protection(Option<bool>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_deletion_protection): <p>A value that indicates whether the DB instance has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection isn't enabled. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_DeleteInstance.html"> Deleting a DB Instance</a>.</p>
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_domain): <p>The Active Directory directory ID to create the DB instance in. Currently, only MySQL, Microsoft SQL Server, Oracle, and PostgreSQL DB instances can be created in an Active Directory Domain.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/kerberos-authentication.html"> Kerberos Authentication</a> in the <i>Amazon RDS User Guide</i>.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`domain_iam_role_name(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::domain_iam_role_name) / [`set_domain_iam_role_name(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_domain_iam_role_name): <p>Specify the name of the IAM role to be used when making API calls to the Directory Service.</p>  <p>This setting doesn't apply to RDS Custom.</p>
    ///   - [`replica_mode(ReplicaMode)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::replica_mode) / [`set_replica_mode(Option<ReplicaMode>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_replica_mode): <p>The open mode of the replica database: mounted or read-only.</p> <note>   <p>This parameter is only supported for Oracle DB instances.</p>  </note>  <p>Mounted DB replicas are included in Oracle Database Enterprise Edition. The main use case for mounted replicas is cross-Region disaster recovery. The primary database doesn't use Active Data Guard to transmit information to the mounted replica. Because it doesn't accept user connections, a mounted replica can't serve a read-only workload.</p>  <p>You can create a combination of mounted and read-only DB replicas for the same primary DB instance. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/oracle-read-replicas.html">Working with Oracle Read Replicas for Amazon RDS</a> in the <i>Amazon RDS User Guide</i>.</p>  <p>For RDS Custom, you must specify this parameter and set it to <code>mounted</code>. The value won't be set by default. After replica creation, you can manage the open mode manually.</p>
    ///   - [`max_allocated_storage(i32)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::max_allocated_storage) / [`set_max_allocated_storage(Option<i32>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_max_allocated_storage): <p>The upper limit in gibibytes (GiB) to which Amazon RDS can automatically scale the storage of the DB instance.</p>  <p>For more information about this setting, including limitations that apply to it, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PIOPS.StorageTypes.html#USER_PIOPS.Autoscaling"> Managing capacity automatically with Amazon RDS storage autoscaling</a> in the <i>Amazon RDS User Guide</i>.</p>
    ///   - [`custom_iam_instance_profile(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::custom_iam_instance_profile) / [`set_custom_iam_instance_profile(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_custom_iam_instance_profile): <p>The instance profile associated with the underlying Amazon EC2 instance of an RDS Custom DB instance. The instance profile must meet the following requirements:</p>  <ul>   <li> <p>The profile must exist in your account.</p> </li>   <li> <p>The profile must have an IAM role that Amazon EC2 has permissions to assume.</p> </li>   <li> <p>The instance profile name and the associated IAM role name must start with the prefix <code>AWSRDSCustom</code>.</p> </li>  </ul>  <p>For the list of permissions required for the IAM role, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-setup-orcl.html#custom-setup-orcl.iam-vpc"> Configure IAM and your VPC</a> in the <i>Amazon RDS User Guide</i>.</p>  <p>This setting is required for RDS Custom.</p>
    ///   - [`network_type(impl Into<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::network_type) / [`set_network_type(Option<String>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_network_type): <p>The network type of the DB instance.</p>  <p>Valid values:</p>  <ul>   <li> <p> <code>IPV4</code> </p> </li>   <li> <p> <code>DUAL</code> </p> </li>  </ul>  <p>The network type is determined by the <code>DBSubnetGroup</code> specified for read replica. A <code>DBSubnetGroup</code> can support only the IPv4 protocol or the IPv4 and the IPv6 protocols (<code>DUAL</code>).</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_VPC.WorkingWithRDSInstanceinaVPC.html"> Working with a DB instance in a VPC</a> in the <i>Amazon RDS User Guide.</i> </p>
    ///   - [`storage_throughput(i32)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::storage_throughput) / [`set_storage_throughput(Option<i32>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_storage_throughput): <p>Specifies the storage throughput value for the read replica.</p>  <p>This setting doesn't apply to RDS Custom or Amazon Aurora.</p>
    ///   - [`enable_customer_owned_ip(bool)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::enable_customer_owned_ip) / [`set_enable_customer_owned_ip(Option<bool>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_enable_customer_owned_ip): <p>A value that indicates whether to enable a customer-owned IP address (CoIP) for an RDS on Outposts read replica.</p>  <p>A <i>CoIP</i> provides local or external connectivity to resources in your Outpost subnets through your on-premises network. For some use cases, a CoIP can provide lower latency for connections to the read replica from outside of its virtual private cloud (VPC) on your local network.</p>  <p>For more information about RDS on Outposts, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-on-outposts.html">Working with Amazon RDS on Amazon Web Services Outposts</a> in the <i>Amazon RDS User Guide</i>.</p>  <p>For more information about CoIPs, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/routing.html#ip-addressing">Customer-owned IP addresses</a> in the <i>Amazon Web Services Outposts User Guide</i>.</p>
    ///   - [`allocated_storage(i32)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::allocated_storage) / [`set_allocated_storage(Option<i32>)`](crate::client::fluent_builders::CreateDBInstanceReadReplica::set_allocated_storage): <p>The amount of storage (in gibibytes) to allocate initially for the read replica. Follow the allocation rules specified in <code>CreateDBInstance</code>.</p> <note>   <p>Be sure to allocate enough memory for your read replica so that the create operation can succeed. You can also allocate additional memory for future growth.</p>  </note>
                            /// - On success, responds with [`CreateDbInstanceReadReplicaOutput`](crate::output::CreateDbInstanceReadReplicaOutput) with field(s):
    ///   - [`db_instance(Option<DbInstance>)`](crate::output::CreateDbInstanceReadReplicaOutput::db_instance): <p>Contains the details of an Amazon RDS DB instance.</p>  <p>This data type is used as a response element in the operations <code>CreateDBInstance</code>, <code>CreateDBInstanceReadReplica</code>, <code>DeleteDBInstance</code>, <code>DescribeDBInstances</code>, <code>ModifyDBInstance</code>, <code>PromoteReadReplica</code>, <code>RebootDBInstance</code>, <code>RestoreDBInstanceFromDBSnapshot</code>, <code>RestoreDBInstanceFromS3</code>, <code>RestoreDBInstanceToPointInTime</code>, <code>StartDBInstance</code>, and <code>StopDBInstance</code>.</p>
                            /// - On failure, responds with [`SdkError<CreateDBInstanceReadReplicaError>`](crate::error::CreateDBInstanceReadReplicaError)
    pub fn create_db_instance_read_replica(&self) -> crate::client::fluent_builders::CreateDBInstanceReadReplica {
                                crate::client::fluent_builders::CreateDBInstanceReadReplica::new(self.handle.clone())
                            }
}

