// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_firewall_rule_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetFirewallRuleGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.firewall_rule_group_id {
        object.key("FirewallRuleGroupId").string(var_1.as_str());
    }
    Ok(())
}

