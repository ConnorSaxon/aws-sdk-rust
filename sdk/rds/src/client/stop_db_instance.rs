// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopDBInstance`](crate::client::fluent_builders::StopDBInstance) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`db_instance_identifier(impl Into<String>)`](crate::client::fluent_builders::StopDBInstance::db_instance_identifier) / [`set_db_instance_identifier(Option<String>)`](crate::client::fluent_builders::StopDBInstance::set_db_instance_identifier): <p>The user-supplied instance identifier.</p>
    ///   - [`db_snapshot_identifier(impl Into<String>)`](crate::client::fluent_builders::StopDBInstance::db_snapshot_identifier) / [`set_db_snapshot_identifier(Option<String>)`](crate::client::fluent_builders::StopDBInstance::set_db_snapshot_identifier): <p>The user-supplied instance identifier of the DB Snapshot created immediately before the DB instance is stopped.</p>
                            /// - On success, responds with [`StopDbInstanceOutput`](crate::output::StopDbInstanceOutput) with field(s):
    ///   - [`db_instance(Option<DbInstance>)`](crate::output::StopDbInstanceOutput::db_instance): <p>Contains the details of an Amazon RDS DB instance.</p>  <p>This data type is used as a response element in the operations <code>CreateDBInstance</code>, <code>CreateDBInstanceReadReplica</code>, <code>DeleteDBInstance</code>, <code>DescribeDBInstances</code>, <code>ModifyDBInstance</code>, <code>PromoteReadReplica</code>, <code>RebootDBInstance</code>, <code>RestoreDBInstanceFromDBSnapshot</code>, <code>RestoreDBInstanceFromS3</code>, <code>RestoreDBInstanceToPointInTime</code>, <code>StartDBInstance</code>, and <code>StopDBInstance</code>.</p>
                            /// - On failure, responds with [`SdkError<StopDBInstanceError>`](crate::error::StopDBInstanceError)
    pub fn stop_db_instance(&self) -> crate::client::fluent_builders::StopDBInstance {
                                crate::client::fluent_builders::StopDBInstance::new(self.handle.clone())
                            }
}

