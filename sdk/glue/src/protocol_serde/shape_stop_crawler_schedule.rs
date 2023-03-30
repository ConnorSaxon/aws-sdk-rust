// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_crawler_schedule_input(input: &crate::input::StopCrawlerScheduleInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_stop_crawler_schedule_input::ser_stop_crawler_schedule_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_crawler_schedule_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StopCrawlerScheduleOutput, crate::error::StopCrawlerScheduleError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::StopCrawlerScheduleError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StopCrawlerScheduleError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EntityNotFoundException" => crate::error::StopCrawlerScheduleError::EntityNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_found_exception::de_entity_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StopCrawlerScheduleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationTimeoutException" => crate::error::StopCrawlerScheduleError::OperationTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_timeout_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_timeout_exception::de_operation_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StopCrawlerScheduleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SchedulerNotRunningException" => crate::error::StopCrawlerScheduleError::SchedulerNotRunningException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::scheduler_not_running_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_scheduler_not_running_exception::de_scheduler_not_running_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StopCrawlerScheduleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SchedulerTransitioningException" => crate::error::StopCrawlerScheduleError::SchedulerTransitioningException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::scheduler_transitioning_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_scheduler_transitioning_exception::de_scheduler_transitioning_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StopCrawlerScheduleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::StopCrawlerScheduleError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_crawler_schedule_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StopCrawlerScheduleOutput, crate::error::StopCrawlerScheduleError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::stop_crawler_schedule_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

