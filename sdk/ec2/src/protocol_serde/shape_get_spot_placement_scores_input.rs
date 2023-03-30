// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_spot_placement_scores_input_input(input: &crate::input::GetSpotPlacementScoresInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "GetSpotPlacementScores", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("InstanceType");
    if let Some(var_2) = &input.instance_types {
        let mut list_4 = scope_1.start_list(true, None);
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("TargetCapacity");
    if let Some(var_7) = &input.target_capacity {
        scope_6.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("TargetCapacityUnitType");
    if let Some(var_9) = &input.target_capacity_unit_type {
        scope_8.string(var_9.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("SingleAvailabilityZone");
    if let Some(var_11) = &input.single_availability_zone {
        scope_10.boolean(*var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("RegionName");
    if let Some(var_13) = &input.region_names {
        let mut list_15 = scope_12.start_list(true, None);
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            entry_16.string(item_14);
        }
        list_15.finish();
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("InstanceRequirementsWithMetadata");
    if let Some(var_18) = &input.instance_requirements_with_metadata {
        crate::protocol_serde::shape_instance_requirements_with_metadata_request::ser_instance_requirements_with_metadata_request(scope_17, var_18)?;
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("DryRun");
    if let Some(var_20) = &input.dry_run {
        scope_19.boolean(*var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("MaxResults");
    if let Some(var_22) = &input.max_results {
        scope_21.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_22).into()));
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("NextToken");
    if let Some(var_24) = &input.next_token {
        scope_23.string(var_24);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

