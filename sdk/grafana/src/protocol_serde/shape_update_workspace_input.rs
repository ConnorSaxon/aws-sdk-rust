// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_workspace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateWorkspaceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_access_type {
        object.key("accountAccessType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.organization_role_name {
        object.key("organizationRoleName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.permission_type {
        object.key("permissionType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.remove_vpc_configuration {
        object.key("removeVpcConfiguration").boolean(*var_4);
    }
    if let Some(var_5) = &input.stack_set_name {
        object.key("stackSetName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("vpcConfiguration").start_object();
        crate::protocol_serde::shape_vpc_configuration::ser_vpc_configuration(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.workspace_data_sources {
        let mut array_9 = object.key("workspaceDataSources").start_array();
        for item_10 in var_8 {
             {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.workspace_description {
        object.key("workspaceDescription").string(var_11.as_str());
    }
    if let Some(var_12) = &input.workspace_name {
        object.key("workspaceName").string(var_12.as_str());
    }
    if let Some(var_13) = &input.workspace_notification_destinations {
        let mut array_14 = object.key("workspaceNotificationDestinations").start_array();
        for item_15 in var_13 {
             {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.workspace_organizational_units {
        let mut array_17 = object.key("workspaceOrganizationalUnits").start_array();
        for item_18 in var_16 {
             {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.workspace_role_arn {
        object.key("workspaceRoleArn").string(var_19.as_str());
    }
    Ok(())
}

