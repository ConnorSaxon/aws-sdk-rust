// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDataCatalogEncryptionSettings`](crate::client::fluent_builders::GetDataCatalogEncryptionSettings) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::GetDataCatalogEncryptionSettings::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::GetDataCatalogEncryptionSettings::set_catalog_id): <p>The ID of the Data Catalog to retrieve the security configuration for. If none is provided, the Amazon Web Services account ID is used by default.</p>
                            /// - On success, responds with [`GetDataCatalogEncryptionSettingsOutput`](crate::output::GetDataCatalogEncryptionSettingsOutput) with field(s):
    ///   - [`data_catalog_encryption_settings(Option<DataCatalogEncryptionSettings>)`](crate::output::GetDataCatalogEncryptionSettingsOutput::data_catalog_encryption_settings): <p>The requested security configuration.</p>
                            /// - On failure, responds with [`SdkError<GetDataCatalogEncryptionSettingsError>`](crate::error::GetDataCatalogEncryptionSettingsError)
    pub fn get_data_catalog_encryption_settings(&self) -> crate::client::fluent_builders::GetDataCatalogEncryptionSettings {
                                crate::client::fluent_builders::GetDataCatalogEncryptionSettings::new(self.handle.clone())
                            }
}

