// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_compliance_by_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeComplianceByResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_type {
        object.key("ResourceType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_id {
        object.key("ResourceId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.compliance_types {
        let mut array_4 = object.key("ComplianceTypes").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if input.limit != 0 {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.limit).into()));
    }
    if let Some(var_6) = &input.next_token {
        object.key("NextToken").string(var_6.as_str());
    }
    Ok(())
}

