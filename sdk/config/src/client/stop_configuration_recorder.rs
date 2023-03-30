// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopConfigurationRecorder`](crate::client::fluent_builders::StopConfigurationRecorder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_recorder_name(impl Into<String>)`](crate::client::fluent_builders::StopConfigurationRecorder::configuration_recorder_name) / [`set_configuration_recorder_name(Option<String>)`](crate::client::fluent_builders::StopConfigurationRecorder::set_configuration_recorder_name): <p>The name of the recorder object that records each configuration change made to the resources.</p>
                            /// - On success, responds with [`StopConfigurationRecorderOutput`](crate::output::StopConfigurationRecorderOutput)
                            /// - On failure, responds with [`SdkError<StopConfigurationRecorderError>`](crate::error::StopConfigurationRecorderError)
    pub fn stop_configuration_recorder(&self) -> crate::client::fluent_builders::StopConfigurationRecorder {
                                crate::client::fluent_builders::StopConfigurationRecorder::new(self.handle.clone())
                            }
}

