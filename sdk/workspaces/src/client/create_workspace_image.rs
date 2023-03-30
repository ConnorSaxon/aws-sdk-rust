// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateWorkspaceImage`](crate::client::fluent_builders::CreateWorkspaceImage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateWorkspaceImage::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateWorkspaceImage::set_name): <p>The name of the new WorkSpace image.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateWorkspaceImage::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateWorkspaceImage::set_description): <p>The description of the new WorkSpace image.</p>
    ///   - [`workspace_id(impl Into<String>)`](crate::client::fluent_builders::CreateWorkspaceImage::workspace_id) / [`set_workspace_id(Option<String>)`](crate::client::fluent_builders::CreateWorkspaceImage::set_workspace_id): <p>The identifier of the source WorkSpace</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateWorkspaceImage::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateWorkspaceImage::set_tags): <p>The tags that you want to add to the new WorkSpace image. To add tags when you're creating the image, you must create an IAM policy that grants your IAM user permission to use <code>workspaces:CreateTags</code>.</p>
                            /// - On success, responds with [`CreateWorkspaceImageOutput`](crate::output::CreateWorkspaceImageOutput) with field(s):
    ///   - [`image_id(Option<String>)`](crate::output::CreateWorkspaceImageOutput::image_id): <p>The identifier of the new WorkSpace image.</p>
    ///   - [`name(Option<String>)`](crate::output::CreateWorkspaceImageOutput::name): <p>The name of the image.</p>
    ///   - [`description(Option<String>)`](crate::output::CreateWorkspaceImageOutput::description): <p>The description of the image.</p>
    ///   - [`operating_system(Option<OperatingSystem>)`](crate::output::CreateWorkspaceImageOutput::operating_system): <p>The operating system that the image is running.</p>
    ///   - [`state(Option<WorkspaceImageState>)`](crate::output::CreateWorkspaceImageOutput::state): <p>The availability status of the image.</p>
    ///   - [`required_tenancy(Option<WorkspaceImageRequiredTenancy>)`](crate::output::CreateWorkspaceImageOutput::required_tenancy): <p>Specifies whether the image is running on dedicated hardware. When Bring Your Own License (BYOL) is enabled, this value is set to DEDICATED. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/byol-windows-images.htm"> Bring Your Own Windows Desktop Images.</a>.</p>
    ///   - [`created(Option<DateTime>)`](crate::output::CreateWorkspaceImageOutput::created): <p>The date when the image was created.</p>
    ///   - [`owner_account_id(Option<String>)`](crate::output::CreateWorkspaceImageOutput::owner_account_id): <p>The identifier of the Amazon Web Services account that owns the image.</p>
                            /// - On failure, responds with [`SdkError<CreateWorkspaceImageError>`](crate::error::CreateWorkspaceImageError)
    pub fn create_workspace_image(&self) -> crate::client::fluent_builders::CreateWorkspaceImage {
                                crate::client::fluent_builders::CreateWorkspaceImage::new(self.handle.clone())
                            }
}

