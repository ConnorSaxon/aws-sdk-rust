// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteIdentities`](crate::client::fluent_builders::DeleteIdentities) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity_ids_to_delete(Vec<String>)`](crate::client::fluent_builders::DeleteIdentities::identity_ids_to_delete) / [`set_identity_ids_to_delete(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteIdentities::set_identity_ids_to_delete): <p>A list of 1-60 identities that you want to delete.</p>
                            /// - On success, responds with [`DeleteIdentitiesOutput`](crate::output::DeleteIdentitiesOutput) with field(s):
    ///   - [`unprocessed_identity_ids(Option<Vec<UnprocessedIdentityId>>)`](crate::output::DeleteIdentitiesOutput::unprocessed_identity_ids): <p>An array of UnprocessedIdentityId objects, each of which contains an ErrorCode and IdentityId.</p>
                            /// - On failure, responds with [`SdkError<DeleteIdentitiesError>`](crate::error::DeleteIdentitiesError)
    pub fn delete_identities(&self) -> crate::client::fluent_builders::DeleteIdentities {
                                crate::client::fluent_builders::DeleteIdentities::new(self.handle.clone())
                            }
}

