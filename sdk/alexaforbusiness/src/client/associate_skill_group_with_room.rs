// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateSkillGroupWithRoom`](crate::client::fluent_builders::AssociateSkillGroupWithRoom) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`skill_group_arn(impl Into<String>)`](crate::client::fluent_builders::AssociateSkillGroupWithRoom::skill_group_arn) / [`set_skill_group_arn(Option<String>)`](crate::client::fluent_builders::AssociateSkillGroupWithRoom::set_skill_group_arn): <p>The ARN of the skill group to associate with a room. Required.</p>
    ///   - [`room_arn(impl Into<String>)`](crate::client::fluent_builders::AssociateSkillGroupWithRoom::room_arn) / [`set_room_arn(Option<String>)`](crate::client::fluent_builders::AssociateSkillGroupWithRoom::set_room_arn): <p>The ARN of the room with which to associate the skill group. Required.</p>
                            /// - On success, responds with [`AssociateSkillGroupWithRoomOutput`](crate::output::AssociateSkillGroupWithRoomOutput)
                            /// - On failure, responds with [`SdkError<AssociateSkillGroupWithRoomError>`](crate::error::AssociateSkillGroupWithRoomError)
    pub fn associate_skill_group_with_room(&self) -> crate::client::fluent_builders::AssociateSkillGroupWithRoom {
                                crate::client::fluent_builders::AssociateSkillGroupWithRoom::new(self.handle.clone())
                            }
}

