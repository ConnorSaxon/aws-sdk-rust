// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateStoreImageTask`](crate::client::fluent_builders::CreateStoreImageTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`image_id(impl Into<String>)`](crate::client::fluent_builders::CreateStoreImageTask::image_id) / [`set_image_id(Option<String>)`](crate::client::fluent_builders::CreateStoreImageTask::set_image_id): <p>The ID of the AMI.</p>
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::CreateStoreImageTask::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::CreateStoreImageTask::set_bucket): <p>The name of the Amazon S3 bucket in which the AMI object will be stored. The bucket must be in the Region in which the request is being made. The AMI object appears in the bucket only after the upload task has completed. </p>
    ///   - [`s3_object_tags(Vec<S3ObjectTag>)`](crate::client::fluent_builders::CreateStoreImageTask::s3_object_tags) / [`set_s3_object_tags(Option<Vec<S3ObjectTag>>)`](crate::client::fluent_builders::CreateStoreImageTask::set_s3_object_tags): <p>The tags to apply to the AMI object that will be stored in the Amazon S3 bucket. </p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateStoreImageTask::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateStoreImageTask::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`CreateStoreImageTaskOutput`](crate::output::CreateStoreImageTaskOutput) with field(s):
    ///   - [`object_key(Option<String>)`](crate::output::CreateStoreImageTaskOutput::object_key): <p>The name of the stored AMI object in the S3 bucket.</p>
                            /// - On failure, responds with [`SdkError<CreateStoreImageTaskError>`](crate::error::CreateStoreImageTaskError)
    pub fn create_store_image_task(&self) -> crate::client::fluent_builders::CreateStoreImageTask {
                                crate::client::fluent_builders::CreateStoreImageTask::new(self.handle.clone())
                            }
}

