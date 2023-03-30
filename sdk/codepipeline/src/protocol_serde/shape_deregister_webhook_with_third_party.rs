// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deregister_webhook_with_third_party_input(input: &crate::input::DeregisterWebhookWithThirdPartyInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_deregister_webhook_with_third_party_input::ser_deregister_webhook_with_third_party_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_webhook_with_third_party_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeregisterWebhookWithThirdPartyOutput, crate::error::DeregisterWebhookWithThirdPartyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeregisterWebhookWithThirdPartyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeregisterWebhookWithThirdPartyError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ValidationException" => crate::error::DeregisterWebhookWithThirdPartyError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeregisterWebhookWithThirdPartyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WebhookNotFoundException" => crate::error::DeregisterWebhookWithThirdPartyError::WebhookNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::webhook_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_webhook_not_found_exception::de_webhook_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeregisterWebhookWithThirdPartyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeregisterWebhookWithThirdPartyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_webhook_with_third_party_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeregisterWebhookWithThirdPartyOutput, crate::error::DeregisterWebhookWithThirdPartyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::deregister_webhook_with_third_party_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

