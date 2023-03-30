// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchDeleteTable`](crate::client::fluent_builders::BatchDeleteTable) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::BatchDeleteTable::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::BatchDeleteTable::set_catalog_id): <p>The ID of the Data Catalog where the table resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::BatchDeleteTable::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::BatchDeleteTable::set_database_name): <p>The name of the catalog database in which the tables to delete reside. For Hive compatibility, this name is entirely lowercase.</p>
    ///   - [`tables_to_delete(Vec<String>)`](crate::client::fluent_builders::BatchDeleteTable::tables_to_delete) / [`set_tables_to_delete(Option<Vec<String>>)`](crate::client::fluent_builders::BatchDeleteTable::set_tables_to_delete): <p>A list of the table to delete.</p>
    ///   - [`transaction_id(impl Into<String>)`](crate::client::fluent_builders::BatchDeleteTable::transaction_id) / [`set_transaction_id(Option<String>)`](crate::client::fluent_builders::BatchDeleteTable::set_transaction_id): <p>The transaction ID at which to delete the table contents.</p>
                            /// - On success, responds with [`BatchDeleteTableOutput`](crate::output::BatchDeleteTableOutput) with field(s):
    ///   - [`errors(Option<Vec<TableError>>)`](crate::output::BatchDeleteTableOutput::errors): <p>A list of errors encountered in attempting to delete the specified tables.</p>
                            /// - On failure, responds with [`SdkError<BatchDeleteTableError>`](crate::error::BatchDeleteTableError)
    pub fn batch_delete_table(&self) -> crate::client::fluent_builders::BatchDeleteTable {
                                crate::client::fluent_builders::BatchDeleteTable::new(self.handle.clone())
                            }
}

