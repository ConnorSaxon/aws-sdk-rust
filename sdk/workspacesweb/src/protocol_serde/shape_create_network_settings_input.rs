// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_network_settings_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateNetworkSettingsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.security_group_ids {
        let mut array_3 = object.key("securityGroupIds").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.subnet_ids {
        let mut array_6 = object.key("subnetIds").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("tags").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.vpc_id {
        object.key("vpcId").string(var_12.as_str());
    }
    Ok(())
}

