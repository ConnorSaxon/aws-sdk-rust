// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRecordingConfiguration`](crate::client::fluent_builders::DeleteRecordingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::DeleteRecordingConfiguration::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::DeleteRecordingConfiguration::set_arn): <p>ARN of the recording configuration to be deleted.</p>
                            /// - On success, responds with [`DeleteRecordingConfigurationOutput`](crate::output::DeleteRecordingConfigurationOutput)
                            /// - On failure, responds with [`SdkError<DeleteRecordingConfigurationError>`](crate::error::DeleteRecordingConfigurationError)
    pub fn delete_recording_configuration(&self) -> crate::client::fluent_builders::DeleteRecordingConfiguration {
                                crate::client::fluent_builders::DeleteRecordingConfiguration::new(self.handle.clone())
                            }
}

