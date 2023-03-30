// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAsset`](crate::client::fluent_builders::DeleteAsset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`asset_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAsset::asset_id) / [`set_asset_id(Option<String>)`](crate::client::fluent_builders::DeleteAsset::set_asset_id): <p>The unique identifier for an asset.</p>
    ///   - [`data_set_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAsset::data_set_id) / [`set_data_set_id(Option<String>)`](crate::client::fluent_builders::DeleteAsset::set_data_set_id): <p>The unique identifier for a data set.</p>
    ///   - [`revision_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAsset::revision_id) / [`set_revision_id(Option<String>)`](crate::client::fluent_builders::DeleteAsset::set_revision_id): <p>The unique identifier for a revision.</p>
                            /// - On success, responds with [`DeleteAssetOutput`](crate::output::DeleteAssetOutput)
                            /// - On failure, responds with [`SdkError<DeleteAssetError>`](crate::error::DeleteAssetError)
    pub fn delete_asset(&self) -> crate::client::fluent_builders::DeleteAsset {
                                crate::client::fluent_builders::DeleteAsset::new(self.handle.clone())
                            }
}

