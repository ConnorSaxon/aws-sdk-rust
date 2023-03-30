// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_firewall_rule_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateFirewallRuleInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.firewall_rule_group_id {
        object.key("FirewallRuleGroupId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.firewall_domain_list_id {
        object.key("FirewallDomainListId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.priority {
        object.key("Priority").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    if let Some(var_5) = &input.action {
        object.key("Action").string(var_5.as_str());
    }
    if let Some(var_6) = &input.block_response {
        object.key("BlockResponse").string(var_6.as_str());
    }
    if let Some(var_7) = &input.block_override_domain {
        object.key("BlockOverrideDomain").string(var_7.as_str());
    }
    if let Some(var_8) = &input.block_override_dns_type {
        object.key("BlockOverrideDnsType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.block_override_ttl {
        object.key("BlockOverrideTtl").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_9).into()));
    }
    if let Some(var_10) = &input.name {
        object.key("Name").string(var_10.as_str());
    }
    Ok(())
}

