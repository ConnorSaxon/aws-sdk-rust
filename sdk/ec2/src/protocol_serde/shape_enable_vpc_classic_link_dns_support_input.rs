// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_enable_vpc_classic_link_dns_support_input_input(input: &crate::input::EnableVpcClassicLinkDnsSupportInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "EnableVpcClassicLinkDnsSupport", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("VpcId");
    if let Some(var_2) = &input.vpc_id {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

