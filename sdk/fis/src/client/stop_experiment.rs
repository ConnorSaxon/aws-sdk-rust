// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopExperiment`](crate::client::fluent_builders::StopExperiment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::StopExperiment::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::StopExperiment::set_id): <p>The ID of the experiment.</p>
                            /// - On success, responds with [`StopExperimentOutput`](crate::output::StopExperimentOutput) with field(s):
    ///   - [`experiment(Option<Experiment>)`](crate::output::StopExperimentOutput::experiment): <p>Information about the experiment.</p>
                            /// - On failure, responds with [`SdkError<StopExperimentError>`](crate::error::StopExperimentError)
    pub fn stop_experiment(&self) -> crate::client::fluent_builders::StopExperiment {
                                crate::client::fluent_builders::StopExperiment::new(self.handle.clone())
                            }
}

