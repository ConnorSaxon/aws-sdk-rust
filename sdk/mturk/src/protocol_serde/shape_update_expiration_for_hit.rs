// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_expiration_for_hit_input(input: &crate::input::UpdateExpirationForHitInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_expiration_for_hit_input::ser_update_expiration_for_hit_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_expiration_for_hit_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateExpirationForHitOutput, crate::error::UpdateExpirationForHITError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateExpirationForHITError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateExpirationForHITError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "RequestError" => crate::error::UpdateExpirationForHITError::RequestError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::request_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_request_error::de_request_error_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateExpirationForHITError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFault" => crate::error::UpdateExpirationForHITError::ServiceFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_fault::de_service_fault_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateExpirationForHITError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateExpirationForHITError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_expiration_for_hit_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateExpirationForHitOutput, crate::error::UpdateExpirationForHITError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_expiration_for_hit_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

