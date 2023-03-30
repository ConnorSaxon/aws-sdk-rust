// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDataSource`](crate::client::fluent_builders::DeleteDataSource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteDataSource::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteDataSource::set_id): <p>The identifier of the data source connector you want to delete.</p>
    ///   - [`index_id(impl Into<String>)`](crate::client::fluent_builders::DeleteDataSource::index_id) / [`set_index_id(Option<String>)`](crate::client::fluent_builders::DeleteDataSource::set_index_id): <p>The identifier of the index used with the data source connector.</p>
                            /// - On success, responds with [`DeleteDataSourceOutput`](crate::output::DeleteDataSourceOutput)
                            /// - On failure, responds with [`SdkError<DeleteDataSourceError>`](crate::error::DeleteDataSourceError)
    pub fn delete_data_source(&self) -> crate::client::fluent_builders::DeleteDataSource {
                                crate::client::fluent_builders::DeleteDataSource::new(self.handle.clone())
                            }
}

