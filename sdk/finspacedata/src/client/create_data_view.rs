// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDataView`](crate::client::fluent_builders::CreateDataView) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateDataView::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateDataView::set_client_token): <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    ///   - [`dataset_id(impl Into<String>)`](crate::client::fluent_builders::CreateDataView::dataset_id) / [`set_dataset_id(Option<String>)`](crate::client::fluent_builders::CreateDataView::set_dataset_id): <p>The unique Dataset identifier that is used to create a Dataview.</p>
    ///   - [`auto_update(bool)`](crate::client::fluent_builders::CreateDataView::auto_update) / [`set_auto_update(bool)`](crate::client::fluent_builders::CreateDataView::set_auto_update): <p>Flag to indicate Dataview should be updated automatically.</p>
    ///   - [`sort_columns(Vec<String>)`](crate::client::fluent_builders::CreateDataView::sort_columns) / [`set_sort_columns(Option<Vec<String>>)`](crate::client::fluent_builders::CreateDataView::set_sort_columns): <p>Columns to be used for sorting the data.</p>
    ///   - [`partition_columns(Vec<String>)`](crate::client::fluent_builders::CreateDataView::partition_columns) / [`set_partition_columns(Option<Vec<String>>)`](crate::client::fluent_builders::CreateDataView::set_partition_columns): <p>Ordered set of column names used to partition data.</p>
    ///   - [`as_of_timestamp(i64)`](crate::client::fluent_builders::CreateDataView::as_of_timestamp) / [`set_as_of_timestamp(Option<i64>)`](crate::client::fluent_builders::CreateDataView::set_as_of_timestamp): <p>Beginning time to use for the Dataview. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p>
    ///   - [`destination_type_params(DataViewDestinationTypeParams)`](crate::client::fluent_builders::CreateDataView::destination_type_params) / [`set_destination_type_params(Option<DataViewDestinationTypeParams>)`](crate::client::fluent_builders::CreateDataView::set_destination_type_params): <p>Options that define the destination type for the Dataview.</p>
                            /// - On success, responds with [`CreateDataViewOutput`](crate::output::CreateDataViewOutput) with field(s):
    ///   - [`dataset_id(Option<String>)`](crate::output::CreateDataViewOutput::dataset_id): <p>The unique identifier of the Dataset used for the Dataview.</p>
    ///   - [`data_view_id(Option<String>)`](crate::output::CreateDataViewOutput::data_view_id): <p>The unique identifier for the created Dataview.</p>
                            /// - On failure, responds with [`SdkError<CreateDataViewError>`](crate::error::CreateDataViewError)
    pub fn create_data_view(&self) -> crate::client::fluent_builders::CreateDataView {
                                crate::client::fluent_builders::CreateDataView::new(self.handle.clone())
                            }
}

