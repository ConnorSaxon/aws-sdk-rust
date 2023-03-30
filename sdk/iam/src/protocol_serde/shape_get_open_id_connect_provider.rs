// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_open_id_connect_provider_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetOpenIdConnectProviderOutput, crate::error::GetOpenIDConnectProviderError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetOpenIDConnectProviderError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetOpenIDConnectProviderError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::error::GetOpenIDConnectProviderError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::GetOpenIDConnectProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::error::GetOpenIDConnectProviderError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::GetOpenIDConnectProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::error::GetOpenIDConnectProviderError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::GetOpenIDConnectProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetOpenIDConnectProviderError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_open_id_connect_provider_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetOpenIdConnectProviderOutput, crate::error::GetOpenIDConnectProviderError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_open_id_connect_provider_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_open_id_connect_provider::de_get_open_id_connect_provider(response.body().as_ref(), output).map_err(crate::error::GetOpenIDConnectProviderError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_open_id_connect_provider(inp: &[u8], mut builder: crate::output::get_open_id_connect_provider_output::Builder) -> Result<crate::output::get_open_id_connect_provider_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("GetOpenIDConnectProviderResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected GetOpenIDConnectProviderResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("GetOpenIDConnectProviderResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected GetOpenIDConnectProviderResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Url") /* Url com.amazonaws.iam.synthetic#GetOpenIDConnectProviderOutput$Url */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_url(var_1);
            }
            ,
            s if s.matches("ClientIDList") /* ClientIDList com.amazonaws.iam.synthetic#GetOpenIDConnectProviderOutput$ClientIDList */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_client_id_list_type::de_client_id_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_client_id_list(var_2);
            }
            ,
            s if s.matches("ThumbprintList") /* ThumbprintList com.amazonaws.iam.synthetic#GetOpenIDConnectProviderOutput$ThumbprintList */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_thumbprint_list_type::de_thumbprint_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_thumbprint_list(var_3);
            }
            ,
            s if s.matches("CreateDate") /* CreateDate com.amazonaws.iam.synthetic#GetOpenIDConnectProviderOutput$CreateDate */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.iam#dateType`)"))
                        ?
                    )
                ;
                builder = builder.set_create_date(var_4);
            }
            ,
            s if s.matches("Tags") /* Tags com.amazonaws.iam.synthetic#GetOpenIDConnectProviderOutput$Tags */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_tag_list_type::de_tag_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_5);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected GetOpenIDConnectProviderResult tag"))
                    };
    Ok(builder)
}

