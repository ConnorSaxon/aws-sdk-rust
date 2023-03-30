// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDatabase`](crate::client::fluent_builders::GetDatabase) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_name(impl Into<String>)`](crate::client::fluent_builders::GetDatabase::catalog_name) / [`set_catalog_name(Option<String>)`](crate::client::fluent_builders::GetDatabase::set_catalog_name): <p>The name of the data catalog that contains the database to return.</p>
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::GetDatabase::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::GetDatabase::set_database_name): <p>The name of the database to return.</p>
                            /// - On success, responds with [`GetDatabaseOutput`](crate::output::GetDatabaseOutput) with field(s):
    ///   - [`database(Option<Database>)`](crate::output::GetDatabaseOutput::database): <p>The database returned.</p>
                            /// - On failure, responds with [`SdkError<GetDatabaseError>`](crate::error::GetDatabaseError)
    pub fn get_database(&self) -> crate::client::fluent_builders::GetDatabase {
                                crate::client::fluent_builders::GetDatabase::new(self.handle.clone())
                            }
}

