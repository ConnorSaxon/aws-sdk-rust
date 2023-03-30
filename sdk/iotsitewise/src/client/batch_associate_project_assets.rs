// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchAssociateProjectAssets`](crate::client::fluent_builders::BatchAssociateProjectAssets) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project_id(impl Into<String>)`](crate::client::fluent_builders::BatchAssociateProjectAssets::project_id) / [`set_project_id(Option<String>)`](crate::client::fluent_builders::BatchAssociateProjectAssets::set_project_id): <p>The ID of the project to which to associate the assets.</p>
    ///   - [`asset_ids(Vec<String>)`](crate::client::fluent_builders::BatchAssociateProjectAssets::asset_ids) / [`set_asset_ids(Option<Vec<String>>)`](crate::client::fluent_builders::BatchAssociateProjectAssets::set_asset_ids): <p>The IDs of the assets to be associated to the project.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::BatchAssociateProjectAssets::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::BatchAssociateProjectAssets::set_client_token): <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
                            /// - On success, responds with [`BatchAssociateProjectAssetsOutput`](crate::output::BatchAssociateProjectAssetsOutput) with field(s):
    ///   - [`errors(Option<Vec<AssetErrorDetails>>)`](crate::output::BatchAssociateProjectAssetsOutput::errors): <p>A list of associated error information, if any.</p>
                            /// - On failure, responds with [`SdkError<BatchAssociateProjectAssetsError>`](crate::error::BatchAssociateProjectAssetsError)
    pub fn batch_associate_project_assets(&self) -> crate::client::fluent_builders::BatchAssociateProjectAssets {
                                crate::client::fluent_builders::BatchAssociateProjectAssets::new(self.handle.clone())
                            }
}

