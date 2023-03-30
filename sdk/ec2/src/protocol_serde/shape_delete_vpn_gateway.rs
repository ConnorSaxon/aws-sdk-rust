// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_vpn_gateway_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteVpnGatewayOutput, crate::error::DeleteVpnGatewayError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteVpnGatewayError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::DeleteVpnGatewayError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_vpn_gateway_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteVpnGatewayOutput, crate::error::DeleteVpnGatewayError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_vpn_gateway_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

