// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_protection_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateProtectionGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.protection_group_id {
        object.key("ProtectionGroupId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.aggregation {
        object.key("Aggregation").string(var_2.as_str());
    }
    if let Some(var_3) = &input.pattern {
        object.key("Pattern").string(var_3.as_str());
    }
    if let Some(var_4) = &input.resource_type {
        object.key("ResourceType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.members {
        let mut array_6 = object.key("Members").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    Ok(())
}

