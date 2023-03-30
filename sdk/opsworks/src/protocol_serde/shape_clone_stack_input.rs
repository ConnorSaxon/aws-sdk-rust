// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_clone_stack_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CloneStackInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.source_stack_id {
        object.key("SourceStackId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.region {
        object.key("Region").string(var_3.as_str());
    }
    if let Some(var_4) = &input.vpc_id {
        object.key("VpcId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Attributes").start_object();
        for (key_7, value_8) in var_5 {
             {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    if let Some(var_9) = &input.service_role_arn {
        object.key("ServiceRoleArn").string(var_9.as_str());
    }
    if let Some(var_10) = &input.default_instance_profile_arn {
        object.key("DefaultInstanceProfileArn").string(var_10.as_str());
    }
    if let Some(var_11) = &input.default_os {
        object.key("DefaultOs").string(var_11.as_str());
    }
    if let Some(var_12) = &input.hostname_theme {
        object.key("HostnameTheme").string(var_12.as_str());
    }
    if let Some(var_13) = &input.default_availability_zone {
        object.key("DefaultAvailabilityZone").string(var_13.as_str());
    }
    if let Some(var_14) = &input.default_subnet_id {
        object.key("DefaultSubnetId").string(var_14.as_str());
    }
    if let Some(var_15) = &input.custom_json {
        object.key("CustomJson").string(var_15.as_str());
    }
    if let Some(var_16) = &input.configuration_manager {
        #[allow(unused_mut)]
        let mut object_17 = object.key("ConfigurationManager").start_object();
        crate::protocol_serde::shape_stack_configuration_manager::ser_stack_configuration_manager(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.chef_configuration {
        #[allow(unused_mut)]
        let mut object_19 = object.key("ChefConfiguration").start_object();
        crate::protocol_serde::shape_chef_configuration::ser_chef_configuration(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.use_custom_cookbooks {
        object.key("UseCustomCookbooks").boolean(*var_20);
    }
    if let Some(var_21) = &input.use_opsworks_security_groups {
        object.key("UseOpsworksSecurityGroups").boolean(*var_21);
    }
    if let Some(var_22) = &input.custom_cookbooks_source {
        #[allow(unused_mut)]
        let mut object_23 = object.key("CustomCookbooksSource").start_object();
        crate::protocol_serde::shape_source::ser_source(&mut object_23, var_22)?;
        object_23.finish();
    }
    if let Some(var_24) = &input.default_ssh_key_name {
        object.key("DefaultSshKeyName").string(var_24.as_str());
    }
    if let Some(var_25) = &input.clone_permissions {
        object.key("ClonePermissions").boolean(*var_25);
    }
    if let Some(var_26) = &input.clone_app_ids {
        let mut array_27 = object.key("CloneAppIds").start_array();
        for item_28 in var_26 {
             {
                array_27.value().string(item_28.as_str());
            }
        }
        array_27.finish();
    }
    if let Some(var_29) = &input.default_root_device_type {
        object.key("DefaultRootDeviceType").string(var_29.as_str());
    }
    if let Some(var_30) = &input.agent_version {
        object.key("AgentVersion").string(var_30.as_str());
    }
    Ok(())
}

