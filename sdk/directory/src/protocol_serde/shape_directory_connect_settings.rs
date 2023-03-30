// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_directory_connect_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DirectoryConnectSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.vpc_id {
        object.key("VpcId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.subnet_ids {
        let mut array_3 = object.key("SubnetIds").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.customer_dns_ips {
        let mut array_6 = object.key("CustomerDnsIps").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.customer_user_name {
        object.key("CustomerUserName").string(var_8.as_str());
    }
    Ok(())
}

