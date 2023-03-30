// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UnarchiveApplication`](crate::client::fluent_builders::UnarchiveApplication) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::UnarchiveApplication::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::UnarchiveApplication::set_application_id): <p>Application ID.</p>
                            /// - On success, responds with [`UnarchiveApplicationOutput`](crate::output::UnarchiveApplicationOutput) with field(s):
    ///   - [`application_id(Option<String>)`](crate::output::UnarchiveApplicationOutput::application_id): <p>Application ID.</p>
    ///   - [`arn(Option<String>)`](crate::output::UnarchiveApplicationOutput::arn): <p>Application ARN.</p>
    ///   - [`name(Option<String>)`](crate::output::UnarchiveApplicationOutput::name): <p>Application name.</p>
    ///   - [`description(Option<String>)`](crate::output::UnarchiveApplicationOutput::description): <p>Application description.</p>
    ///   - [`is_archived(Option<bool>)`](crate::output::UnarchiveApplicationOutput::is_archived): <p>Application archival status.</p>
    ///   - [`application_aggregated_status(Option<ApplicationAggregatedStatus>)`](crate::output::UnarchiveApplicationOutput::application_aggregated_status): <p>Application aggregated status.</p>
    ///   - [`creation_date_time(Option<String>)`](crate::output::UnarchiveApplicationOutput::creation_date_time): <p>Application creation dateTime.</p>
    ///   - [`last_modified_date_time(Option<String>)`](crate::output::UnarchiveApplicationOutput::last_modified_date_time): <p>Application last modified dateTime.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::UnarchiveApplicationOutput::tags): <p>Application tags.</p>
    ///   - [`wave_id(Option<String>)`](crate::output::UnarchiveApplicationOutput::wave_id): <p>Application wave ID.</p>
                            /// - On failure, responds with [`SdkError<UnarchiveApplicationError>`](crate::error::UnarchiveApplicationError)
    pub fn unarchive_application(&self) -> crate::client::fluent_builders::UnarchiveApplication {
                                crate::client::fluent_builders::UnarchiveApplication::new(self.handle.clone())
                            }
}

