// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_addon_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAddonInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.addon_name {
        object.key("addonName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.addon_version {
        object.key("addonVersion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.client_request_token {
        object.key("clientRequestToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.configuration_values {
        object.key("configurationValues").string(var_4.as_str());
    }
    if let Some(var_5) = &input.resolve_conflicts {
        object.key("resolveConflicts").string(var_5.as_str());
    }
    if let Some(var_6) = &input.service_account_role_arn {
        object.key("serviceAccountRoleArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("tags").start_object();
        for (key_9, value_10) in var_7 {
             {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}

