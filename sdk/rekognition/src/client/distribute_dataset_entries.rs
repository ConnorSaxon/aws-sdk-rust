// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DistributeDatasetEntries`](crate::client::fluent_builders::DistributeDatasetEntries) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`datasets(Vec<DistributeDataset>)`](crate::client::fluent_builders::DistributeDatasetEntries::datasets) / [`set_datasets(Option<Vec<DistributeDataset>>)`](crate::client::fluent_builders::DistributeDatasetEntries::set_datasets): <p>The ARNS for the training dataset and test dataset that you want to use. The datasets must belong to the same project. The test dataset must be empty. </p>
                            /// - On success, responds with [`DistributeDatasetEntriesOutput`](crate::output::DistributeDatasetEntriesOutput)
                            /// - On failure, responds with [`SdkError<DistributeDatasetEntriesError>`](crate::error::DistributeDatasetEntriesError)
    pub fn distribute_dataset_entries(&self) -> crate::client::fluent_builders::DistributeDatasetEntries {
                                crate::client::fluent_builders::DistributeDatasetEntries::new(self.handle.clone())
                            }
}

