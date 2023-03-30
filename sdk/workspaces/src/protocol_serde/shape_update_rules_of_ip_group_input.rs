// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_rules_of_ip_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateRulesOfIpGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.group_id {
        object.key("GroupId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.user_rules {
        let mut array_3 = object.key("UserRules").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_ip_rule_item::ser_ip_rule_item(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

