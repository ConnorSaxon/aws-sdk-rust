// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyReplicationTask`](crate::client::fluent_builders::ModifyReplicationTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`replication_task_arn(impl Into<String>)`](crate::client::fluent_builders::ModifyReplicationTask::replication_task_arn) / [`set_replication_task_arn(Option<String>)`](crate::client::fluent_builders::ModifyReplicationTask::set_replication_task_arn): <p>The Amazon Resource Name (ARN) of the replication task.</p>
    ///   - [`replication_task_identifier(impl Into<String>)`](crate::client::fluent_builders::ModifyReplicationTask::replication_task_identifier) / [`set_replication_task_identifier(Option<String>)`](crate::client::fluent_builders::ModifyReplicationTask::set_replication_task_identifier): <p>The replication task identifier.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must contain 1-255 alphanumeric characters or hyphens.</p> </li>   <li> <p>First character must be a letter.</p> </li>   <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li>  </ul>
    ///   - [`migration_type(MigrationTypeValue)`](crate::client::fluent_builders::ModifyReplicationTask::migration_type) / [`set_migration_type(Option<MigrationTypeValue>)`](crate::client::fluent_builders::ModifyReplicationTask::set_migration_type): <p>The migration type. Valid values: <code>full-load</code> | <code>cdc</code> | <code>full-load-and-cdc</code> </p>
    ///   - [`table_mappings(impl Into<String>)`](crate::client::fluent_builders::ModifyReplicationTask::table_mappings) / [`set_table_mappings(Option<String>)`](crate::client::fluent_builders::ModifyReplicationTask::set_table_mappings): <p>When using the CLI or boto3, provide the path of the JSON file that contains the table mappings. Precede the path with <code>file://</code>. For example, <code>--table-mappings file://mappingfile.json</code>. When working with the DMS API, provide the JSON as the parameter value. </p>
    ///   - [`replication_task_settings(impl Into<String>)`](crate::client::fluent_builders::ModifyReplicationTask::replication_task_settings) / [`set_replication_task_settings(Option<String>)`](crate::client::fluent_builders::ModifyReplicationTask::set_replication_task_settings): <p>JSON file that contains settings for the task, such as task metadata settings.</p>
    ///   - [`cdc_start_time(DateTime)`](crate::client::fluent_builders::ModifyReplicationTask::cdc_start_time) / [`set_cdc_start_time(Option<DateTime>)`](crate::client::fluent_builders::ModifyReplicationTask::set_cdc_start_time): <p>Indicates the start time for a change data capture (CDC) operation. Use either CdcStartTime or CdcStartPosition to specify when you want a CDC operation to start. Specifying both values results in an error.</p>  <p>Timestamp Example: --cdc-start-time “2018-03-08T12:12:12”</p>
    ///   - [`cdc_start_position(impl Into<String>)`](crate::client::fluent_builders::ModifyReplicationTask::cdc_start_position) / [`set_cdc_start_position(Option<String>)`](crate::client::fluent_builders::ModifyReplicationTask::set_cdc_start_position): <p>Indicates when you want a change data capture (CDC) operation to start. Use either CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start. Specifying both values results in an error.</p>  <p> The value can be in date, checkpoint, or LSN/SCN format.</p>  <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p>  <p>Checkpoint Example: --cdc-start-position "checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93"</p>  <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p> <note>   <p>When you use this task setting with a source PostgreSQL database, a logical replication slot should already be created and associated with the source endpoint. You can verify this by setting the <code>slotName</code> extra connection attribute to the name of this logical replication slot. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.PostgreSQL.html#CHAP_Source.PostgreSQL.ConnectionAttrib">Extra Connection Attributes When Using PostgreSQL as a Source for DMS</a>.</p>  </note>
    ///   - [`cdc_stop_position(impl Into<String>)`](crate::client::fluent_builders::ModifyReplicationTask::cdc_stop_position) / [`set_cdc_stop_position(Option<String>)`](crate::client::fluent_builders::ModifyReplicationTask::set_cdc_stop_position): <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p>  <p>Server time example: --cdc-stop-position “server_time:2018-02-09T12:12:12”</p>  <p>Commit time example: --cdc-stop-position “commit_time: 2018-02-09T12:12:12 “</p>
    ///   - [`task_data(impl Into<String>)`](crate::client::fluent_builders::ModifyReplicationTask::task_data) / [`set_task_data(Option<String>)`](crate::client::fluent_builders::ModifyReplicationTask::set_task_data): <p>Supplemental information that the task requires to migrate the data for certain source and target endpoints. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.TaskData.html">Specifying Supplemental Data for Task Settings</a> in the <i>Database Migration Service User Guide.</i> </p>
                            /// - On success, responds with [`ModifyReplicationTaskOutput`](crate::output::ModifyReplicationTaskOutput) with field(s):
    ///   - [`replication_task(Option<ReplicationTask>)`](crate::output::ModifyReplicationTaskOutput::replication_task): <p>The replication task that was modified.</p>
                            /// - On failure, responds with [`SdkError<ModifyReplicationTaskError>`](crate::error::ModifyReplicationTaskError)
    pub fn modify_replication_task(&self) -> crate::client::fluent_builders::ModifyReplicationTask {
                                crate::client::fluent_builders::ModifyReplicationTask::new(self.handle.clone())
                            }
}

