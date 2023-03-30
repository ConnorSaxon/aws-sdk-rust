// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ApproveSkill`](crate::client::fluent_builders::ApproveSkill) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`skill_id(impl Into<String>)`](crate::client::fluent_builders::ApproveSkill::skill_id) / [`set_skill_id(Option<String>)`](crate::client::fluent_builders::ApproveSkill::set_skill_id): <p>The unique identifier of the skill.</p>
                            /// - On success, responds with [`ApproveSkillOutput`](crate::output::ApproveSkillOutput)
                            /// - On failure, responds with [`SdkError<ApproveSkillError>`](crate::error::ApproveSkillError)
    pub fn approve_skill(&self) -> crate::client::fluent_builders::ApproveSkill {
                                crate::client::fluent_builders::ApproveSkill::new(self.handle.clone())
                            }
}

