// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_create_transit_gateway_connect_request_options(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::CreateTransitGatewayConnectRequestOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Protocol");
    if let Some(var_2) = &input.protocol {
        scope_1.string(var_2.as_str());
    }
    Ok(())
}

