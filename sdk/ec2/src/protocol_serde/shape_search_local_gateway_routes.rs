// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_search_local_gateway_routes_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SearchLocalGatewayRoutesOutput, crate::error::SearchLocalGatewayRoutesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::SearchLocalGatewayRoutesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::SearchLocalGatewayRoutesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_search_local_gateway_routes_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SearchLocalGatewayRoutesOutput, crate::error::SearchLocalGatewayRoutesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::search_local_gateway_routes_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_search_local_gateway_routes::de_search_local_gateway_routes(response.body().as_ref(), output).map_err(crate::error::SearchLocalGatewayRoutesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_search_local_gateway_routes(inp: &[u8], mut builder: crate::output::search_local_gateway_routes_output::Builder) -> Result<crate::output::search_local_gateway_routes_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("SearchLocalGatewayRoutesResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected SearchLocalGatewayRoutesResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("routeSet") /* Routes com.amazonaws.ec2.synthetic#SearchLocalGatewayRoutesOutput$Routes */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_local_gateway_route_list::de_local_gateway_route_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_routes(var_1);
            }
            ,
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#SearchLocalGatewayRoutesOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

