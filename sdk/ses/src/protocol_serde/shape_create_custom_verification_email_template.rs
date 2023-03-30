// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_custom_verification_email_template_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateCustomVerificationEmailTemplateOutput, crate::error::CreateCustomVerificationEmailTemplateError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateCustomVerificationEmailTemplateError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateCustomVerificationEmailTemplateError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CustomVerificationEmailInvalidContent" => crate::error::CreateCustomVerificationEmailTemplateError::CustomVerificationEmailInvalidContentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::custom_verification_email_invalid_content_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_custom_verification_email_invalid_content_exception::de_custom_verification_email_invalid_content_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomVerificationEmailTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CustomVerificationEmailTemplateAlreadyExists" => crate::error::CreateCustomVerificationEmailTemplateError::CustomVerificationEmailTemplateAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::custom_verification_email_template_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_custom_verification_email_template_already_exists_exception::de_custom_verification_email_template_already_exists_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomVerificationEmailTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "FromEmailAddressNotVerified" => crate::error::CreateCustomVerificationEmailTemplateError::FromEmailAddressNotVerifiedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::from_email_address_not_verified_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_from_email_address_not_verified_exception::de_from_email_address_not_verified_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomVerificationEmailTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceeded" => crate::error::CreateCustomVerificationEmailTemplateError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomVerificationEmailTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateCustomVerificationEmailTemplateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_custom_verification_email_template_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateCustomVerificationEmailTemplateOutput, crate::error::CreateCustomVerificationEmailTemplateError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_custom_verification_email_template_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

