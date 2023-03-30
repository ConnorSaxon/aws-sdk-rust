// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateLocationS3`](crate::client::fluent_builders::CreateLocationS3) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`subdirectory(impl Into<String>)`](crate::client::fluent_builders::CreateLocationS3::subdirectory) / [`set_subdirectory(Option<String>)`](crate::client::fluent_builders::CreateLocationS3::set_subdirectory): <p>A subdirectory in the Amazon S3 bucket. This subdirectory in Amazon S3 is used to read data from the S3 source location or write data to the S3 destination.</p>
    ///   - [`s3_bucket_arn(impl Into<String>)`](crate::client::fluent_builders::CreateLocationS3::s3_bucket_arn) / [`set_s3_bucket_arn(Option<String>)`](crate::client::fluent_builders::CreateLocationS3::set_s3_bucket_arn): <p>The ARN of the Amazon S3 bucket. If the bucket is on an Amazon Web Services Outpost, this must be an access point ARN.</p>
    ///   - [`s3_storage_class(S3StorageClass)`](crate::client::fluent_builders::CreateLocationS3::s3_storage_class) / [`set_s3_storage_class(Option<S3StorageClass>)`](crate::client::fluent_builders::CreateLocationS3::set_s3_storage_class): <p>The Amazon S3 storage class that you want to store your files in when this location is used as a task destination. For buckets in Amazon Web Services Regions, the storage class defaults to Standard. For buckets on Outposts, the storage class defaults to Amazon Web Services S3 Outposts.</p>  <p>For more information about S3 storage classes, see <a href="http://aws.amazon.com/s3/storage-classes/">Amazon S3 Storage Classes</a>. Some storage classes have behaviors that can affect your S3 storage cost. For detailed information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#using-storage-classes">Considerations when working with S3 storage classes in DataSync</a>.</p>
    ///   - [`s3_config(S3Config)`](crate::client::fluent_builders::CreateLocationS3::s3_config) / [`set_s3_config(Option<S3Config>)`](crate::client::fluent_builders::CreateLocationS3::set_s3_config): <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role used to access an Amazon S3 bucket.</p>  <p>For detailed information about using such a role, see Creating a Location for Amazon S3 in the <i>DataSync User Guide</i>.</p>
    ///   - [`agent_arns(Vec<String>)`](crate::client::fluent_builders::CreateLocationS3::agent_arns) / [`set_agent_arns(Option<Vec<String>>)`](crate::client::fluent_builders::CreateLocationS3::set_agent_arns): <p>If you're using DataSync on an Amazon Web Services Outpost, specify the Amazon Resource Names (ARNs) of the DataSync agents deployed on your Outpost. For more information about launching a DataSync agent on an Amazon Web Services Outpost, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/deploy-agents.html#outposts-agent">Deploy your DataSync agent on Outposts</a>.</p>
    ///   - [`tags(Vec<TagListEntry>)`](crate::client::fluent_builders::CreateLocationS3::tags) / [`set_tags(Option<Vec<TagListEntry>>)`](crate::client::fluent_builders::CreateLocationS3::set_tags): <p>The key-value pair that represents the tag that you want to add to the location. The value can be an empty string. We recommend using tags to name your resources.</p>
                            /// - On success, responds with [`CreateLocationS3Output`](crate::output::CreateLocationS3Output) with field(s):
    ///   - [`location_arn(Option<String>)`](crate::output::CreateLocationS3Output::location_arn): <p>The Amazon Resource Name (ARN) of the source Amazon S3 bucket location that is created.</p>
                            /// - On failure, responds with [`SdkError<CreateLocationS3Error>`](crate::error::CreateLocationS3Error)
    pub fn create_location_s3(&self) -> crate::client::fluent_builders::CreateLocationS3 {
                                crate::client::fluent_builders::CreateLocationS3::new(self.handle.clone())
                            }
}

