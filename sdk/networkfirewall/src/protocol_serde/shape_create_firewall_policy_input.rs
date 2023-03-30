// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_firewall_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateFirewallPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.firewall_policy_name {
        object.key("FirewallPolicyName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.firewall_policy {
        #[allow(unused_mut)]
        let mut object_3 = object.key("FirewallPolicy").start_object();
        crate::protocol_serde::shape_firewall_policy::ser_firewall_policy(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if input.dry_run {
        object.key("DryRun").boolean(input.dry_run);
    }
    if let Some(var_9) = &input.encryption_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("EncryptionConfiguration").start_object();
        crate::protocol_serde::shape_encryption_configuration::ser_encryption_configuration(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}

