// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_crawler_input(input: &crate::input::UpdateCrawlerInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_crawler_input::ser_update_crawler_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_crawler_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateCrawlerOutput, crate::error::UpdateCrawlerError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateCrawlerError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateCrawlerError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CrawlerRunningException" => crate::error::UpdateCrawlerError::CrawlerRunningException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::crawler_running_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_crawler_running_exception::de_crawler_running_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCrawlerError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EntityNotFoundException" => crate::error::UpdateCrawlerError::EntityNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_found_exception::de_entity_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCrawlerError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::error::UpdateCrawlerError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCrawlerError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationTimeoutException" => crate::error::UpdateCrawlerError::OperationTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_timeout_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_timeout_exception::de_operation_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCrawlerError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "VersionMismatchException" => crate::error::UpdateCrawlerError::VersionMismatchException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::version_mismatch_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_version_mismatch_exception::de_version_mismatch_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCrawlerError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateCrawlerError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_crawler_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateCrawlerOutput, crate::error::UpdateCrawlerError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_crawler_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

