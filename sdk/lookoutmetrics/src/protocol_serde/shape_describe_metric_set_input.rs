// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_metric_set_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeMetricSetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.metric_set_arn {
        object.key("MetricSetArn").string(var_1.as_str());
    }
    Ok(())
}

