// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_detect_stack_set_drift_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DetectStackSetDriftOutput, crate::error::DetectStackSetDriftError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DetectStackSetDriftError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DetectStackSetDriftError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidOperationException" => crate::error::DetectStackSetDriftError::InvalidOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_operation_exception::de_invalid_operation_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DetectStackSetDriftError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationInProgressException" => crate::error::DetectStackSetDriftError::OperationInProgressException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_in_progress_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_in_progress_exception::de_operation_in_progress_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DetectStackSetDriftError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "StackSetNotFoundException" => crate::error::DetectStackSetDriftError::StackSetNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::stack_set_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_stack_set_not_found_exception::de_stack_set_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DetectStackSetDriftError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DetectStackSetDriftError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_detect_stack_set_drift_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DetectStackSetDriftOutput, crate::error::DetectStackSetDriftError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::detect_stack_set_drift_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_detect_stack_set_drift::de_detect_stack_set_drift(response.body().as_ref(), output).map_err(crate::error::DetectStackSetDriftError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_detect_stack_set_drift(inp: &[u8], mut builder: crate::output::detect_stack_set_drift_output::Builder) -> Result<crate::output::detect_stack_set_drift_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DetectStackSetDriftResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DetectStackSetDriftResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DetectStackSetDriftResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DetectStackSetDriftResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("OperationId") /* OperationId com.amazonaws.cloudformation.synthetic#DetectStackSetDriftOutput$OperationId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_operation_id(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DetectStackSetDriftResult tag"))
                    };
    Ok(builder)
}

