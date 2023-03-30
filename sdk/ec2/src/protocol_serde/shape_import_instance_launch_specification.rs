// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_import_instance_launch_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ImportInstanceLaunchSpecification) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AdditionalInfo");
    if let Some(var_2) = &input.additional_info {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Architecture");
    if let Some(var_4) = &input.architecture {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("GroupId");
    if let Some(var_6) = &input.group_ids {
        let mut list_8 = scope_5.start_list(true, Some("SecurityGroupId"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            entry_9.string(item_7);
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("GroupName");
    if let Some(var_11) = &input.group_names {
        let mut list_13 = scope_10.start_list(true, Some("SecurityGroup"));
        for item_12 in var_11 {
            #[allow(unused_mut)]
            let mut entry_14 = list_13.entry();
            entry_14.string(item_12);
        }
        list_13.finish();
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("InstanceInitiatedShutdownBehavior");
    if let Some(var_16) = &input.instance_initiated_shutdown_behavior {
        scope_15.string(var_16.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("InstanceType");
    if let Some(var_18) = &input.instance_type {
        scope_17.string(var_18.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("Monitoring");
    if let Some(var_20) = &input.monitoring {
        scope_19.boolean(*var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Placement");
    if let Some(var_22) = &input.placement {
        crate::protocol_serde::shape_placement::ser_placement(scope_21, var_22)?;
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("PrivateIpAddress");
    if let Some(var_24) = &input.private_ip_address {
        scope_23.string(var_24);
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("SubnetId");
    if let Some(var_26) = &input.subnet_id {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("UserData");
    if let Some(var_28) = &input.user_data {
        crate::protocol_serde::shape_user_data::ser_user_data(scope_27, var_28)?;
    }
    Ok(())
}

