// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_firewall_domain_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteFirewallDomainListInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.firewall_domain_list_id {
        object.key("FirewallDomainListId").string(var_1.as_str());
    }
    Ok(())
}

