// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteFleetAdvisorDatabases`](crate::client::fluent_builders::DeleteFleetAdvisorDatabases) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`database_ids(Vec<String>)`](crate::client::fluent_builders::DeleteFleetAdvisorDatabases::database_ids) / [`set_database_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteFleetAdvisorDatabases::set_database_ids): <p>The IDs of the Fleet Advisor collector databases to delete.</p>
                            /// - On success, responds with [`DeleteFleetAdvisorDatabasesOutput`](crate::output::DeleteFleetAdvisorDatabasesOutput) with field(s):
    ///   - [`database_ids(Option<Vec<String>>)`](crate::output::DeleteFleetAdvisorDatabasesOutput::database_ids): <p>The IDs of the databases that the operation deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteFleetAdvisorDatabasesError>`](crate::error::DeleteFleetAdvisorDatabasesError)
    pub fn delete_fleet_advisor_databases(&self) -> crate::client::fluent_builders::DeleteFleetAdvisorDatabases {
                                crate::client::fluent_builders::DeleteFleetAdvisorDatabases::new(self.handle.clone())
                            }
}

