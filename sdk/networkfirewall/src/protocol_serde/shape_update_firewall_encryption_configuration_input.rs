// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_firewall_encryption_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateFirewallEncryptionConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.update_token {
        object.key("UpdateToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.firewall_arn {
        object.key("FirewallArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.firewall_name {
        object.key("FirewallName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.encryption_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("EncryptionConfiguration").start_object();
        crate::protocol_serde::shape_encryption_configuration::ser_encryption_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

