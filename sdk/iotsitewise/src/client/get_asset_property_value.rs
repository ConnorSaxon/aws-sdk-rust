// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetAssetPropertyValue`](crate::client::fluent_builders::GetAssetPropertyValue) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`asset_id(impl Into<String>)`](crate::client::fluent_builders::GetAssetPropertyValue::asset_id) / [`set_asset_id(Option<String>)`](crate::client::fluent_builders::GetAssetPropertyValue::set_asset_id): <p>The ID of the asset.</p>
    ///   - [`property_id(impl Into<String>)`](crate::client::fluent_builders::GetAssetPropertyValue::property_id) / [`set_property_id(Option<String>)`](crate::client::fluent_builders::GetAssetPropertyValue::set_property_id): <p>The ID of the asset property.</p>
    ///   - [`property_alias(impl Into<String>)`](crate::client::fluent_builders::GetAssetPropertyValue::property_alias) / [`set_property_alias(Option<String>)`](crate::client::fluent_builders::GetAssetPropertyValue::set_property_alias): <p>The alias that identifies the property, such as an OPC-UA server data stream path (for example, <code>/company/windfarm/3/turbine/7/temperature</code>). For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/connect-data-streams.html">Mapping industrial data streams to asset properties</a> in the <i>IoT SiteWise User Guide</i>.</p>
                            /// - On success, responds with [`GetAssetPropertyValueOutput`](crate::output::GetAssetPropertyValueOutput) with field(s):
    ///   - [`property_value(Option<AssetPropertyValue>)`](crate::output::GetAssetPropertyValueOutput::property_value): <p>The current asset property value.</p>
                            /// - On failure, responds with [`SdkError<GetAssetPropertyValueError>`](crate::error::GetAssetPropertyValueError)
    pub fn get_asset_property_value(&self) -> crate::client::fluent_builders::GetAssetPropertyValue {
                                crate::client::fluent_builders::GetAssetPropertyValue::new(self.handle.clone())
                            }
}

