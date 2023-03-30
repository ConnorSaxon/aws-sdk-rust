// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_accept_transit_gateway_peering_attachment_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AcceptTransitGatewayPeeringAttachmentOutput, crate::error::AcceptTransitGatewayPeeringAttachmentError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AcceptTransitGatewayPeeringAttachmentError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::AcceptTransitGatewayPeeringAttachmentError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_accept_transit_gateway_peering_attachment_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AcceptTransitGatewayPeeringAttachmentOutput, crate::error::AcceptTransitGatewayPeeringAttachmentError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::accept_transit_gateway_peering_attachment_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_accept_transit_gateway_peering_attachment::de_accept_transit_gateway_peering_attachment(response.body().as_ref(), output).map_err(crate::error::AcceptTransitGatewayPeeringAttachmentError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_accept_transit_gateway_peering_attachment(inp: &[u8], mut builder: crate::output::accept_transit_gateway_peering_attachment_output::Builder) -> Result<crate::output::accept_transit_gateway_peering_attachment_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("AcceptTransitGatewayPeeringAttachmentResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected AcceptTransitGatewayPeeringAttachmentResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("transitGatewayPeeringAttachment") /* TransitGatewayPeeringAttachment com.amazonaws.ec2.synthetic#AcceptTransitGatewayPeeringAttachmentOutput$TransitGatewayPeeringAttachment */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_transit_gateway_peering_attachment::de_transit_gateway_peering_attachment(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_peering_attachment(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

