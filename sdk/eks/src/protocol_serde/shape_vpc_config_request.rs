// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_vpc_config_request(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VpcConfigRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.subnet_ids {
        let mut array_2 = object.key("subnetIds").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.security_group_ids {
        let mut array_5 = object.key("securityGroupIds").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.endpoint_public_access {
        object.key("endpointPublicAccess").boolean(*var_7);
    }
    if let Some(var_8) = &input.endpoint_private_access {
        object.key("endpointPrivateAccess").boolean(*var_8);
    }
    if let Some(var_9) = &input.public_access_cidrs {
        let mut array_10 = object.key("publicAccessCidrs").start_array();
        for item_11 in var_9 {
             {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    Ok(())
}

