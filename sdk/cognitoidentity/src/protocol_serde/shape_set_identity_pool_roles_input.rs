// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_set_identity_pool_roles_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SetIdentityPoolRolesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.roles {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Roles").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.role_mappings {
        #[allow(unused_mut)]
        let mut object_7 = object.key("RoleMappings").start_object();
        for (key_8, value_9) in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_10 = object_7.key(key_8.as_str()).start_object();
                crate::protocol_serde::shape_role_mapping::ser_role_mapping(&mut object_10, value_9)?;
                object_10.finish();
            }
        }
        object_7.finish();
    }
    Ok(())
}

