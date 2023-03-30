// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RemoveUserFromGroup`](crate::client::fluent_builders::RemoveUserFromGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`group_name(impl Into<String>)`](crate::client::fluent_builders::RemoveUserFromGroup::group_name) / [`set_group_name(Option<String>)`](crate::client::fluent_builders::RemoveUserFromGroup::set_group_name): <p>The name of the group to update.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`user_name(impl Into<String>)`](crate::client::fluent_builders::RemoveUserFromGroup::user_name) / [`set_user_name(Option<String>)`](crate::client::fluent_builders::RemoveUserFromGroup::set_user_name): <p>The name of the user to remove.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
                            /// - On success, responds with [`RemoveUserFromGroupOutput`](crate::output::RemoveUserFromGroupOutput)
                            /// - On failure, responds with [`SdkError<RemoveUserFromGroupError>`](crate::error::RemoveUserFromGroupError)
    pub fn remove_user_from_group(&self) -> crate::client::fluent_builders::RemoveUserFromGroup {
                                crate::client::fluent_builders::RemoveUserFromGroup::new(self.handle.clone())
                            }
}

