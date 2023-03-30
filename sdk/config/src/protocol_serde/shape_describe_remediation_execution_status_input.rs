// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_remediation_execution_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeRemediationExecutionStatusInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.config_rule_name {
        object.key("ConfigRuleName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_keys {
        let mut array_3 = object.key("ResourceKeys").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_resource_key::ser_resource_key(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if input.limit != 0 {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.limit).into()));
    }
    if let Some(var_6) = &input.next_token {
        object.key("NextToken").string(var_6.as_str());
    }
    Ok(())
}

