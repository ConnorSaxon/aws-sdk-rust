// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDataCellsFilter`](crate::client::fluent_builders::DeleteDataCellsFilter) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`table_catalog_id(impl Into<String>)`](crate::client::fluent_builders::DeleteDataCellsFilter::table_catalog_id) / [`set_table_catalog_id(Option<String>)`](crate::client::fluent_builders::DeleteDataCellsFilter::set_table_catalog_id): <p>The ID of the catalog to which the table belongs.</p>
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::DeleteDataCellsFilter::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::DeleteDataCellsFilter::set_database_name): <p>A database in the Glue Data Catalog.</p>
    ///   - [`table_name(impl Into<String>)`](crate::client::fluent_builders::DeleteDataCellsFilter::table_name) / [`set_table_name(Option<String>)`](crate::client::fluent_builders::DeleteDataCellsFilter::set_table_name): <p>A table in the database.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteDataCellsFilter::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteDataCellsFilter::set_name): <p>The name given by the user to the data filter cell.</p>
                            /// - On success, responds with [`DeleteDataCellsFilterOutput`](crate::output::DeleteDataCellsFilterOutput)
                            /// - On failure, responds with [`SdkError<DeleteDataCellsFilterError>`](crate::error::DeleteDataCellsFilterError)
    pub fn delete_data_cells_filter(&self) -> crate::client::fluent_builders::DeleteDataCellsFilter {
                                crate::client::fluent_builders::DeleteDataCellsFilter::new(self.handle.clone())
                            }
}

