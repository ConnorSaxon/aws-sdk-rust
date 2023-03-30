// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_orderable_db_instance_options_input_input(input: &crate::input::DescribeOrderableDbInstanceOptionsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeOrderableDBInstanceOptions", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Engine");
    if let Some(var_2) = &input.engine {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("EngineVersion");
    if let Some(var_4) = &input.engine_version {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DBInstanceClass");
    if let Some(var_6) = &input.db_instance_class {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("LicenseModel");
    if let Some(var_8) = &input.license_model {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("AvailabilityZoneGroup");
    if let Some(var_10) = &input.availability_zone_group {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Vpc");
    if let Some(var_12) = &input.vpc {
        scope_11.boolean(*var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("Filters");
    if let Some(var_14) = &input.filters {
        let mut list_16 = scope_13.start_list(false, Some("Filter"));
        for item_15 in var_14 {
            #[allow(unused_mut)]
            let mut entry_17 = list_16.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_17, item_15)?;
        }
        list_16.finish();
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("MaxRecords");
    if let Some(var_19) = &input.max_records {
        scope_18.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_19).into()));
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("Marker");
    if let Some(var_21) = &input.marker {
        scope_20.string(var_21);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

