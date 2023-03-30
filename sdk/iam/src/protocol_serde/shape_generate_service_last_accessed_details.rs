// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_generate_service_last_accessed_details_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GenerateServiceLastAccessedDetailsOutput, crate::error::GenerateServiceLastAccessedDetailsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GenerateServiceLastAccessedDetailsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GenerateServiceLastAccessedDetailsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::error::GenerateServiceLastAccessedDetailsError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::GenerateServiceLastAccessedDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::error::GenerateServiceLastAccessedDetailsError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::GenerateServiceLastAccessedDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GenerateServiceLastAccessedDetailsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_generate_service_last_accessed_details_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GenerateServiceLastAccessedDetailsOutput, crate::error::GenerateServiceLastAccessedDetailsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::generate_service_last_accessed_details_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_generate_service_last_accessed_details::de_generate_service_last_accessed_details(response.body().as_ref(), output).map_err(crate::error::GenerateServiceLastAccessedDetailsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_generate_service_last_accessed_details(inp: &[u8], mut builder: crate::output::generate_service_last_accessed_details_output::Builder) -> Result<crate::output::generate_service_last_accessed_details_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("GenerateServiceLastAccessedDetailsResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected GenerateServiceLastAccessedDetailsResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("GenerateServiceLastAccessedDetailsResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected GenerateServiceLastAccessedDetailsResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("JobId") /* JobId com.amazonaws.iam.synthetic#GenerateServiceLastAccessedDetailsOutput$JobId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_job_id(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected GenerateServiceLastAccessedDetailsResult tag"))
                    };
    Ok(builder)
}

