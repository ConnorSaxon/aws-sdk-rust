// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartRecovery`](crate::client::fluent_builders::StartRecovery) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_servers(Vec<StartRecoveryRequestSourceServer>)`](crate::client::fluent_builders::StartRecovery::source_servers) / [`set_source_servers(Option<Vec<StartRecoveryRequestSourceServer>>)`](crate::client::fluent_builders::StartRecovery::set_source_servers): <p>The Source Servers that we want to start a Recovery Job for.</p>
    ///   - [`is_drill(bool)`](crate::client::fluent_builders::StartRecovery::is_drill) / [`set_is_drill(Option<bool>)`](crate::client::fluent_builders::StartRecovery::set_is_drill): <p>Whether this Source Server Recovery operation is a drill or not.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::StartRecovery::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::StartRecovery::set_tags): <p>The tags to be associated with the Recovery Job.</p>
                            /// - On success, responds with [`StartRecoveryOutput`](crate::output::StartRecoveryOutput) with field(s):
    ///   - [`job(Option<Job>)`](crate::output::StartRecoveryOutput::job): <p>The Recovery Job.</p>
                            /// - On failure, responds with [`SdkError<StartRecoveryError>`](crate::error::StartRecoveryError)
    pub fn start_recovery(&self) -> crate::client::fluent_builders::StartRecovery {
                                crate::client::fluent_builders::StartRecovery::new(self.handle.clone())
                            }
}

