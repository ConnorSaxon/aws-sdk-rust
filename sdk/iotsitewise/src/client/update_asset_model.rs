// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateAssetModel`](crate::client::fluent_builders::UpdateAssetModel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`asset_model_id(impl Into<String>)`](crate::client::fluent_builders::UpdateAssetModel::asset_model_id) / [`set_asset_model_id(Option<String>)`](crate::client::fluent_builders::UpdateAssetModel::set_asset_model_id): <p>The ID of the asset model to update.</p>
    ///   - [`asset_model_name(impl Into<String>)`](crate::client::fluent_builders::UpdateAssetModel::asset_model_name) / [`set_asset_model_name(Option<String>)`](crate::client::fluent_builders::UpdateAssetModel::set_asset_model_name): <p>A unique, friendly name for the asset model.</p>
    ///   - [`asset_model_description(impl Into<String>)`](crate::client::fluent_builders::UpdateAssetModel::asset_model_description) / [`set_asset_model_description(Option<String>)`](crate::client::fluent_builders::UpdateAssetModel::set_asset_model_description): <p>A description for the asset model.</p>
    ///   - [`asset_model_properties(Vec<AssetModelProperty>)`](crate::client::fluent_builders::UpdateAssetModel::asset_model_properties) / [`set_asset_model_properties(Option<Vec<AssetModelProperty>>)`](crate::client::fluent_builders::UpdateAssetModel::set_asset_model_properties): <p>The updated property definitions of the asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-properties.html">Asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>  <p>You can specify up to 200 properties per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    ///   - [`asset_model_hierarchies(Vec<AssetModelHierarchy>)`](crate::client::fluent_builders::UpdateAssetModel::asset_model_hierarchies) / [`set_asset_model_hierarchies(Option<Vec<AssetModelHierarchy>>)`](crate::client::fluent_builders::UpdateAssetModel::set_asset_model_hierarchies): <p>The updated hierarchy definitions of the asset model. Each hierarchy specifies an asset model whose assets can be children of any other assets created from this asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-hierarchies.html">Asset hierarchies</a> in the <i>IoT SiteWise User Guide</i>.</p>  <p>You can specify up to 10 hierarchies per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    ///   - [`asset_model_composite_models(Vec<AssetModelCompositeModel>)`](crate::client::fluent_builders::UpdateAssetModel::asset_model_composite_models) / [`set_asset_model_composite_models(Option<Vec<AssetModelCompositeModel>>)`](crate::client::fluent_builders::UpdateAssetModel::set_asset_model_composite_models): <p>The composite asset models that are part of this asset model. Composite asset models are asset models that contain specific properties. Each composite model has a type that defines the properties that the composite model supports. Use composite asset models to define alarms on this asset model.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::UpdateAssetModel::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::UpdateAssetModel::set_client_token): <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
                            /// - On success, responds with [`UpdateAssetModelOutput`](crate::output::UpdateAssetModelOutput) with field(s):
    ///   - [`asset_model_status(Option<AssetModelStatus>)`](crate::output::UpdateAssetModelOutput::asset_model_status): <p>The status of the asset model, which contains a state (<code>UPDATING</code> after successfully calling this operation) and any error message.</p>
                            /// - On failure, responds with [`SdkError<UpdateAssetModelError>`](crate::error::UpdateAssetModelError)
    pub fn update_asset_model(&self) -> crate::client::fluent_builders::UpdateAssetModel {
                                crate::client::fluent_builders::UpdateAssetModel::new(self.handle.clone())
                            }
}

