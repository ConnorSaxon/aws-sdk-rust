// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_reserved_node_exchange_configuration_options_input_input(input: &crate::input::GetReservedNodeExchangeConfigurationOptionsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "GetReservedNodeExchangeConfigurationOptions", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ActionType");
    if let Some(var_2) = &input.action_type {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ClusterIdentifier");
    if let Some(var_4) = &input.cluster_identifier {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("SnapshotIdentifier");
    if let Some(var_6) = &input.snapshot_identifier {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("MaxRecords");
    if let Some(var_8) = &input.max_records {
        scope_7.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Marker");
    if let Some(var_10) = &input.marker {
        scope_9.string(var_10);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

