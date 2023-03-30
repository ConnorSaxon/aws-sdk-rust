// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SetPrincipalTagAttributeMap`](crate::client::fluent_builders::SetPrincipalTagAttributeMap) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity_pool_id(impl Into<String>)`](crate::client::fluent_builders::SetPrincipalTagAttributeMap::identity_pool_id) / [`set_identity_pool_id(Option<String>)`](crate::client::fluent_builders::SetPrincipalTagAttributeMap::set_identity_pool_id): <p>The ID of the Identity Pool you want to set attribute mappings for.</p>
    ///   - [`identity_provider_name(impl Into<String>)`](crate::client::fluent_builders::SetPrincipalTagAttributeMap::identity_provider_name) / [`set_identity_provider_name(Option<String>)`](crate::client::fluent_builders::SetPrincipalTagAttributeMap::set_identity_provider_name): <p>The provider name you want to use for attribute mappings.</p>
    ///   - [`use_defaults(bool)`](crate::client::fluent_builders::SetPrincipalTagAttributeMap::use_defaults) / [`set_use_defaults(Option<bool>)`](crate::client::fluent_builders::SetPrincipalTagAttributeMap::set_use_defaults): <p>You can use this operation to use default (username and clientID) attribute mappings.</p>
    ///   - [`principal_tags(HashMap<String, String>)`](crate::client::fluent_builders::SetPrincipalTagAttributeMap::principal_tags) / [`set_principal_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::SetPrincipalTagAttributeMap::set_principal_tags): <p>You can use this operation to add principal tags.</p>
                            /// - On success, responds with [`SetPrincipalTagAttributeMapOutput`](crate::output::SetPrincipalTagAttributeMapOutput) with field(s):
    ///   - [`identity_pool_id(Option<String>)`](crate::output::SetPrincipalTagAttributeMapOutput::identity_pool_id): <p>The ID of the Identity Pool you want to set attribute mappings for.</p>
    ///   - [`identity_provider_name(Option<String>)`](crate::output::SetPrincipalTagAttributeMapOutput::identity_provider_name): <p>The provider name you want to use for attribute mappings.</p>
    ///   - [`use_defaults(Option<bool>)`](crate::output::SetPrincipalTagAttributeMapOutput::use_defaults): <p>You can use this operation to select default (username and clientID) attribute mappings.</p>
    ///   - [`principal_tags(Option<HashMap<String, String>>)`](crate::output::SetPrincipalTagAttributeMapOutput::principal_tags): <p>You can use this operation to add principal tags. The <code>PrincipalTags</code>operation enables you to reference user attributes in your IAM permissions policy.</p>
                            /// - On failure, responds with [`SdkError<SetPrincipalTagAttributeMapError>`](crate::error::SetPrincipalTagAttributeMapError)
    pub fn set_principal_tag_attribute_map(&self) -> crate::client::fluent_builders::SetPrincipalTagAttributeMap {
                                crate::client::fluent_builders::SetPrincipalTagAttributeMap::new(self.handle.clone())
                            }
}

