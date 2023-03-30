// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_firewall_rule_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteFirewallRuleInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.firewall_rule_group_id {
        object.key("FirewallRuleGroupId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.firewall_domain_list_id {
        object.key("FirewallDomainListId").string(var_2.as_str());
    }
    Ok(())
}

