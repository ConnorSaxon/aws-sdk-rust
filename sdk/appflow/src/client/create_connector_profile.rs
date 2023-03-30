// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateConnectorProfile`](crate::client::fluent_builders::CreateConnectorProfile) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`connector_profile_name(impl Into<String>)`](crate::client::fluent_builders::CreateConnectorProfile::connector_profile_name) / [`set_connector_profile_name(Option<String>)`](crate::client::fluent_builders::CreateConnectorProfile::set_connector_profile_name): <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in your Amazon Web Services account. </p>
    ///   - [`kms_arn(impl Into<String>)`](crate::client::fluent_builders::CreateConnectorProfile::kms_arn) / [`set_kms_arn(Option<String>)`](crate::client::fluent_builders::CreateConnectorProfile::set_kms_arn): <p> The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key. </p>
    ///   - [`connector_type(ConnectorType)`](crate::client::fluent_builders::CreateConnectorProfile::connector_type) / [`set_connector_type(Option<ConnectorType>)`](crate::client::fluent_builders::CreateConnectorProfile::set_connector_type): <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    ///   - [`connector_label(impl Into<String>)`](crate::client::fluent_builders::CreateConnectorProfile::connector_label) / [`set_connector_label(Option<String>)`](crate::client::fluent_builders::CreateConnectorProfile::set_connector_label): <p>The label of the connector. The label is unique for each <code>ConnectorRegistration</code> in your Amazon Web Services account. Only needed if calling for CUSTOMCONNECTOR connector type/.</p>
    ///   - [`connection_mode(ConnectionMode)`](crate::client::fluent_builders::CreateConnectorProfile::connection_mode) / [`set_connection_mode(Option<ConnectionMode>)`](crate::client::fluent_builders::CreateConnectorProfile::set_connection_mode): <p> Indicates the connection mode and specifies whether it is public or private. Private flows use Amazon Web Services PrivateLink to route data over Amazon Web Services infrastructure without exposing it to the public internet. </p>
    ///   - [`connector_profile_config(ConnectorProfileConfig)`](crate::client::fluent_builders::CreateConnectorProfile::connector_profile_config) / [`set_connector_profile_config(Option<ConnectorProfileConfig>)`](crate::client::fluent_builders::CreateConnectorProfile::set_connector_profile_config): <p> Defines the connector-specific configuration and credentials. </p>
                            /// - On success, responds with [`CreateConnectorProfileOutput`](crate::output::CreateConnectorProfileOutput) with field(s):
    ///   - [`connector_profile_arn(Option<String>)`](crate::output::CreateConnectorProfileOutput::connector_profile_arn): <p> The Amazon Resource Name (ARN) of the connector profile. </p>
                            /// - On failure, responds with [`SdkError<CreateConnectorProfileError>`](crate::error::CreateConnectorProfileError)
    pub fn create_connector_profile(&self) -> crate::client::fluent_builders::CreateConnectorProfile {
                                crate::client::fluent_builders::CreateConnectorProfile::new(self.handle.clone())
                            }
}

