// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_load_balancer_attributes_input_input(input: &crate::input::DescribeLoadBalancerAttributesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeLoadBalancerAttributes", "2015-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LoadBalancerArn");
    if let Some(var_2) = &input.load_balancer_arn {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

