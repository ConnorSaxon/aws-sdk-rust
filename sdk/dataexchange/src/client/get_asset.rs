// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetAsset`](crate::client::fluent_builders::GetAsset) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`asset_id(impl Into<String>)`](crate::client::fluent_builders::GetAsset::asset_id) / [`set_asset_id(Option<String>)`](crate::client::fluent_builders::GetAsset::set_asset_id): <p>The unique identifier for an asset.</p>
    ///   - [`data_set_id(impl Into<String>)`](crate::client::fluent_builders::GetAsset::data_set_id) / [`set_data_set_id(Option<String>)`](crate::client::fluent_builders::GetAsset::set_data_set_id): <p>The unique identifier for a data set.</p>
    ///   - [`revision_id(impl Into<String>)`](crate::client::fluent_builders::GetAsset::revision_id) / [`set_revision_id(Option<String>)`](crate::client::fluent_builders::GetAsset::set_revision_id): <p>The unique identifier for a revision.</p>
                            /// - On success, responds with [`GetAssetOutput`](crate::output::GetAssetOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::GetAssetOutput::arn): <p>The ARN for the asset.</p>
    ///   - [`asset_details(Option<AssetDetails>)`](crate::output::GetAssetOutput::asset_details): <p>Details about the asset.</p>
    ///   - [`asset_type(Option<AssetType>)`](crate::output::GetAssetOutput::asset_type): <p>The type of asset that is added to a data set.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::GetAssetOutput::created_at): <p>The date and time that the asset was created, in ISO 8601 format.</p>
    ///   - [`data_set_id(Option<String>)`](crate::output::GetAssetOutput::data_set_id): <p>The unique identifier for the data set associated with this asset.</p>
    ///   - [`id(Option<String>)`](crate::output::GetAssetOutput::id): <p>The unique identifier for the asset.</p>
    ///   - [`name(Option<String>)`](crate::output::GetAssetOutput::name): <p>The name of the asset. When importing from Amazon S3, the Amazon S3 object key is used as the asset name. When exporting to Amazon S3, the asset name is used as default target Amazon S3 object key. When importing from Amazon API Gateway API, the API name is used as the asset name. When importing from Amazon Redshift, the datashare name is used as the asset name. When importing from AWS Lake Formation, the static values of "Database(s) included in the LF-tag policy" or "Table(s) included in the LF-tag policy" are used as the asset name.</p>
    ///   - [`revision_id(Option<String>)`](crate::output::GetAssetOutput::revision_id): <p>The unique identifier for the revision associated with this asset.</p>
    ///   - [`source_id(Option<String>)`](crate::output::GetAssetOutput::source_id): <p>The asset ID of the owned asset corresponding to the entitled asset being viewed. This parameter is returned when an asset owner is viewing the entitled copy of its owned asset.</p>
    ///   - [`updated_at(Option<DateTime>)`](crate::output::GetAssetOutput::updated_at): <p>The date and time that the asset was last updated, in ISO 8601 format.</p>
                            /// - On failure, responds with [`SdkError<GetAssetError>`](crate::error::GetAssetError)
    pub fn get_asset(&self) -> crate::client::fluent_builders::GetAsset {
                                crate::client::fluent_builders::GetAsset::new(self.handle.clone())
                            }
}

