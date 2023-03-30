// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetFuotaTask`](crate::client::fluent_builders::GetFuotaTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetFuotaTask::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetFuotaTask::set_id): <p>The ID of a FUOTA task.</p>
                            /// - On success, responds with [`GetFuotaTaskOutput`](crate::output::GetFuotaTaskOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::GetFuotaTaskOutput::arn): <p>The arn of a FUOTA task.</p>
    ///   - [`id(Option<String>)`](crate::output::GetFuotaTaskOutput::id): <p>The ID of a FUOTA task.</p>
    ///   - [`status(Option<FuotaTaskStatus>)`](crate::output::GetFuotaTaskOutput::status): <p>The status of a FUOTA task.</p>
    ///   - [`name(Option<String>)`](crate::output::GetFuotaTaskOutput::name): <p>The name of a FUOTA task.</p>
    ///   - [`description(Option<String>)`](crate::output::GetFuotaTaskOutput::description): <p>The description of the new resource.</p>
    ///   - [`lo_ra_wan(Option<LoRaWanFuotaTaskGetInfo>)`](crate::output::GetFuotaTaskOutput::lo_ra_wan): <p>The LoRaWAN information returned from getting a FUOTA task.</p>
    ///   - [`firmware_update_image(Option<String>)`](crate::output::GetFuotaTaskOutput::firmware_update_image): <p>The S3 URI points to a firmware update image that is to be used with a FUOTA task.</p>
    ///   - [`firmware_update_role(Option<String>)`](crate::output::GetFuotaTaskOutput::firmware_update_role): <p>The firmware update role that is to be used with a FUOTA task.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::GetFuotaTaskOutput::created_at): <p>Created at timestamp for the resource.</p>
                            /// - On failure, responds with [`SdkError<GetFuotaTaskError>`](crate::error::GetFuotaTaskError)
    pub fn get_fuota_task(&self) -> crate::client::fluent_builders::GetFuotaTask {
                                crate::client::fluent_builders::GetFuotaTask::new(self.handle.clone())
                            }
}

