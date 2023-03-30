// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteMember`](crate::client::fluent_builders::DeleteMember) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`collaboration_identifier(impl Into<String>)`](crate::client::fluent_builders::DeleteMember::collaboration_identifier) / [`set_collaboration_identifier(Option<String>)`](crate::client::fluent_builders::DeleteMember::set_collaboration_identifier): <p>The unique identifier for the associated collaboration.</p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::DeleteMember::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::DeleteMember::set_account_id): <p>The account ID of the member to remove.</p>
                            /// - On success, responds with [`DeleteMemberOutput`](crate::output::DeleteMemberOutput)
                            /// - On failure, responds with [`SdkError<DeleteMemberError>`](crate::error::DeleteMemberError)
    pub fn delete_member(&self) -> crate::client::fluent_builders::DeleteMember {
                                crate::client::fluent_builders::DeleteMember::new(self.handle.clone())
                            }
}

