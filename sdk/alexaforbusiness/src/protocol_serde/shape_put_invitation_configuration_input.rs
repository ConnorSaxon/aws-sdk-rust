// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_invitation_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutInvitationConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.organization_name {
        object.key("OrganizationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.contact_email {
        object.key("ContactEmail").string(var_2.as_str());
    }
    if let Some(var_3) = &input.private_skill_ids {
        let mut array_4 = object.key("PrivateSkillIds").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}

