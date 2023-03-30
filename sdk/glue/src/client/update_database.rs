// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateDatabase`](crate::client::fluent_builders::UpdateDatabase) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::UpdateDatabase::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::UpdateDatabase::set_catalog_id): <p>The ID of the Data Catalog in which the metadata database resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateDatabase::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateDatabase::set_name): <p>The name of the database to update in the catalog. For Hive compatibility, this is folded to lowercase.</p>
    ///   - [`database_input(DatabaseInput)`](crate::client::fluent_builders::UpdateDatabase::database_input) / [`set_database_input(Option<DatabaseInput>)`](crate::client::fluent_builders::UpdateDatabase::set_database_input): <p>A <code>DatabaseInput</code> object specifying the new definition of the metadata database in the catalog.</p>
                            /// - On success, responds with [`UpdateDatabaseOutput`](crate::output::UpdateDatabaseOutput)
                            /// - On failure, responds with [`SdkError<UpdateDatabaseError>`](crate::error::UpdateDatabaseError)
    pub fn update_database(&self) -> crate::client::fluent_builders::UpdateDatabase {
                                crate::client::fluent_builders::UpdateDatabase::new(self.handle.clone())
                            }
}

