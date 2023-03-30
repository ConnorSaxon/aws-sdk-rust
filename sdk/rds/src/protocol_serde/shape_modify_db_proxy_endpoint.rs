// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_db_proxy_endpoint_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyDbProxyEndpointOutput, crate::error::ModifyDBProxyEndpointError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ModifyDBProxyEndpointError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ModifyDBProxyEndpointError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DBProxyEndpointAlreadyExistsFault" => crate::error::ModifyDBProxyEndpointError::DbProxyEndpointAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_proxy_endpoint_already_exists_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_proxy_endpoint_already_exists_fault::de_db_proxy_endpoint_already_exists_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyDBProxyEndpointError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBProxyEndpointNotFoundFault" => crate::error::ModifyDBProxyEndpointError::DbProxyEndpointNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_proxy_endpoint_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_proxy_endpoint_not_found_fault::de_db_proxy_endpoint_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyDBProxyEndpointError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBProxyEndpointStateFault" => crate::error::ModifyDBProxyEndpointError::InvalidDbProxyEndpointStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_proxy_endpoint_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_proxy_endpoint_state_fault::de_invalid_db_proxy_endpoint_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyDBProxyEndpointError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBProxyStateFault" => crate::error::ModifyDBProxyEndpointError::InvalidDbProxyStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_proxy_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_proxy_state_fault::de_invalid_db_proxy_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyDBProxyEndpointError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ModifyDBProxyEndpointError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_db_proxy_endpoint_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyDbProxyEndpointOutput, crate::error::ModifyDBProxyEndpointError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::modify_db_proxy_endpoint_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_db_proxy_endpoint::de_modify_db_proxy_endpoint(response.body().as_ref(), output).map_err(crate::error::ModifyDBProxyEndpointError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_db_proxy_endpoint(inp: &[u8], mut builder: crate::output::modify_db_proxy_endpoint_output::Builder) -> Result<crate::output::modify_db_proxy_endpoint_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ModifyDBProxyEndpointResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ModifyDBProxyEndpointResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("ModifyDBProxyEndpointResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected ModifyDBProxyEndpointResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("DBProxyEndpoint") /* DBProxyEndpoint com.amazonaws.rds.synthetic#ModifyDBProxyEndpointOutput$DBProxyEndpoint */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_db_proxy_endpoint::de_db_proxy_endpoint(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_proxy_endpoint(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected ModifyDBProxyEndpointResult tag"))
                    };
    Ok(builder)
}

