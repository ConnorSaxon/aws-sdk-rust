// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateSkillFromUsers`](crate::client::fluent_builders::DisassociateSkillFromUsers) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`skill_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateSkillFromUsers::skill_id) / [`set_skill_id(Option<String>)`](crate::client::fluent_builders::DisassociateSkillFromUsers::set_skill_id): <p> The private skill ID you want to make unavailable for enrolled users.</p>
                            /// - On success, responds with [`DisassociateSkillFromUsersOutput`](crate::output::DisassociateSkillFromUsersOutput)
                            /// - On failure, responds with [`SdkError<DisassociateSkillFromUsersError>`](crate::error::DisassociateSkillFromUsersError)
    pub fn disassociate_skill_from_users(&self) -> crate::client::fluent_builders::DisassociateSkillFromUsers {
                                crate::client::fluent_builders::DisassociateSkillFromUsers::new(self.handle.clone())
                            }
}

