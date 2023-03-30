// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_slack_channel_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSlackChannelConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.channel_id {
        object.key("channelId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.channel_name {
        object.key("channelName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.channel_role_arn {
        object.key("channelRoleArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.notify_on_add_correspondence_to_case {
        object.key("notifyOnAddCorrespondenceToCase").boolean(*var_4);
    }
    if let Some(var_5) = &input.notify_on_case_severity {
        object.key("notifyOnCaseSeverity").string(var_5.as_str());
    }
    if let Some(var_6) = &input.notify_on_create_or_reopen_case {
        object.key("notifyOnCreateOrReopenCase").boolean(*var_6);
    }
    if let Some(var_7) = &input.notify_on_resolve_case {
        object.key("notifyOnResolveCase").boolean(*var_7);
    }
    if let Some(var_8) = &input.team_id {
        object.key("teamId").string(var_8.as_str());
    }
    Ok(())
}

