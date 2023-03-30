// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`MergeDeveloperIdentities`](crate::client::fluent_builders::MergeDeveloperIdentities) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`source_user_identifier(impl Into<String>)`](crate::client::fluent_builders::MergeDeveloperIdentities::source_user_identifier) / [`set_source_user_identifier(Option<String>)`](crate::client::fluent_builders::MergeDeveloperIdentities::set_source_user_identifier): <p>User identifier for the source user. The value should be a <code>DeveloperUserIdentifier</code>.</p>
    ///   - [`destination_user_identifier(impl Into<String>)`](crate::client::fluent_builders::MergeDeveloperIdentities::destination_user_identifier) / [`set_destination_user_identifier(Option<String>)`](crate::client::fluent_builders::MergeDeveloperIdentities::set_destination_user_identifier): <p>User identifier for the destination user. The value should be a <code>DeveloperUserIdentifier</code>.</p>
    ///   - [`developer_provider_name(impl Into<String>)`](crate::client::fluent_builders::MergeDeveloperIdentities::developer_provider_name) / [`set_developer_provider_name(Option<String>)`](crate::client::fluent_builders::MergeDeveloperIdentities::set_developer_provider_name): <p>The "domain" by which Cognito will refer to your users. This is a (pseudo) domain name that you provide while creating an identity pool. This name acts as a placeholder that allows your backend and the Cognito service to communicate about the developer provider. For the <code>DeveloperProviderName</code>, you can use letters as well as period (.), underscore (_), and dash (-).</p>
    ///   - [`identity_pool_id(impl Into<String>)`](crate::client::fluent_builders::MergeDeveloperIdentities::identity_pool_id) / [`set_identity_pool_id(Option<String>)`](crate::client::fluent_builders::MergeDeveloperIdentities::set_identity_pool_id): <p>An identity pool ID in the format REGION:GUID.</p>
                            /// - On success, responds with [`MergeDeveloperIdentitiesOutput`](crate::output::MergeDeveloperIdentitiesOutput) with field(s):
    ///   - [`identity_id(Option<String>)`](crate::output::MergeDeveloperIdentitiesOutput::identity_id): <p>A unique identifier in the format REGION:GUID.</p>
                            /// - On failure, responds with [`SdkError<MergeDeveloperIdentitiesError>`](crate::error::MergeDeveloperIdentitiesError)
    pub fn merge_developer_identities(&self) -> crate::client::fluent_builders::MergeDeveloperIdentities {
                                crate::client::fluent_builders::MergeDeveloperIdentities::new(self.handle.clone())
                            }
}

