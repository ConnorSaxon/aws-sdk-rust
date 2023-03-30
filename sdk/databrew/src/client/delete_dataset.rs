// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDataset`](crate::client::fluent_builders::DeleteDataset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteDataset::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteDataset::set_name): <p>The name of the dataset to be deleted.</p>
                            /// - On success, responds with [`DeleteDatasetOutput`](crate::output::DeleteDatasetOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::DeleteDatasetOutput::name): <p>The name of the dataset that you deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteDatasetError>`](crate::error::DeleteDatasetError)
    pub fn delete_dataset(&self) -> crate::client::fluent_builders::DeleteDataset {
                                crate::client::fluent_builders::DeleteDataset::new(self.handle.clone())
                            }
}

