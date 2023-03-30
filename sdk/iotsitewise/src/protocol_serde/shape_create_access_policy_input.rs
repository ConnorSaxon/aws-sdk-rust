// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_access_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAccessPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.access_policy_identity {
        #[allow(unused_mut)]
        let mut object_2 = object.key("accessPolicyIdentity").start_object();
        crate::protocol_serde::shape_identity::ser_identity(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.access_policy_permission {
        object.key("accessPolicyPermission").string(var_3.as_str());
    }
    if let Some(var_4) = &input.access_policy_resource {
        #[allow(unused_mut)]
        let mut object_5 = object.key("accessPolicyResource").start_object();
        crate::protocol_serde::shape_resource::ser_resource(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.client_token {
        object.key("clientToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("tags").start_object();
        for (key_9, value_10) in var_7 {
             {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}

