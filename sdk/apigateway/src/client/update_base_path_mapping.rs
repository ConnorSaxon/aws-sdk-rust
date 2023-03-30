// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateBasePathMapping`](crate::client::fluent_builders::UpdateBasePathMapping) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::UpdateBasePathMapping::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::UpdateBasePathMapping::set_domain_name): <p>The domain name of the BasePathMapping resource to change.</p>
    ///   - [`base_path(impl Into<String>)`](crate::client::fluent_builders::UpdateBasePathMapping::base_path) / [`set_base_path(Option<String>)`](crate::client::fluent_builders::UpdateBasePathMapping::set_base_path): <p>The base path of the BasePathMapping resource to change.</p>  <p>To specify an empty base path, set this parameter to <code>'(none)'</code>.</p>
    ///   - [`patch_operations(Vec<PatchOperation>)`](crate::client::fluent_builders::UpdateBasePathMapping::patch_operations) / [`set_patch_operations(Option<Vec<PatchOperation>>)`](crate::client::fluent_builders::UpdateBasePathMapping::set_patch_operations): <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
                            /// - On success, responds with [`UpdateBasePathMappingOutput`](crate::output::UpdateBasePathMappingOutput) with field(s):
    ///   - [`base_path(Option<String>)`](crate::output::UpdateBasePathMappingOutput::base_path): <p>The base path name that callers of the API must provide as part of the URL after the domain name.</p>
    ///   - [`rest_api_id(Option<String>)`](crate::output::UpdateBasePathMappingOutput::rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`stage(Option<String>)`](crate::output::UpdateBasePathMappingOutput::stage): <p>The name of the associated stage.</p>
                            /// - On failure, responds with [`SdkError<UpdateBasePathMappingError>`](crate::error::UpdateBasePathMappingError)
    pub fn update_base_path_mapping(&self) -> crate::client::fluent_builders::UpdateBasePathMapping {
                                crate::client::fluent_builders::UpdateBasePathMapping::new(self.handle.clone())
                            }
}

