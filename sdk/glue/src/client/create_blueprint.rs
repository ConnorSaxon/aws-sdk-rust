// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateBlueprint`](crate::client::fluent_builders::CreateBlueprint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateBlueprint::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateBlueprint::set_name): <p>The name of the blueprint.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateBlueprint::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateBlueprint::set_description): <p>A description of the blueprint.</p>
    ///   - [`blueprint_location(impl Into<String>)`](crate::client::fluent_builders::CreateBlueprint::blueprint_location) / [`set_blueprint_location(Option<String>)`](crate::client::fluent_builders::CreateBlueprint::set_blueprint_location): <p>Specifies a path in Amazon S3 where the blueprint is published.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateBlueprint::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateBlueprint::set_tags): <p>The tags to be applied to this blueprint.</p>
                            /// - On success, responds with [`CreateBlueprintOutput`](crate::output::CreateBlueprintOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::CreateBlueprintOutput::name): <p>Returns the name of the blueprint that was registered.</p>
                            /// - On failure, responds with [`SdkError<CreateBlueprintError>`](crate::error::CreateBlueprintError)
    pub fn create_blueprint(&self) -> crate::client::fluent_builders::CreateBlueprint {
                                crate::client::fluent_builders::CreateBlueprint::new(self.handle.clone())
                            }
}

