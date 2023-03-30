// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_ip_restriction_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateIpRestrictionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.enabled {
        object.key("Enabled").boolean(*var_1);
    }
    if let Some(var_2) = &input.ip_restriction_rule_map {
        #[allow(unused_mut)]
        let mut object_3 = object.key("IpRestrictionRuleMap").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    Ok(())
}

