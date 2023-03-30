// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDataset`](crate::client::fluent_builders::CreateDataset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dataset_name(impl Into<String>)`](crate::client::fluent_builders::CreateDataset::dataset_name) / [`set_dataset_name(Option<String>)`](crate::client::fluent_builders::CreateDataset::set_dataset_name): <p>The name of the dataset being created. </p>
    ///   - [`dataset_schema(DatasetSchema)`](crate::client::fluent_builders::CreateDataset::dataset_schema) / [`set_dataset_schema(Option<DatasetSchema>)`](crate::client::fluent_builders::CreateDataset::set_dataset_schema): <p>A JSON description of the data that is in each time series dataset, including names, column names, and data types. </p>
    ///   - [`server_side_kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CreateDataset::server_side_kms_key_id) / [`set_server_side_kms_key_id(Option<String>)`](crate::client::fluent_builders::CreateDataset::set_server_side_kms_key_id): <p>Provides the identifier of the KMS key used to encrypt dataset data by Amazon Lookout for Equipment. </p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateDataset::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateDataset::set_client_token): <p> A unique identifier for the request. If you do not set the client request token, Amazon Lookout for Equipment generates one. </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateDataset::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateDataset::set_tags): <p>Any tags associated with the ingested data described in the dataset. </p>
                            /// - On success, responds with [`CreateDatasetOutput`](crate::output::CreateDatasetOutput) with field(s):
    ///   - [`dataset_name(Option<String>)`](crate::output::CreateDatasetOutput::dataset_name): <p>The name of the dataset being created. </p>
    ///   - [`dataset_arn(Option<String>)`](crate::output::CreateDatasetOutput::dataset_arn): <p> The Amazon Resource Name (ARN) of the dataset being created. </p>
    ///   - [`status(Option<DatasetStatus>)`](crate::output::CreateDatasetOutput::status): <p>Indicates the status of the <code>CreateDataset</code> operation. </p>
                            /// - On failure, responds with [`SdkError<CreateDatasetError>`](crate::error::CreateDatasetError)
    pub fn create_dataset(&self) -> crate::client::fluent_builders::CreateDataset {
                                crate::client::fluent_builders::CreateDataset::new(self.handle.clone())
                            }
}

