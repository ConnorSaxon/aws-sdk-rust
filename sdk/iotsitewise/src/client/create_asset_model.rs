// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAssetModel`](crate::client::fluent_builders::CreateAssetModel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`asset_model_name(impl Into<String>)`](crate::client::fluent_builders::CreateAssetModel::asset_model_name) / [`set_asset_model_name(Option<String>)`](crate::client::fluent_builders::CreateAssetModel::set_asset_model_name): <p>A unique, friendly name for the asset model.</p>
    ///   - [`asset_model_description(impl Into<String>)`](crate::client::fluent_builders::CreateAssetModel::asset_model_description) / [`set_asset_model_description(Option<String>)`](crate::client::fluent_builders::CreateAssetModel::set_asset_model_description): <p>A description for the asset model.</p>
    ///   - [`asset_model_properties(Vec<AssetModelPropertyDefinition>)`](crate::client::fluent_builders::CreateAssetModel::asset_model_properties) / [`set_asset_model_properties(Option<Vec<AssetModelPropertyDefinition>>)`](crate::client::fluent_builders::CreateAssetModel::set_asset_model_properties): <p>The property definitions of the asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-properties.html">Asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>  <p>You can specify up to 200 properties per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    ///   - [`asset_model_hierarchies(Vec<AssetModelHierarchyDefinition>)`](crate::client::fluent_builders::CreateAssetModel::asset_model_hierarchies) / [`set_asset_model_hierarchies(Option<Vec<AssetModelHierarchyDefinition>>)`](crate::client::fluent_builders::CreateAssetModel::set_asset_model_hierarchies): <p>The hierarchy definitions of the asset model. Each hierarchy specifies an asset model whose assets can be children of any other assets created from this asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-hierarchies.html">Asset hierarchies</a> in the <i>IoT SiteWise User Guide</i>.</p>  <p>You can specify up to 10 hierarchies per asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/quotas.html">Quotas</a> in the <i>IoT SiteWise User Guide</i>.</p>
    ///   - [`asset_model_composite_models(Vec<AssetModelCompositeModelDefinition>)`](crate::client::fluent_builders::CreateAssetModel::asset_model_composite_models) / [`set_asset_model_composite_models(Option<Vec<AssetModelCompositeModelDefinition>>)`](crate::client::fluent_builders::CreateAssetModel::set_asset_model_composite_models): <p>The composite asset models that are part of this asset model. Composite asset models are asset models that contain specific properties. Each composite model has a type that defines the properties that the composite model supports. Use composite asset models to define alarms on this asset model.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateAssetModel::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateAssetModel::set_client_token): <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateAssetModel::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateAssetModel::set_tags): <p>A list of key-value pairs that contain metadata for the asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/tag-resources.html">Tagging your IoT SiteWise resources</a> in the <i>IoT SiteWise User Guide</i>.</p>
                            /// - On success, responds with [`CreateAssetModelOutput`](crate::output::CreateAssetModelOutput) with field(s):
    ///   - [`asset_model_id(Option<String>)`](crate::output::CreateAssetModelOutput::asset_model_id): <p>The ID of the asset model. You can use this ID when you call other IoT SiteWise APIs.</p>
    ///   - [`asset_model_arn(Option<String>)`](crate::output::CreateAssetModelOutput::asset_model_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the asset model, which has the following format.</p>  <p> <code>arn:${Partition}:iotsitewise:${Region}:${Account}:asset-model/${AssetModelId}</code> </p>
    ///   - [`asset_model_status(Option<AssetModelStatus>)`](crate::output::CreateAssetModelOutput::asset_model_status): <p>The status of the asset model, which contains a state (<code>CREATING</code> after successfully calling this operation) and any error message.</p>
                            /// - On failure, responds with [`SdkError<CreateAssetModelError>`](crate::error::CreateAssetModelError)
    pub fn create_asset_model(&self) -> crate::client::fluent_builders::CreateAssetModel {
                                crate::client::fluent_builders::CreateAssetModel::new(self.handle.clone())
                            }
}

