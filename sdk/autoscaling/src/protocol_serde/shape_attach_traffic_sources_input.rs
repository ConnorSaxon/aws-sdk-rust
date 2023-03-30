// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attach_traffic_sources_input_input(input: &crate::input::AttachTrafficSourcesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AttachTrafficSources", "2011-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AutoScalingGroupName");
    if let Some(var_2) = &input.auto_scaling_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TrafficSources");
    if let Some(var_4) = &input.traffic_sources {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            crate::protocol_serde::shape_traffic_source_identifier::ser_traffic_source_identifier(entry_7, item_5)?;
        }
        list_6.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

