// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.duration_seconds {
        object.key("durationSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    if let Some(var_2) = &input.managed_policy_arns {
        let mut array_3 = object.key("managedPolicyArns").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.name {
        object.key("name").string(var_5.as_str());
    }
    if let Some(var_6) = &input.role_arns {
        let mut array_7 = object.key("roleArns").start_array();
        for item_8 in var_6 {
             {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.session_policy {
        object.key("sessionPolicy").string(var_9.as_str());
    }
    Ok(())
}

