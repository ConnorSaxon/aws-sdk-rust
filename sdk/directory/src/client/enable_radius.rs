// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`EnableRadius`](crate::client::fluent_builders::EnableRadius) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::client::fluent_builders::EnableRadius::directory_id) / [`set_directory_id(Option<String>)`](crate::client::fluent_builders::EnableRadius::set_directory_id): <p>The identifier of the directory for which to enable MFA.</p>
    ///   - [`radius_settings(RadiusSettings)`](crate::client::fluent_builders::EnableRadius::radius_settings) / [`set_radius_settings(Option<RadiusSettings>)`](crate::client::fluent_builders::EnableRadius::set_radius_settings): <p>A <code>RadiusSettings</code> object that contains information about the RADIUS server.</p>
                            /// - On success, responds with [`EnableRadiusOutput`](crate::output::EnableRadiusOutput)
                            /// - On failure, responds with [`SdkError<EnableRadiusError>`](crate::error::EnableRadiusError)
    pub fn enable_radius(&self) -> crate::client::fluent_builders::EnableRadius {
                                crate::client::fluent_builders::EnableRadius::new(self.handle.clone())
                            }
}

