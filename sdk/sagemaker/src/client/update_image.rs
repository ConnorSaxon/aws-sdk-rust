// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateImage`](crate::client::fluent_builders::UpdateImage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`delete_properties(Vec<String>)`](crate::client::fluent_builders::UpdateImage::delete_properties) / [`set_delete_properties(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateImage::set_delete_properties): <p>A list of properties to delete. Only the <code>Description</code> and <code>DisplayName</code> properties can be deleted.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateImage::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateImage::set_description): <p>The new description for the image.</p>
    ///   - [`display_name(impl Into<String>)`](crate::client::fluent_builders::UpdateImage::display_name) / [`set_display_name(Option<String>)`](crate::client::fluent_builders::UpdateImage::set_display_name): <p>The new display name for the image.</p>
    ///   - [`image_name(impl Into<String>)`](crate::client::fluent_builders::UpdateImage::image_name) / [`set_image_name(Option<String>)`](crate::client::fluent_builders::UpdateImage::set_image_name): <p>The name of the image to update.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateImage::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::UpdateImage::set_role_arn): <p>The new ARN for the IAM role that enables Amazon SageMaker to perform tasks on your behalf.</p>
                            /// - On success, responds with [`UpdateImageOutput`](crate::output::UpdateImageOutput) with field(s):
    ///   - [`image_arn(Option<String>)`](crate::output::UpdateImageOutput::image_arn): <p>The ARN of the image.</p>
                            /// - On failure, responds with [`SdkError<UpdateImageError>`](crate::error::UpdateImageError)
    pub fn update_image(&self) -> crate::client::fluent_builders::UpdateImage {
                                crate::client::fluent_builders::UpdateImage::new(self.handle.clone())
                            }
}

