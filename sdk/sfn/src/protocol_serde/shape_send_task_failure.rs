// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_task_failure_input(input: &crate::input::SendTaskFailureInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_send_task_failure_input::ser_send_task_failure_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_send_task_failure_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SendTaskFailureOutput, crate::error::SendTaskFailureError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::SendTaskFailureError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::SendTaskFailureError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidToken" => crate::error::SendTaskFailureError::InvalidToken({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_token::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_token::de_invalid_token_json_err(response.body().as_ref(), output).map_err(crate::error::SendTaskFailureError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TaskDoesNotExist" => crate::error::SendTaskFailureError::TaskDoesNotExist({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::task_does_not_exist::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_task_does_not_exist::de_task_does_not_exist_json_err(response.body().as_ref(), output).map_err(crate::error::SendTaskFailureError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TaskTimedOut" => crate::error::SendTaskFailureError::TaskTimedOut({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::task_timed_out::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_task_timed_out::de_task_timed_out_json_err(response.body().as_ref(), output).map_err(crate::error::SendTaskFailureError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::SendTaskFailureError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_send_task_failure_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SendTaskFailureOutput, crate::error::SendTaskFailureError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::send_task_failure_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

