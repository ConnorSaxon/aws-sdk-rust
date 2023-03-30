// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_cluster_versions_input_input(input: &crate::input::DescribeClusterVersionsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeClusterVersions", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClusterVersion");
    if let Some(var_2) = &input.cluster_version {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ClusterParameterGroupFamily");
    if let Some(var_4) = &input.cluster_parameter_group_family {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MaxRecords");
    if let Some(var_6) = &input.max_records {
        scope_5.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Marker");
    if let Some(var_8) = &input.marker {
        scope_7.string(var_8);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

