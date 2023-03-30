// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateGraphqlApi`](crate::client::fluent_builders::UpdateGraphqlApi) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::client::fluent_builders::UpdateGraphqlApi::api_id) / [`set_api_id(Option<String>)`](crate::client::fluent_builders::UpdateGraphqlApi::set_api_id): <p>The API ID.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateGraphqlApi::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateGraphqlApi::set_name): <p>The new name for the <code>GraphqlApi</code> object.</p>
    ///   - [`log_config(LogConfig)`](crate::client::fluent_builders::UpdateGraphqlApi::log_config) / [`set_log_config(Option<LogConfig>)`](crate::client::fluent_builders::UpdateGraphqlApi::set_log_config): <p>The Amazon CloudWatch Logs configuration for the <code>GraphqlApi</code> object.</p>
    ///   - [`authentication_type(AuthenticationType)`](crate::client::fluent_builders::UpdateGraphqlApi::authentication_type) / [`set_authentication_type(Option<AuthenticationType>)`](crate::client::fluent_builders::UpdateGraphqlApi::set_authentication_type): <p>The new authentication type for the <code>GraphqlApi</code> object.</p>
    ///   - [`user_pool_config(UserPoolConfig)`](crate::client::fluent_builders::UpdateGraphqlApi::user_pool_config) / [`set_user_pool_config(Option<UserPoolConfig>)`](crate::client::fluent_builders::UpdateGraphqlApi::set_user_pool_config): <p>The new Amazon Cognito user pool configuration for the <code>~GraphqlApi</code> object.</p>
    ///   - [`open_id_connect_config(OpenIdConnectConfig)`](crate::client::fluent_builders::UpdateGraphqlApi::open_id_connect_config) / [`set_open_id_connect_config(Option<OpenIdConnectConfig>)`](crate::client::fluent_builders::UpdateGraphqlApi::set_open_id_connect_config): <p>The OpenID Connect configuration for the <code>GraphqlApi</code> object.</p>
    ///   - [`additional_authentication_providers(Vec<AdditionalAuthenticationProvider>)`](crate::client::fluent_builders::UpdateGraphqlApi::additional_authentication_providers) / [`set_additional_authentication_providers(Option<Vec<AdditionalAuthenticationProvider>>)`](crate::client::fluent_builders::UpdateGraphqlApi::set_additional_authentication_providers): <p>A list of additional authentication providers for the <code>GraphqlApi</code> API.</p>
    ///   - [`xray_enabled(bool)`](crate::client::fluent_builders::UpdateGraphqlApi::xray_enabled) / [`set_xray_enabled(bool)`](crate::client::fluent_builders::UpdateGraphqlApi::set_xray_enabled): <p>A flag indicating whether to use X-Ray tracing for the <code>GraphqlApi</code>.</p>
    ///   - [`lambda_authorizer_config(LambdaAuthorizerConfig)`](crate::client::fluent_builders::UpdateGraphqlApi::lambda_authorizer_config) / [`set_lambda_authorizer_config(Option<LambdaAuthorizerConfig>)`](crate::client::fluent_builders::UpdateGraphqlApi::set_lambda_authorizer_config): <p>Configuration for Lambda function authorization.</p>
                            /// - On success, responds with [`UpdateGraphqlApiOutput`](crate::output::UpdateGraphqlApiOutput) with field(s):
    ///   - [`graphql_api(Option<GraphqlApi>)`](crate::output::UpdateGraphqlApiOutput::graphql_api): <p>The updated <code>GraphqlApi</code> object.</p>
                            /// - On failure, responds with [`SdkError<UpdateGraphqlApiError>`](crate::error::UpdateGraphqlApiError)
    pub fn update_graphql_api(&self) -> crate::client::fluent_builders::UpdateGraphqlApi {
                                crate::client::fluent_builders::UpdateGraphqlApi::new(self.handle.clone())
                            }
}

