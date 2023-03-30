// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRole`](crate::client::fluent_builders::GetRole) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`role_name(impl Into<String>)`](crate::client::fluent_builders::GetRole::role_name) / [`set_role_name(Option<String>)`](crate::client::fluent_builders::GetRole::set_role_name): <p>The name of the IAM role to get information about.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
                            /// - On success, responds with [`GetRoleOutput`](crate::output::GetRoleOutput) with field(s):
    ///   - [`role(Option<Role>)`](crate::output::GetRoleOutput::role): <p>A structure containing details about the IAM role.</p>
                            /// - On failure, responds with [`SdkError<GetRoleError>`](crate::error::GetRoleError)
    pub fn get_role(&self) -> crate::client::fluent_builders::GetRole {
                                crate::client::fluent_builders::GetRole::new(self.handle.clone())
                            }
}

