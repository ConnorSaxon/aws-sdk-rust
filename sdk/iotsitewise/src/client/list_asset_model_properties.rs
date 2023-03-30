// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAssetModelProperties`](crate::client::fluent_builders::ListAssetModelProperties) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAssetModelProperties::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`asset_model_id(impl Into<String>)`](crate::client::fluent_builders::ListAssetModelProperties::asset_model_id) / [`set_asset_model_id(Option<String>)`](crate::client::fluent_builders::ListAssetModelProperties::set_asset_model_id): <p>The ID of the asset model.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAssetModelProperties::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAssetModelProperties::set_next_token): <p>The token to be used for the next set of paginated results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAssetModelProperties::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAssetModelProperties::set_max_results): <p>The maximum number of results to return for each paginated request. If not specified, the default value is 50.</p>
    ///   - [`filter(ListAssetModelPropertiesFilter)`](crate::client::fluent_builders::ListAssetModelProperties::filter) / [`set_filter(Option<ListAssetModelPropertiesFilter>)`](crate::client::fluent_builders::ListAssetModelProperties::set_filter): <p> Filters the requested list of asset model properties. You can choose one of the following options:</p>  <ul>   <li> <p> <code>ALL</code> – The list includes all asset model properties for a given asset model ID. </p> </li>   <li> <p> <code>BASE</code> – The list includes only base asset model properties for a given asset model ID. </p> </li>  </ul>  <p>Default: <code>BASE</code> </p>
                            /// - On success, responds with [`ListAssetModelPropertiesOutput`](crate::output::ListAssetModelPropertiesOutput) with field(s):
    ///   - [`asset_model_property_summaries(Option<Vec<AssetModelPropertySummary>>)`](crate::output::ListAssetModelPropertiesOutput::asset_model_property_summaries): <p>A list that summarizes the properties associated with the specified asset model.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListAssetModelPropertiesOutput::next_token): <p>The token for the next set of results, or null if there are no additional results.</p>
                            /// - On failure, responds with [`SdkError<ListAssetModelPropertiesError>`](crate::error::ListAssetModelPropertiesError)
    pub fn list_asset_model_properties(&self) -> crate::client::fluent_builders::ListAssetModelProperties {
                                crate::client::fluent_builders::ListAssetModelProperties::new(self.handle.clone())
                            }
}

