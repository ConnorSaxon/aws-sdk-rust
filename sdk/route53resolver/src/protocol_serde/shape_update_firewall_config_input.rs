// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_firewall_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateFirewallConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_id {
        object.key("ResourceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.firewall_fail_open {
        object.key("FirewallFailOpen").string(var_2.as_str());
    }
    Ok(())
}

