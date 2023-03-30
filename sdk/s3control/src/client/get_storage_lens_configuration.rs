// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetStorageLensConfiguration`](crate::client::fluent_builders::GetStorageLensConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`config_id(impl Into<String>)`](crate::client::fluent_builders::GetStorageLensConfiguration::config_id) / [`set_config_id(Option<String>)`](crate::client::fluent_builders::GetStorageLensConfiguration::set_config_id): <p>The ID of the Amazon S3 Storage Lens configuration.</p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::GetStorageLensConfiguration::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::GetStorageLensConfiguration::set_account_id): <p>The account ID of the requester.</p>
                            /// - On success, responds with [`GetStorageLensConfigurationOutput`](crate::output::GetStorageLensConfigurationOutput) with field(s):
    ///   - [`storage_lens_configuration(Option<StorageLensConfiguration>)`](crate::output::GetStorageLensConfigurationOutput::storage_lens_configuration): <p>The S3 Storage Lens configuration requested.</p>
                            /// - On failure, responds with [`SdkError<GetStorageLensConfigurationError>`](crate::error::GetStorageLensConfigurationError)
    pub fn get_storage_lens_configuration(&self) -> crate::client::fluent_builders::GetStorageLensConfiguration {
                                crate::client::fluent_builders::GetStorageLensConfiguration::new(self.handle.clone())
                            }
}

