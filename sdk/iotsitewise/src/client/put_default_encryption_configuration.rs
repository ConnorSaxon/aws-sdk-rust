// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutDefaultEncryptionConfiguration`](crate::client::fluent_builders::PutDefaultEncryptionConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`encryption_type(EncryptionType)`](crate::client::fluent_builders::PutDefaultEncryptionConfiguration::encryption_type) / [`set_encryption_type(Option<EncryptionType>)`](crate::client::fluent_builders::PutDefaultEncryptionConfiguration::set_encryption_type): <p>The type of encryption used for the encryption configuration.</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::PutDefaultEncryptionConfiguration::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::PutDefaultEncryptionConfiguration::set_kms_key_id): <p>The Key ID of the customer managed key used for KMS encryption. This is required if you use <code>KMS_BASED_ENCRYPTION</code>.</p>
                            /// - On success, responds with [`PutDefaultEncryptionConfigurationOutput`](crate::output::PutDefaultEncryptionConfigurationOutput) with field(s):
    ///   - [`encryption_type(Option<EncryptionType>)`](crate::output::PutDefaultEncryptionConfigurationOutput::encryption_type): <p>The type of encryption used for the encryption configuration.</p>
    ///   - [`kms_key_arn(Option<String>)`](crate::output::PutDefaultEncryptionConfigurationOutput::kms_key_arn): <p>The Key ARN of the KMS key used for KMS encryption if you use <code>KMS_BASED_ENCRYPTION</code>.</p>
    ///   - [`configuration_status(Option<ConfigurationStatus>)`](crate::output::PutDefaultEncryptionConfigurationOutput::configuration_status): <p>The status of the account configuration. This contains the <code>ConfigurationState</code>. If there is an error, it also contains the <code>ErrorDetails</code>.</p>
                            /// - On failure, responds with [`SdkError<PutDefaultEncryptionConfigurationError>`](crate::error::PutDefaultEncryptionConfigurationError)
    pub fn put_default_encryption_configuration(&self) -> crate::client::fluent_builders::PutDefaultEncryptionConfiguration {
                                crate::client::fluent_builders::PutDefaultEncryptionConfiguration::new(self.handle.clone())
                            }
}

