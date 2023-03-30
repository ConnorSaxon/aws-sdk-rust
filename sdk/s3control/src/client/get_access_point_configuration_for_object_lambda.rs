// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetAccessPointConfigurationForObjectLambda`](crate::client::fluent_builders::GetAccessPointConfigurationForObjectLambda) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::GetAccessPointConfigurationForObjectLambda::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::GetAccessPointConfigurationForObjectLambda::set_account_id): <p>The account ID for the account that owns the specified Object Lambda Access Point.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::GetAccessPointConfigurationForObjectLambda::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::GetAccessPointConfigurationForObjectLambda::set_name): <p>The name of the Object Lambda Access Point you want to return the configuration for.</p>
                            /// - On success, responds with [`GetAccessPointConfigurationForObjectLambdaOutput`](crate::output::GetAccessPointConfigurationForObjectLambdaOutput) with field(s):
    ///   - [`configuration(Option<ObjectLambdaConfiguration>)`](crate::output::GetAccessPointConfigurationForObjectLambdaOutput::configuration): <p>Object Lambda Access Point configuration document.</p>
                            /// - On failure, responds with [`SdkError<GetAccessPointConfigurationForObjectLambdaError>`](crate::error::GetAccessPointConfigurationForObjectLambdaError)
    pub fn get_access_point_configuration_for_object_lambda(&self) -> crate::client::fluent_builders::GetAccessPointConfigurationForObjectLambda {
                                crate::client::fluent_builders::GetAccessPointConfigurationForObjectLambda::new(self.handle.clone())
                            }
}

