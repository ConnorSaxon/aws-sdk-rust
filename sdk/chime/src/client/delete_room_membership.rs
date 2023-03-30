// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRoomMembership`](crate::client::fluent_builders::DeleteRoomMembership) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::DeleteRoomMembership::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::DeleteRoomMembership::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`room_id(impl Into<String>)`](crate::client::fluent_builders::DeleteRoomMembership::room_id) / [`set_room_id(Option<String>)`](crate::client::fluent_builders::DeleteRoomMembership::set_room_id): <p>The room ID.</p>
    ///   - [`member_id(impl Into<String>)`](crate::client::fluent_builders::DeleteRoomMembership::member_id) / [`set_member_id(Option<String>)`](crate::client::fluent_builders::DeleteRoomMembership::set_member_id): <p>The member ID (user ID or bot ID).</p>
                            /// - On success, responds with [`DeleteRoomMembershipOutput`](crate::output::DeleteRoomMembershipOutput)
                            /// - On failure, responds with [`SdkError<DeleteRoomMembershipError>`](crate::error::DeleteRoomMembershipError)
    pub fn delete_room_membership(&self) -> crate::client::fluent_builders::DeleteRoomMembership {
                                crate::client::fluent_builders::DeleteRoomMembership::new(self.handle.clone())
                            }
}

