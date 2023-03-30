// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SetIdentityDkimEnabled`](crate::client::fluent_builders::SetIdentityDkimEnabled) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity(impl Into<String>)`](crate::client::fluent_builders::SetIdentityDkimEnabled::identity) / [`set_identity(Option<String>)`](crate::client::fluent_builders::SetIdentityDkimEnabled::set_identity): <p>The identity for which DKIM signing should be enabled or disabled.</p>
    ///   - [`dkim_enabled(bool)`](crate::client::fluent_builders::SetIdentityDkimEnabled::dkim_enabled) / [`set_dkim_enabled(bool)`](crate::client::fluent_builders::SetIdentityDkimEnabled::set_dkim_enabled): <p>Sets whether DKIM signing is enabled for an identity. Set to <code>true</code> to enable DKIM signing for this identity; <code>false</code> to disable it. </p>
                            /// - On success, responds with [`SetIdentityDkimEnabledOutput`](crate::output::SetIdentityDkimEnabledOutput)
                            /// - On failure, responds with [`SdkError<SetIdentityDkimEnabledError>`](crate::error::SetIdentityDkimEnabledError)
    pub fn set_identity_dkim_enabled(&self) -> crate::client::fluent_builders::SetIdentityDkimEnabled {
                                crate::client::fluent_builders::SetIdentityDkimEnabled::new(self.handle.clone())
                            }
}

