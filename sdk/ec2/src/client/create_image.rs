// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateImage`](crate::client::fluent_builders::CreateImage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`block_device_mappings(Vec<BlockDeviceMapping>)`](crate::client::fluent_builders::CreateImage::block_device_mappings) / [`set_block_device_mappings(Option<Vec<BlockDeviceMapping>>)`](crate::client::fluent_builders::CreateImage::set_block_device_mappings): <p>The block device mappings. This parameter cannot be used to modify the encryption status of existing volumes or snapshots. To create an AMI with encrypted snapshots, use the <code>CopyImage</code> action.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateImage::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateImage::set_description): <p>A description for the new image.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateImage::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateImage::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::CreateImage::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::CreateImage::set_instance_id): <p>The ID of the instance.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateImage::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateImage::set_name): <p>A name for the new image.</p>  <p>Constraints: 3-128 alphanumeric characters, parentheses (()), square brackets ([]), spaces ( ), periods (.), slashes (/), dashes (-), single quotes ('), at-signs (@), or underscores(_)</p>
    ///   - [`no_reboot(bool)`](crate::client::fluent_builders::CreateImage::no_reboot) / [`set_no_reboot(Option<bool>)`](crate::client::fluent_builders::CreateImage::set_no_reboot): <p>By default, when Amazon EC2 creates the new AMI, it reboots the instance so that it can take snapshots of the attached volumes while data is at rest, in order to ensure a consistent state. You can set the <code>NoReboot</code> parameter to <code>true</code> in the API request, or use the <code>--no-reboot</code> option in the CLI to prevent Amazon EC2 from shutting down and rebooting the instance.</p> <important>   <p>If you choose to bypass the shutdown and reboot process by setting the <code>NoReboot</code> parameter to <code>true</code> in the API request, or by using the <code>--no-reboot</code> option in the CLI, we can't guarantee the file system integrity of the created image.</p>  </important>  <p>Default: <code>false</code> (follow standard reboot process)</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::CreateImage::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::CreateImage::set_tag_specifications): <p>The tags to apply to the AMI and snapshots on creation. You can tag the AMI, the snapshots, or both.</p>  <ul>   <li> <p>To tag the AMI, the value for <code>ResourceType</code> must be <code>image</code>.</p> </li>   <li> <p>To tag the snapshots that are created of the root volume and of other Amazon EBS volumes that are attached to the instance, the value for <code>ResourceType</code> must be <code>snapshot</code>. The same tag is applied to all of the snapshots that are created.</p> </li>  </ul>  <p>If you specify other values for <code>ResourceType</code>, the request fails.</p>  <p>To tag an AMI or snapshot after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
                            /// - On success, responds with [`CreateImageOutput`](crate::output::CreateImageOutput) with field(s):
    ///   - [`image_id(Option<String>)`](crate::output::CreateImageOutput::image_id): <p>The ID of the new AMI.</p>
                            /// - On failure, responds with [`SdkError<CreateImageError>`](crate::error::CreateImageError)
    pub fn create_image(&self) -> crate::client::fluent_builders::CreateImage {
                                crate::client::fluent_builders::CreateImage::new(self.handle.clone())
                            }
}

