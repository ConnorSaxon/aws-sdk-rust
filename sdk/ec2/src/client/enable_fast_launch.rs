// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`EnableFastLaunch`](crate::client::fluent_builders::EnableFastLaunch) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`image_id(impl Into<String>)`](crate::client::fluent_builders::EnableFastLaunch::image_id) / [`set_image_id(Option<String>)`](crate::client::fluent_builders::EnableFastLaunch::set_image_id): <p>The ID of the image for which you’re enabling faster launching.</p>
    ///   - [`resource_type(impl Into<String>)`](crate::client::fluent_builders::EnableFastLaunch::resource_type) / [`set_resource_type(Option<String>)`](crate::client::fluent_builders::EnableFastLaunch::set_resource_type): <p>The type of resource to use for pre-provisioning the Windows AMI for faster launching. Supported values include: <code>snapshot</code>, which is the default value.</p>
    ///   - [`snapshot_configuration(FastLaunchSnapshotConfigurationRequest)`](crate::client::fluent_builders::EnableFastLaunch::snapshot_configuration) / [`set_snapshot_configuration(Option<FastLaunchSnapshotConfigurationRequest>)`](crate::client::fluent_builders::EnableFastLaunch::set_snapshot_configuration): <p>Configuration settings for creating and managing the snapshots that are used for pre-provisioning the Windows AMI for faster launching. The associated <code>ResourceType</code> must be <code>snapshot</code>.</p>
    ///   - [`launch_template(FastLaunchLaunchTemplateSpecificationRequest)`](crate::client::fluent_builders::EnableFastLaunch::launch_template) / [`set_launch_template(Option<FastLaunchLaunchTemplateSpecificationRequest>)`](crate::client::fluent_builders::EnableFastLaunch::set_launch_template): <p>The launch template to use when launching Windows instances from pre-provisioned snapshots. Launch template parameters can include either the name or ID of the launch template, but not both.</p>
    ///   - [`max_parallel_launches(i32)`](crate::client::fluent_builders::EnableFastLaunch::max_parallel_launches) / [`set_max_parallel_launches(Option<i32>)`](crate::client::fluent_builders::EnableFastLaunch::set_max_parallel_launches): <p>The maximum number of parallel instances to launch for creating resources. Value must be <code>6</code> or greater. </p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::EnableFastLaunch::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::EnableFastLaunch::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`EnableFastLaunchOutput`](crate::output::EnableFastLaunchOutput) with field(s):
    ///   - [`image_id(Option<String>)`](crate::output::EnableFastLaunchOutput::image_id): <p>The image ID that identifies the Windows AMI for which faster launching was enabled.</p>
    ///   - [`resource_type(Option<FastLaunchResourceType>)`](crate::output::EnableFastLaunchOutput::resource_type): <p>The type of resource that was defined for pre-provisioning the Windows AMI for faster launching.</p>
    ///   - [`snapshot_configuration(Option<FastLaunchSnapshotConfigurationResponse>)`](crate::output::EnableFastLaunchOutput::snapshot_configuration): <p>The configuration settings that were defined for creating and managing the pre-provisioned snapshots for faster launching of the Windows AMI. This property is returned when the associated <code>resourceType</code> is <code>snapshot</code>.</p>
    ///   - [`launch_template(Option<FastLaunchLaunchTemplateSpecificationResponse>)`](crate::output::EnableFastLaunchOutput::launch_template): <p>The launch template that is used when launching Windows instances from pre-provisioned snapshots.</p>
    ///   - [`max_parallel_launches(Option<i32>)`](crate::output::EnableFastLaunchOutput::max_parallel_launches): <p>The maximum number of parallel instances to launch for creating resources.</p>
    ///   - [`owner_id(Option<String>)`](crate::output::EnableFastLaunchOutput::owner_id): <p>The owner ID for the Windows AMI for which faster launching was enabled.</p>
    ///   - [`state(Option<FastLaunchStateCode>)`](crate::output::EnableFastLaunchOutput::state): <p>The current state of faster launching for the specified Windows AMI.</p>
    ///   - [`state_transition_reason(Option<String>)`](crate::output::EnableFastLaunchOutput::state_transition_reason): <p>The reason that the state changed for faster launching for the Windows AMI.</p>
    ///   - [`state_transition_time(Option<DateTime>)`](crate::output::EnableFastLaunchOutput::state_transition_time): <p>The time that the state changed for faster launching for the Windows AMI.</p>
                            /// - On failure, responds with [`SdkError<EnableFastLaunchError>`](crate::error::EnableFastLaunchError)
    pub fn enable_fast_launch(&self) -> crate::client::fluent_builders::EnableFastLaunch {
                                crate::client::fluent_builders::EnableFastLaunch::new(self.handle.clone())
                            }
}

