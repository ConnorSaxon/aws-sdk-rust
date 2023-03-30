// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyDBInstance`](crate::client::fluent_builders::ModifyDBInstance) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_instance_identifier(impl Into<String>)`](crate::client::fluent_builders::ModifyDBInstance::db_instance_identifier) / [`set_db_instance_identifier(Option<String>)`](crate::client::fluent_builders::ModifyDBInstance::set_db_instance_identifier): <p>The instance identifier. This value is stored as a lowercase string.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must match the identifier of an existing <code>DBInstance</code>.</p> </li>  </ul>
    ///   - [`db_instance_class(impl Into<String>)`](crate::client::fluent_builders::ModifyDBInstance::db_instance_class) / [`set_db_instance_class(Option<String>)`](crate::client::fluent_builders::ModifyDBInstance::set_db_instance_class): <p>The new compute and memory capacity of the instance; for example, <code>db.r5.large</code>. Not all instance classes are available in all Amazon Web Services Regions. </p>  <p>If you modify the instance class, an outage occurs during the change. The change is applied during the next maintenance window, unless <code>ApplyImmediately</code> is specified as <code>true</code> for this request. </p>  <p>Default: Uses existing setting.</p>
    ///   - [`apply_immediately(bool)`](crate::client::fluent_builders::ModifyDBInstance::apply_immediately) / [`set_apply_immediately(bool)`](crate::client::fluent_builders::ModifyDBInstance::set_apply_immediately): <p>Specifies whether the modifications in this request and any pending modifications are asynchronously applied as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the instance. </p>  <p> If this parameter is set to <code>false</code>, changes to the instance are applied during the next maintenance window. Some parameter changes can cause an outage and are applied on the next reboot.</p>  <p>Default: <code>false</code> </p>
    ///   - [`preferred_maintenance_window(impl Into<String>)`](crate::client::fluent_builders::ModifyDBInstance::preferred_maintenance_window) / [`set_preferred_maintenance_window(Option<String>)`](crate::client::fluent_builders::ModifyDBInstance::set_preferred_maintenance_window): <p>The weekly time range (in UTC) during which system maintenance can occur, which might result in an outage. Changing this parameter doesn't result in an outage except in the following situation, and the change is asynchronously applied as soon as possible. If there are pending actions that cause a reboot, and the maintenance window is changed to include the current time, changing this parameter causes a reboot of the instance. If you are moving this window to the current time, there must be at least 30 minutes between the current time and end of the window to ensure that pending changes are applied.</p>  <p>Default: Uses existing setting.</p>  <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p>  <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p>  <p>Constraints: Must be at least 30 minutes.</p>
    ///   - [`auto_minor_version_upgrade(bool)`](crate::client::fluent_builders::ModifyDBInstance::auto_minor_version_upgrade) / [`set_auto_minor_version_upgrade(Option<bool>)`](crate::client::fluent_builders::ModifyDBInstance::set_auto_minor_version_upgrade): <p>This parameter does not apply to Amazon DocumentDB. Amazon DocumentDB does not perform minor version upgrades regardless of the value set.</p>
    ///   - [`new_db_instance_identifier(impl Into<String>)`](crate::client::fluent_builders::ModifyDBInstance::new_db_instance_identifier) / [`set_new_db_instance_identifier(Option<String>)`](crate::client::fluent_builders::ModifyDBInstance::set_new_db_instance_identifier): <p> The new instance identifier for the instance when renaming an instance. When you change the instance identifier, an instance reboot occurs immediately if you set <code>Apply Immediately</code> to <code>true</code>. It occurs during the next maintenance window if you set <code>Apply Immediately</code> to <code>false</code>. This value is stored as a lowercase string. </p>  <p>Constraints:</p>  <ul>   <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li>   <li> <p>The first character must be a letter.</p> </li>   <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li>  </ul>  <p>Example: <code>mydbinstance</code> </p>
    ///   - [`ca_certificate_identifier(impl Into<String>)`](crate::client::fluent_builders::ModifyDBInstance::ca_certificate_identifier) / [`set_ca_certificate_identifier(Option<String>)`](crate::client::fluent_builders::ModifyDBInstance::set_ca_certificate_identifier): <p>Indicates the certificate that needs to be associated with the instance.</p>
    ///   - [`copy_tags_to_snapshot(bool)`](crate::client::fluent_builders::ModifyDBInstance::copy_tags_to_snapshot) / [`set_copy_tags_to_snapshot(Option<bool>)`](crate::client::fluent_builders::ModifyDBInstance::set_copy_tags_to_snapshot): <p>A value that indicates whether to copy all tags from the DB instance to snapshots of the DB instance. By default, tags are not copied.</p>
    ///   - [`promotion_tier(i32)`](crate::client::fluent_builders::ModifyDBInstance::promotion_tier) / [`set_promotion_tier(Option<i32>)`](crate::client::fluent_builders::ModifyDBInstance::set_promotion_tier): <p>A value that specifies the order in which an Amazon DocumentDB replica is promoted to the primary instance after a failure of the existing primary instance.</p>  <p>Default: 1</p>  <p>Valid values: 0-15</p>
    ///   - [`enable_performance_insights(bool)`](crate::client::fluent_builders::ModifyDBInstance::enable_performance_insights) / [`set_enable_performance_insights(Option<bool>)`](crate::client::fluent_builders::ModifyDBInstance::set_enable_performance_insights): <p>A value that indicates whether to enable Performance Insights for the DB Instance. For more information, see <a href="https://docs.aws.amazon.com/documentdb/latest/developerguide/performance-insights.html">Using Amazon Performance Insights</a>.</p>
    ///   - [`performance_insights_kms_key_id(impl Into<String>)`](crate::client::fluent_builders::ModifyDBInstance::performance_insights_kms_key_id) / [`set_performance_insights_kms_key_id(Option<String>)`](crate::client::fluent_builders::ModifyDBInstance::set_performance_insights_kms_key_id): <p>The KMS key identifier for encryption of Performance Insights data.</p>  <p>The KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>  <p>If you do not specify a value for PerformanceInsightsKMSKeyId, then Amazon DocumentDB uses your default KMS key. There is a default KMS key for your Amazon Web Services account. Your Amazon Web Services account has a different default KMS key for each Amazon Web Services region.</p>
                            /// - On success, responds with [`ModifyDbInstanceOutput`](crate::output::ModifyDbInstanceOutput) with field(s):
    ///   - [`db_instance(Option<DbInstance>)`](crate::output::ModifyDbInstanceOutput::db_instance): <p>Detailed information about an instance. </p>
                            /// - On failure, responds with [`SdkError<ModifyDBInstanceError>`](crate::error::ModifyDBInstanceError)
    pub fn modify_db_instance(&self) -> crate::client::fluent_builders::ModifyDBInstance {
                                crate::client::fluent_builders::ModifyDBInstance::new(self.handle.clone())
                            }
}

