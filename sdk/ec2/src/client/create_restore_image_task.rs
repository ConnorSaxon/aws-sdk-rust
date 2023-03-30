// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateRestoreImageTask`](crate::client::fluent_builders::CreateRestoreImageTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::CreateRestoreImageTask::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::CreateRestoreImageTask::set_bucket): <p>The name of the Amazon S3 bucket that contains the stored AMI object.</p>
    ///   - [`object_key(impl Into<String>)`](crate::client::fluent_builders::CreateRestoreImageTask::object_key) / [`set_object_key(Option<String>)`](crate::client::fluent_builders::CreateRestoreImageTask::set_object_key): <p>The name of the stored AMI object in the bucket.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateRestoreImageTask::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateRestoreImageTask::set_name): <p>The name for the restored AMI. The name must be unique for AMIs in the Region for this account. If you do not provide a name, the new AMI gets the same name as the original AMI.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::CreateRestoreImageTask::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::CreateRestoreImageTask::set_tag_specifications): <p>The tags to apply to the AMI and snapshots on restoration. You can tag the AMI, the snapshots, or both.</p>  <ul>   <li> <p>To tag the AMI, the value for <code>ResourceType</code> must be <code>image</code>.</p> </li>   <li> <p>To tag the snapshots, the value for <code>ResourceType</code> must be <code>snapshot</code>. The same tag is applied to all of the snapshots that are created.</p> </li>  </ul>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateRestoreImageTask::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateRestoreImageTask::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`CreateRestoreImageTaskOutput`](crate::output::CreateRestoreImageTaskOutput) with field(s):
    ///   - [`image_id(Option<String>)`](crate::output::CreateRestoreImageTaskOutput::image_id): <p>The AMI ID.</p>
                            /// - On failure, responds with [`SdkError<CreateRestoreImageTaskError>`](crate::error::CreateRestoreImageTaskError)
    pub fn create_restore_image_task(&self) -> crate::client::fluent_builders::CreateRestoreImageTask {
                                crate::client::fluent_builders::CreateRestoreImageTask::new(self.handle.clone())
                            }
}

