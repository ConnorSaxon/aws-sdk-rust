// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDatasets`](crate::client::fluent_builders::ListDatasets) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity_pool_id(impl Into<String>)`](crate::client::fluent_builders::ListDatasets::identity_pool_id) / [`set_identity_pool_id(Option<String>)`](crate::client::fluent_builders::ListDatasets::set_identity_pool_id): A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    ///   - [`identity_id(impl Into<String>)`](crate::client::fluent_builders::ListDatasets::identity_id) / [`set_identity_id(Option<String>)`](crate::client::fluent_builders::ListDatasets::set_identity_id): A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDatasets::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDatasets::set_next_token): A pagination token for obtaining the next page of results.
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListDatasets::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListDatasets::set_max_results): The maximum number of results to be returned.
                            /// - On success, responds with [`ListDatasetsOutput`](crate::output::ListDatasetsOutput) with field(s):
    ///   - [`datasets(Option<Vec<Dataset>>)`](crate::output::ListDatasetsOutput::datasets): A set of datasets.
    ///   - [`count(i32)`](crate::output::ListDatasetsOutput::count): Number of datasets returned.
    ///   - [`next_token(Option<String>)`](crate::output::ListDatasetsOutput::next_token): A pagination token for obtaining the next page of results.
                            /// - On failure, responds with [`SdkError<ListDatasetsError>`](crate::error::ListDatasetsError)
    pub fn list_datasets(&self) -> crate::client::fluent_builders::ListDatasets {
                                crate::client::fluent_builders::ListDatasets::new(self.handle.clone())
                            }
}

