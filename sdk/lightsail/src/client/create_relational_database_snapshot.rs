// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateRelationalDatabaseSnapshot`](crate::client::fluent_builders::CreateRelationalDatabaseSnapshot) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`relational_database_name(impl Into<String>)`](crate::client::fluent_builders::CreateRelationalDatabaseSnapshot::relational_database_name) / [`set_relational_database_name(Option<String>)`](crate::client::fluent_builders::CreateRelationalDatabaseSnapshot::set_relational_database_name): <p>The name of the database on which to base your new snapshot.</p>
    ///   - [`relational_database_snapshot_name(impl Into<String>)`](crate::client::fluent_builders::CreateRelationalDatabaseSnapshot::relational_database_snapshot_name) / [`set_relational_database_snapshot_name(Option<String>)`](crate::client::fluent_builders::CreateRelationalDatabaseSnapshot::set_relational_database_snapshot_name): <p>The name for your new database snapshot.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li>   <li> <p>The first and last character must be a letter or number.</p> </li>  </ul>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateRelationalDatabaseSnapshot::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateRelationalDatabaseSnapshot::set_tags): <p>The tag keys and optional values to add to the resource during create.</p>  <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p>
                            /// - On success, responds with [`CreateRelationalDatabaseSnapshotOutput`](crate::output::CreateRelationalDatabaseSnapshotOutput) with field(s):
    ///   - [`operations(Option<Vec<Operation>>)`](crate::output::CreateRelationalDatabaseSnapshotOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
                            /// - On failure, responds with [`SdkError<CreateRelationalDatabaseSnapshotError>`](crate::error::CreateRelationalDatabaseSnapshotError)
    pub fn create_relational_database_snapshot(&self) -> crate::client::fluent_builders::CreateRelationalDatabaseSnapshot {
                                crate::client::fluent_builders::CreateRelationalDatabaseSnapshot::new(self.handle.clone())
                            }
}

