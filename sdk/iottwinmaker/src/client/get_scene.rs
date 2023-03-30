// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetScene`](crate::client::fluent_builders::GetScene) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::client::fluent_builders::GetScene::workspace_id) / [`set_workspace_id(Option<String>)`](crate::client::fluent_builders::GetScene::set_workspace_id): <p>The ID of the workspace that contains the scene.</p>
    ///   - [`scene_id(impl Into<String>)`](crate::client::fluent_builders::GetScene::scene_id) / [`set_scene_id(Option<String>)`](crate::client::fluent_builders::GetScene::set_scene_id): <p>The ID of the scene.</p>
                            /// - On success, responds with [`GetSceneOutput`](crate::output::GetSceneOutput) with field(s):
    ///   - [`workspace_id(Option<String>)`](crate::output::GetSceneOutput::workspace_id): <p>The ID of the workspace that contains the scene.</p>
    ///   - [`scene_id(Option<String>)`](crate::output::GetSceneOutput::scene_id): <p>The ID of the scene.</p>
    ///   - [`content_location(Option<String>)`](crate::output::GetSceneOutput::content_location): <p>The relative path that specifies the location of the content definition file.</p>
    ///   - [`arn(Option<String>)`](crate::output::GetSceneOutput::arn): <p>The ARN of the scene.</p>
    ///   - [`creation_date_time(Option<DateTime>)`](crate::output::GetSceneOutput::creation_date_time): <p>The date and time when the scene was created.</p>
    ///   - [`update_date_time(Option<DateTime>)`](crate::output::GetSceneOutput::update_date_time): <p>The date and time when the scene was last updated.</p>
    ///   - [`description(Option<String>)`](crate::output::GetSceneOutput::description): <p>The description of the scene.</p>
    ///   - [`capabilities(Option<Vec<String>>)`](crate::output::GetSceneOutput::capabilities): <p>A list of capabilities that the scene uses to render.</p>
                            /// - On failure, responds with [`SdkError<GetSceneError>`](crate::error::GetSceneError)
    pub fn get_scene(&self) -> crate::client::fluent_builders::GetScene {
                                crate::client::fluent_builders::GetScene::new(self.handle.clone())
                            }
}

