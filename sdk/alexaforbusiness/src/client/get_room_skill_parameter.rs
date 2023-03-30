// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRoomSkillParameter`](crate::client::fluent_builders::GetRoomSkillParameter) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`room_arn(impl Into<String>)`](crate::client::fluent_builders::GetRoomSkillParameter::room_arn) / [`set_room_arn(Option<String>)`](crate::client::fluent_builders::GetRoomSkillParameter::set_room_arn): <p>The ARN of the room from which to get the room skill parameter details. </p>
    ///   - [`skill_id(impl Into<String>)`](crate::client::fluent_builders::GetRoomSkillParameter::skill_id) / [`set_skill_id(Option<String>)`](crate::client::fluent_builders::GetRoomSkillParameter::set_skill_id): <p>The ARN of the skill from which to get the room skill parameter details. Required.</p>
    ///   - [`parameter_key(impl Into<String>)`](crate::client::fluent_builders::GetRoomSkillParameter::parameter_key) / [`set_parameter_key(Option<String>)`](crate::client::fluent_builders::GetRoomSkillParameter::set_parameter_key): <p>The room skill parameter key for which to get details. Required.</p>
                            /// - On success, responds with [`GetRoomSkillParameterOutput`](crate::output::GetRoomSkillParameterOutput) with field(s):
    ///   - [`room_skill_parameter(Option<RoomSkillParameter>)`](crate::output::GetRoomSkillParameterOutput::room_skill_parameter): <p>The details of the room skill parameter requested. Required.</p>
                            /// - On failure, responds with [`SdkError<GetRoomSkillParameterError>`](crate::error::GetRoomSkillParameterError)
    pub fn get_room_skill_parameter(&self) -> crate::client::fluent_builders::GetRoomSkillParameter {
                                crate::client::fluent_builders::GetRoomSkillParameter::new(self.handle.clone())
                            }
}

