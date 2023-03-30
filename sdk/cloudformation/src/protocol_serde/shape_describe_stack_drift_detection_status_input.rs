// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_stack_drift_detection_status_input_input(input: &crate::input::DescribeStackDriftDetectionStatusInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeStackDriftDetectionStatus", "2010-05-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("StackDriftDetectionId");
    if let Some(var_2) = &input.stack_drift_detection_id {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

