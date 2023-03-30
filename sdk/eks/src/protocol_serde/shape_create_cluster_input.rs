// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_cluster_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateClusterInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("clientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.encryption_config {
        let mut array_3 = object.key("encryptionConfig").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_encryption_config::ser_encryption_config(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.kubernetes_network_config {
        #[allow(unused_mut)]
        let mut object_7 = object.key("kubernetesNetworkConfig").start_object();
        crate::protocol_serde::shape_kubernetes_network_config_request::ser_kubernetes_network_config_request(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.logging {
        #[allow(unused_mut)]
        let mut object_9 = object.key("logging").start_object();
        crate::protocol_serde::shape_logging::ser_logging(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.name {
        object.key("name").string(var_10.as_str());
    }
    if let Some(var_11) = &input.outpost_config {
        #[allow(unused_mut)]
        let mut object_12 = object.key("outpostConfig").start_object();
        crate::protocol_serde::shape_outpost_config_request::ser_outpost_config_request(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.resources_vpc_config {
        #[allow(unused_mut)]
        let mut object_14 = object.key("resourcesVpcConfig").start_object();
        crate::protocol_serde::shape_vpc_config_request::ser_vpc_config_request(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.role_arn {
        object.key("roleArn").string(var_15.as_str());
    }
    if let Some(var_16) = &input.tags {
        #[allow(unused_mut)]
        let mut object_17 = object.key("tags").start_object();
        for (key_18, value_19) in var_16 {
             {
                object_17.key(key_18.as_str()).string(value_19.as_str());
            }
        }
        object_17.finish();
    }
    if let Some(var_20) = &input.version {
        object.key("version").string(var_20.as_str());
    }
    Ok(())
}

