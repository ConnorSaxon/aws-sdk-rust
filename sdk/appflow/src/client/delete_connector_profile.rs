// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteConnectorProfile`](crate::client::fluent_builders::DeleteConnectorProfile) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`connector_profile_name(impl Into<String>)`](crate::client::fluent_builders::DeleteConnectorProfile::connector_profile_name) / [`set_connector_profile_name(Option<String>)`](crate::client::fluent_builders::DeleteConnectorProfile::set_connector_profile_name): <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in your account. </p>
    ///   - [`force_delete(bool)`](crate::client::fluent_builders::DeleteConnectorProfile::force_delete) / [`set_force_delete(bool)`](crate::client::fluent_builders::DeleteConnectorProfile::set_force_delete): <p> Indicates whether Amazon AppFlow should delete the profile, even if it is currently in use in one or more flows. </p>
                            /// - On success, responds with [`DeleteConnectorProfileOutput`](crate::output::DeleteConnectorProfileOutput)
                            /// - On failure, responds with [`SdkError<DeleteConnectorProfileError>`](crate::error::DeleteConnectorProfileError)
    pub fn delete_connector_profile(&self) -> crate::client::fluent_builders::DeleteConnectorProfile {
                                crate::client::fluent_builders::DeleteConnectorProfile::new(self.handle.clone())
                            }
}

