// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListRooms`](crate::client::fluent_builders::ListRooms) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListRooms::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::ListRooms::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::ListRooms::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`member_id(impl Into<String>)`](crate::client::fluent_builders::ListRooms::member_id) / [`set_member_id(Option<String>)`](crate::client::fluent_builders::ListRooms::set_member_id): <p>The member ID (user ID or bot ID).</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListRooms::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListRooms::set_max_results): <p>The maximum number of results to return in a single call.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListRooms::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListRooms::set_next_token): <p>The token to use to retrieve the next page of results.</p>
                            /// - On success, responds with [`ListRoomsOutput`](crate::output::ListRoomsOutput) with field(s):
    ///   - [`rooms(Option<Vec<Room>>)`](crate::output::ListRoomsOutput::rooms): <p>The room details.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListRoomsOutput::next_token): <p>The token to use to retrieve the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListRoomsError>`](crate::error::ListRoomsError)
    pub fn list_rooms(&self) -> crate::client::fluent_builders::ListRooms {
                                crate::client::fluent_builders::ListRooms::new(self.handle.clone())
                            }
}

