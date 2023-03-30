// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_revoke_invitation_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RevokeInvitationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.user_arn {
        object.key("UserArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.enrollment_id {
        object.key("EnrollmentId").string(var_2.as_str());
    }
    Ok(())
}

