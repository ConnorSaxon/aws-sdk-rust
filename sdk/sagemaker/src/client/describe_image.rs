// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeImage`](crate::client::fluent_builders::DescribeImage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`image_name(impl Into<String>)`](crate::client::fluent_builders::DescribeImage::image_name) / [`set_image_name(Option<String>)`](crate::client::fluent_builders::DescribeImage::set_image_name): <p>The name of the image to describe.</p>
                            /// - On success, responds with [`DescribeImageOutput`](crate::output::DescribeImageOutput) with field(s):
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeImageOutput::creation_time): <p>When the image was created.</p>
    ///   - [`description(Option<String>)`](crate::output::DescribeImageOutput::description): <p>The description of the image.</p>
    ///   - [`display_name(Option<String>)`](crate::output::DescribeImageOutput::display_name): <p>The name of the image as displayed.</p>
    ///   - [`failure_reason(Option<String>)`](crate::output::DescribeImageOutput::failure_reason): <p>When a create, update, or delete operation fails, the reason for the failure.</p>
    ///   - [`image_arn(Option<String>)`](crate::output::DescribeImageOutput::image_arn): <p>The ARN of the image.</p>
    ///   - [`image_name(Option<String>)`](crate::output::DescribeImageOutput::image_name): <p>The name of the image.</p>
    ///   - [`image_status(Option<ImageStatus>)`](crate::output::DescribeImageOutput::image_status): <p>The status of the image.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::DescribeImageOutput::last_modified_time): <p>When the image was last modified.</p>
    ///   - [`role_arn(Option<String>)`](crate::output::DescribeImageOutput::role_arn): <p>The ARN of the IAM role that enables Amazon SageMaker to perform tasks on your behalf.</p>
                            /// - On failure, responds with [`SdkError<DescribeImageError>`](crate::error::DescribeImageError)
    pub fn describe_image(&self) -> crate::client::fluent_builders::DescribeImage {
                                crate::client::fluent_builders::DescribeImage::new(self.handle.clone())
                            }
}

