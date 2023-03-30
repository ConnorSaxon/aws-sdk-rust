// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_rule_group_metadata_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeRuleGroupMetadataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.rule_group_name {
        object.key("RuleGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.rule_group_arn {
        object.key("RuleGroupArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.r#type {
        object.key("Type").string(var_3.as_str());
    }
    Ok(())
}

