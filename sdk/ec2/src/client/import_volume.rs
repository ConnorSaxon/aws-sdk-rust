// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ImportVolume`](crate::client::fluent_builders::ImportVolume) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`availability_zone(impl Into<String>)`](crate::client::fluent_builders::ImportVolume::availability_zone) / [`set_availability_zone(Option<String>)`](crate::client::fluent_builders::ImportVolume::set_availability_zone): <p>The Availability Zone for the resulting EBS volume.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::ImportVolume::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::ImportVolume::set_description): <p>A description of the volume.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::ImportVolume::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::ImportVolume::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`image(DiskImageDetail)`](crate::client::fluent_builders::ImportVolume::image) / [`set_image(Option<DiskImageDetail>)`](crate::client::fluent_builders::ImportVolume::set_image): <p>The disk image.</p>
    ///   - [`volume(VolumeDetail)`](crate::client::fluent_builders::ImportVolume::volume) / [`set_volume(Option<VolumeDetail>)`](crate::client::fluent_builders::ImportVolume::set_volume): <p>The volume size.</p>
                            /// - On success, responds with [`ImportVolumeOutput`](crate::output::ImportVolumeOutput) with field(s):
    ///   - [`conversion_task(Option<ConversionTask>)`](crate::output::ImportVolumeOutput::conversion_task): <p>Information about the conversion task.</p>
                            /// - On failure, responds with [`SdkError<ImportVolumeError>`](crate::error::ImportVolumeError)
    pub fn import_volume(&self) -> crate::client::fluent_builders::ImportVolume {
                                crate::client::fluent_builders::ImportVolume::new(self.handle.clone())
                            }
}

