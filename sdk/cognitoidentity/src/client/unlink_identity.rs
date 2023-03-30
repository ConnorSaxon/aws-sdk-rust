// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UnlinkIdentity`](crate::client::fluent_builders::UnlinkIdentity) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity_id(impl Into<String>)`](crate::client::fluent_builders::UnlinkIdentity::identity_id) / [`set_identity_id(Option<String>)`](crate::client::fluent_builders::UnlinkIdentity::set_identity_id): <p>A unique identifier in the format REGION:GUID.</p>
    ///   - [`logins(HashMap<String, String>)`](crate::client::fluent_builders::UnlinkIdentity::logins) / [`set_logins(Option<HashMap<String, String>>)`](crate::client::fluent_builders::UnlinkIdentity::set_logins): <p>A set of optional name-value pairs that map provider names to provider tokens.</p>
    ///   - [`logins_to_remove(Vec<String>)`](crate::client::fluent_builders::UnlinkIdentity::logins_to_remove) / [`set_logins_to_remove(Option<Vec<String>>)`](crate::client::fluent_builders::UnlinkIdentity::set_logins_to_remove): <p>Provider names to unlink from this identity.</p>
                            /// - On success, responds with [`UnlinkIdentityOutput`](crate::output::UnlinkIdentityOutput)
                            /// - On failure, responds with [`SdkError<UnlinkIdentityError>`](crate::error::UnlinkIdentityError)
    pub fn unlink_identity(&self) -> crate::client::fluent_builders::UnlinkIdentity {
                                crate::client::fluent_builders::UnlinkIdentity::new(self.handle.clone())
                            }
}

