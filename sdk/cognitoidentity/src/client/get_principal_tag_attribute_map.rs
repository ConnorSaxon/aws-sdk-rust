// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPrincipalTagAttributeMap`](crate::client::fluent_builders::GetPrincipalTagAttributeMap) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity_pool_id(impl Into<String>)`](crate::client::fluent_builders::GetPrincipalTagAttributeMap::identity_pool_id) / [`set_identity_pool_id(Option<String>)`](crate::client::fluent_builders::GetPrincipalTagAttributeMap::set_identity_pool_id): <p>You can use this operation to get the ID of the Identity Pool you setup attribute mappings for.</p>
    ///   - [`identity_provider_name(impl Into<String>)`](crate::client::fluent_builders::GetPrincipalTagAttributeMap::identity_provider_name) / [`set_identity_provider_name(Option<String>)`](crate::client::fluent_builders::GetPrincipalTagAttributeMap::set_identity_provider_name): <p>You can use this operation to get the provider name.</p>
                            /// - On success, responds with [`GetPrincipalTagAttributeMapOutput`](crate::output::GetPrincipalTagAttributeMapOutput) with field(s):
    ///   - [`identity_pool_id(Option<String>)`](crate::output::GetPrincipalTagAttributeMapOutput::identity_pool_id): <p>You can use this operation to get the ID of the Identity Pool you setup attribute mappings for.</p>
    ///   - [`identity_provider_name(Option<String>)`](crate::output::GetPrincipalTagAttributeMapOutput::identity_provider_name): <p>You can use this operation to get the provider name.</p>
    ///   - [`use_defaults(Option<bool>)`](crate::output::GetPrincipalTagAttributeMapOutput::use_defaults): <p>You can use this operation to list </p>
    ///   - [`principal_tags(Option<HashMap<String, String>>)`](crate::output::GetPrincipalTagAttributeMapOutput::principal_tags): <p>You can use this operation to add principal tags. The <code>PrincipalTags</code>operation enables you to reference user attributes in your IAM permissions policy.</p>
                            /// - On failure, responds with [`SdkError<GetPrincipalTagAttributeMapError>`](crate::error::GetPrincipalTagAttributeMapError)
    pub fn get_principal_tag_attribute_map(&self) -> crate::client::fluent_builders::GetPrincipalTagAttributeMap {
                                crate::client::fluent_builders::GetPrincipalTagAttributeMap::new(self.handle.clone())
                            }
}

