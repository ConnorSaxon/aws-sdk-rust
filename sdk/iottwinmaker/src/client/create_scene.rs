// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateScene`](crate::operation::create_scene::builders::CreateSceneFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::set_workspace_id):<br>required: **true**<br><p>The ID of the workspace that contains the scene.</p><br>
    ///   - [`scene_id(impl Into<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::scene_id) / [`set_scene_id(Option<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::set_scene_id):<br>required: **true**<br><p>The ID of the scene.</p><br>
    ///   - [`content_location(impl Into<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::content_location) / [`set_content_location(Option<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::set_content_location):<br>required: **true**<br><p>The relative path that specifies the location of the content definition file.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::set_description):<br>required: **false**<br><p>The description for this scene.</p><br>
    ///   - [`capabilities(impl Into<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::capabilities) / [`set_capabilities(Option<Vec::<String>>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::set_capabilities):<br>required: **false**<br><p>A list of capabilities that the scene uses to render itself.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::set_tags):<br>required: **false**<br><p>Metadata that you can use to manage the scene.</p><br>
    ///   - [`scene_metadata(impl Into<String>, impl Into<String>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::scene_metadata) / [`set_scene_metadata(Option<HashMap::<String, String>>)`](crate::operation::create_scene::builders::CreateSceneFluentBuilder::set_scene_metadata):<br>required: **false**<br><p>The request metadata.</p><br>
    /// - On success, responds with [`CreateSceneOutput`](crate::operation::create_scene::CreateSceneOutput) with field(s):
    ///   - [`arn(String)`](crate::operation::create_scene::CreateSceneOutput::arn): <p>The ARN of the scene.</p>
    ///   - [`creation_date_time(DateTime)`](crate::operation::create_scene::CreateSceneOutput::creation_date_time): <p>The date and time when the scene was created.</p>
    /// - On failure, responds with [`SdkError<CreateSceneError>`](crate::operation::create_scene::CreateSceneError)
    pub fn create_scene(&self) -> crate::operation::create_scene::builders::CreateSceneFluentBuilder {
        crate::operation::create_scene::builders::CreateSceneFluentBuilder::new(self.handle.clone())
    }
}
