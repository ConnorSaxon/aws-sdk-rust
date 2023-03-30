// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_vpc_endpoint_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyVpcEndpointOutput, crate::error::ModifyVpcEndpointError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ModifyVpcEndpointError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::ModifyVpcEndpointError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_vpc_endpoint_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyVpcEndpointOutput, crate::error::ModifyVpcEndpointError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::modify_vpc_endpoint_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_vpc_endpoint::de_modify_vpc_endpoint(response.body().as_ref(), output).map_err(crate::error::ModifyVpcEndpointError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_vpc_endpoint(inp: &[u8], mut builder: crate::output::modify_vpc_endpoint_output::Builder) -> Result<crate::output::modify_vpc_endpoint_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ModifyVpcEndpointResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ModifyVpcEndpointResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("return") /* Return com.amazonaws.ec2.synthetic#ModifyVpcEndpointOutput$Return */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_return(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

