// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateSkillGroup`](crate::client::fluent_builders::UpdateSkillGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`skill_group_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateSkillGroup::skill_group_arn) / [`set_skill_group_arn(Option<String>)`](crate::client::fluent_builders::UpdateSkillGroup::set_skill_group_arn): <p>The ARN of the skill group to update. </p>
    ///   - [`skill_group_name(impl Into<String>)`](crate::client::fluent_builders::UpdateSkillGroup::skill_group_name) / [`set_skill_group_name(Option<String>)`](crate::client::fluent_builders::UpdateSkillGroup::set_skill_group_name): <p>The updated name for the skill group.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateSkillGroup::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateSkillGroup::set_description): <p>The updated description for the skill group.</p>
                            /// - On success, responds with [`UpdateSkillGroupOutput`](crate::output::UpdateSkillGroupOutput)
                            /// - On failure, responds with [`SdkError<UpdateSkillGroupError>`](crate::error::UpdateSkillGroupError)
    pub fn update_skill_group(&self) -> crate::client::fluent_builders::UpdateSkillGroup {
                                crate::client::fluent_builders::UpdateSkillGroup::new(self.handle.clone())
                            }
}

