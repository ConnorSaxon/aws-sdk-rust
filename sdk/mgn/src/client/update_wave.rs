// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateWave`](crate::client::fluent_builders::UpdateWave) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`wave_id(impl Into<String>)`](crate::client::fluent_builders::UpdateWave::wave_id) / [`set_wave_id(Option<String>)`](crate::client::fluent_builders::UpdateWave::set_wave_id): <p>Wave ID.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateWave::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateWave::set_name): <p>Wave name.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateWave::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateWave::set_description): <p>Wave description.</p>
                            /// - On success, responds with [`UpdateWaveOutput`](crate::output::UpdateWaveOutput) with field(s):
    ///   - [`wave_id(Option<String>)`](crate::output::UpdateWaveOutput::wave_id): <p>Wave ID.</p>
    ///   - [`arn(Option<String>)`](crate::output::UpdateWaveOutput::arn): <p>Wave ARN.</p>
    ///   - [`name(Option<String>)`](crate::output::UpdateWaveOutput::name): <p>Wave name.</p>
    ///   - [`description(Option<String>)`](crate::output::UpdateWaveOutput::description): <p>Wave description.</p>
    ///   - [`is_archived(Option<bool>)`](crate::output::UpdateWaveOutput::is_archived): <p>Wave archival status.</p>
    ///   - [`wave_aggregated_status(Option<WaveAggregatedStatus>)`](crate::output::UpdateWaveOutput::wave_aggregated_status): <p>Wave aggregated status.</p>
    ///   - [`creation_date_time(Option<String>)`](crate::output::UpdateWaveOutput::creation_date_time): <p>Wave creation dateTime.</p>
    ///   - [`last_modified_date_time(Option<String>)`](crate::output::UpdateWaveOutput::last_modified_date_time): <p>Wave last modified dateTime.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::UpdateWaveOutput::tags): <p>Wave tags.</p>
                            /// - On failure, responds with [`SdkError<UpdateWaveError>`](crate::error::UpdateWaveError)
    pub fn update_wave(&self) -> crate::client::fluent_builders::UpdateWave {
                                crate::client::fluent_builders::UpdateWave::new(self.handle.clone())
                            }
}

