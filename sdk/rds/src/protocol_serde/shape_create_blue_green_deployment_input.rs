// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_blue_green_deployment_input_input(input: &crate::input::CreateBlueGreenDeploymentInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateBlueGreenDeployment", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("BlueGreenDeploymentName");
    if let Some(var_2) = &input.blue_green_deployment_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Source");
    if let Some(var_4) = &input.source {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TargetEngineVersion");
    if let Some(var_6) = &input.target_engine_version {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("TargetDBParameterGroupName");
    if let Some(var_8) = &input.target_db_parameter_group_name {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("TargetDBClusterParameterGroupName");
    if let Some(var_10) = &input.target_db_cluster_parameter_group_name {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Tags");
    if let Some(var_12) = &input.tags {
        let mut list_14 = scope_11.start_list(false, Some("Tag"));
        for item_13 in var_12 {
            #[allow(unused_mut)]
            let mut entry_15 = list_14.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_15, item_13)?;
        }
        list_14.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

