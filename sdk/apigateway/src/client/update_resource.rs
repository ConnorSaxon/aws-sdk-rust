// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateResource`](crate::client::fluent_builders::UpdateResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl Into<String>)`](crate::client::fluent_builders::UpdateResource::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::client::fluent_builders::UpdateResource::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`resource_id(impl Into<String>)`](crate::client::fluent_builders::UpdateResource::resource_id) / [`set_resource_id(Option<String>)`](crate::client::fluent_builders::UpdateResource::set_resource_id): <p>The identifier of the Resource resource.</p>
    ///   - [`patch_operations(Vec<PatchOperation>)`](crate::client::fluent_builders::UpdateResource::patch_operations) / [`set_patch_operations(Option<Vec<PatchOperation>>)`](crate::client::fluent_builders::UpdateResource::set_patch_operations): <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
                            /// - On success, responds with [`UpdateResourceOutput`](crate::output::UpdateResourceOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::UpdateResourceOutput::id): <p>The resource's identifier.</p>
    ///   - [`parent_id(Option<String>)`](crate::output::UpdateResourceOutput::parent_id): <p>The parent resource's identifier.</p>
    ///   - [`path_part(Option<String>)`](crate::output::UpdateResourceOutput::path_part): <p>The last path segment for this resource.</p>
    ///   - [`path(Option<String>)`](crate::output::UpdateResourceOutput::path): <p>The full path for this resource.</p>
    ///   - [`resource_methods(Option<HashMap<String, Method>>)`](crate::output::UpdateResourceOutput::resource_methods): <p>Gets an API resource's method of a given HTTP verb.</p>
                            /// - On failure, responds with [`SdkError<UpdateResourceError>`](crate::error::UpdateResourceError)
    pub fn update_resource(&self) -> crate::client::fluent_builders::UpdateResource {
                                crate::client::fluent_builders::UpdateResource::new(self.handle.clone())
                            }
}

