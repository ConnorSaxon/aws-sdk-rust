// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_certificate_authority_input(input: &crate::input::UpdateCertificateAuthorityInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_certificate_authority_input::ser_update_certificate_authority_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_certificate_authority_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateCertificateAuthorityOutput, crate::error::UpdateCertificateAuthorityError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateCertificateAuthorityError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateCertificateAuthorityError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ConcurrentModificationException" => crate::error::UpdateCertificateAuthorityError::ConcurrentModificationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::concurrent_modification_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCertificateAuthorityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgsException" => crate::error::UpdateCertificateAuthorityError::InvalidArgsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_args_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_args_exception::de_invalid_args_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCertificateAuthorityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArnException" => crate::error::UpdateCertificateAuthorityError::InvalidArnException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_arn_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_arn_exception::de_invalid_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCertificateAuthorityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPolicyException" => crate::error::UpdateCertificateAuthorityError::InvalidPolicyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_policy_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_policy_exception::de_invalid_policy_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCertificateAuthorityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidStateException" => crate::error::UpdateCertificateAuthorityError::InvalidStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_state_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_state_exception::de_invalid_state_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCertificateAuthorityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::UpdateCertificateAuthorityError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCertificateAuthorityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateCertificateAuthorityError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_certificate_authority_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateCertificateAuthorityOutput, crate::error::UpdateCertificateAuthorityError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_certificate_authority_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

