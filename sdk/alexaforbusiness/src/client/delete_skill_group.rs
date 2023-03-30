// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteSkillGroup`](crate::client::fluent_builders::DeleteSkillGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`skill_group_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteSkillGroup::skill_group_arn) / [`set_skill_group_arn(Option<String>)`](crate::client::fluent_builders::DeleteSkillGroup::set_skill_group_arn): <p>The ARN of the skill group to delete. Required.</p>
                            /// - On success, responds with [`DeleteSkillGroupOutput`](crate::output::DeleteSkillGroupOutput)
                            /// - On failure, responds with [`SdkError<DeleteSkillGroupError>`](crate::error::DeleteSkillGroupError)
    pub fn delete_skill_group(&self) -> crate::client::fluent_builders::DeleteSkillGroup {
                                crate::client::fluent_builders::DeleteSkillGroup::new(self.handle.clone())
                            }
}

