// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_change_set_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateChangeSetOutput, crate::error::CreateChangeSetError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateChangeSetError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateChangeSetError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AlreadyExistsException" => crate::error::CreateChangeSetError::AlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_already_exists_exception::de_already_exists_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateChangeSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InsufficientCapabilitiesException" => crate::error::CreateChangeSetError::InsufficientCapabilitiesException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::insufficient_capabilities_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_capabilities_exception::de_insufficient_capabilities_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateChangeSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::CreateChangeSetError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateChangeSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateChangeSetError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_change_set_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateChangeSetOutput, crate::error::CreateChangeSetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_change_set_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_change_set::de_create_change_set(response.body().as_ref(), output).map_err(crate::error::CreateChangeSetError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_change_set(inp: &[u8], mut builder: crate::output::create_change_set_output::Builder) -> Result<crate::output::create_change_set_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("CreateChangeSetResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected CreateChangeSetResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("CreateChangeSetResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected CreateChangeSetResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.cloudformation.synthetic#CreateChangeSetOutput$Id */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_1);
            }
            ,
            s if s.matches("StackId") /* StackId com.amazonaws.cloudformation.synthetic#CreateChangeSetOutput$StackId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_stack_id(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected CreateChangeSetResult tag"))
                    };
    Ok(builder)
}

