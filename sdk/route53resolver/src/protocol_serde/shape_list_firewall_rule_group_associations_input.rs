// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_firewall_rule_group_associations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListFirewallRuleGroupAssociationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.firewall_rule_group_id {
        object.key("FirewallRuleGroupId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.vpc_id {
        object.key("VpcId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.priority {
        object.key("Priority").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.status {
        object.key("Status").string(var_4.as_str());
    }
    if let Some(var_5) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    if let Some(var_6) = &input.next_token {
        object.key("NextToken").string(var_6.as_str());
    }
    Ok(())
}

