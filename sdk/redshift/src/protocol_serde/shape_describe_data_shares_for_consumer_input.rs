// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_data_shares_for_consumer_input_input(input: &crate::input::DescribeDataSharesForConsumerInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeDataSharesForConsumer", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ConsumerArn");
    if let Some(var_2) = &input.consumer_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Status");
    if let Some(var_4) = &input.status {
        scope_3.string(var_4.as_str());
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

