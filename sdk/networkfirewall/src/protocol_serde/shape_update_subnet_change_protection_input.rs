// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_subnet_change_protection_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSubnetChangeProtectionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.update_token {
        object.key("UpdateToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.firewall_arn {
        object.key("FirewallArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.firewall_name {
        object.key("FirewallName").string(var_3.as_str());
    }
     {
        object.key("SubnetChangeProtection").boolean(input.subnet_change_protection);
    }
    Ok(())
}

