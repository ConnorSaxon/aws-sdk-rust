// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDataset`](crate::client::fluent_builders::DeleteDataset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dataset_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteDataset::dataset_arn) / [`set_dataset_arn(Option<String>)`](crate::client::fluent_builders::DeleteDataset::set_dataset_arn): <p> The ARN of the Amazon Rekognition Custom Labels dataset that you want to delete. </p>
                            /// - On success, responds with [`DeleteDatasetOutput`](crate::output::DeleteDatasetOutput)
                            /// - On failure, responds with [`SdkError<DeleteDatasetError>`](crate::error::DeleteDatasetError)
    pub fn delete_dataset(&self) -> crate::client::fluent_builders::DeleteDataset {
                                crate::client::fluent_builders::DeleteDataset::new(self.handle.clone())
                            }
}

