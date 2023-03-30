// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_container_state_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ContainerStateChange) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.container_name {
        object.key("containerName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.image_digest {
        object.key("imageDigest").string(var_2.as_str());
    }
    if let Some(var_3) = &input.runtime_id {
        object.key("runtimeId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.exit_code {
        object.key("exitCode").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    if let Some(var_5) = &input.network_bindings {
        let mut array_6 = object.key("networkBindings").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_network_binding::ser_network_binding(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.reason {
        object.key("reason").string(var_9.as_str());
    }
    if let Some(var_10) = &input.status {
        object.key("status").string(var_10.as_str());
    }
    Ok(())
}

