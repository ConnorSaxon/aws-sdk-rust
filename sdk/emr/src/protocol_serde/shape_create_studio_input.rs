// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_studio_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStudioInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.auth_mode {
        object.key("AuthMode").string(var_3.as_str());
    }
    if let Some(var_4) = &input.vpc_id {
        object.key("VpcId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.subnet_ids {
        let mut array_6 = object.key("SubnetIds").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.service_role {
        object.key("ServiceRole").string(var_8.as_str());
    }
    if let Some(var_9) = &input.user_role {
        object.key("UserRole").string(var_9.as_str());
    }
    if let Some(var_10) = &input.workspace_security_group_id {
        object.key("WorkspaceSecurityGroupId").string(var_10.as_str());
    }
    if let Some(var_11) = &input.engine_security_group_id {
        object.key("EngineSecurityGroupId").string(var_11.as_str());
    }
    if let Some(var_12) = &input.default_s3_location {
        object.key("DefaultS3Location").string(var_12.as_str());
    }
    if let Some(var_13) = &input.idp_auth_url {
        object.key("IdpAuthUrl").string(var_13.as_str());
    }
    if let Some(var_14) = &input.idp_relay_state_parameter_name {
        object.key("IdpRelayStateParameterName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.tags {
        let mut array_16 = object.key("Tags").start_array();
        for item_17 in var_15 {
             {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    Ok(())
}

