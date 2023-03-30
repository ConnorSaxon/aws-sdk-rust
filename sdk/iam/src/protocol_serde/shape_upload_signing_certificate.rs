// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_upload_signing_certificate_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UploadSigningCertificateOutput, crate::error::UploadSigningCertificateError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UploadSigningCertificateError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UploadSigningCertificateError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DuplicateCertificate" => crate::error::UploadSigningCertificateError::DuplicateCertificateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::duplicate_certificate_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_certificate_exception::de_duplicate_certificate_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSigningCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EntityAlreadyExists" => crate::error::UploadSigningCertificateError::EntityAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_already_exists_exception::de_entity_already_exists_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSigningCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCertificate" => crate::error::UploadSigningCertificateError::InvalidCertificateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_certificate_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_certificate_exception::de_invalid_certificate_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSigningCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceeded" => crate::error::UploadSigningCertificateError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSigningCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MalformedCertificate" => crate::error::UploadSigningCertificateError::MalformedCertificateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::malformed_certificate_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_malformed_certificate_exception::de_malformed_certificate_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSigningCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::error::UploadSigningCertificateError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSigningCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::error::UploadSigningCertificateError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSigningCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UploadSigningCertificateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_upload_signing_certificate_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UploadSigningCertificateOutput, crate::error::UploadSigningCertificateError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::upload_signing_certificate_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_upload_signing_certificate::de_upload_signing_certificate(response.body().as_ref(), output).map_err(crate::error::UploadSigningCertificateError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_upload_signing_certificate(inp: &[u8], mut builder: crate::output::upload_signing_certificate_output::Builder) -> Result<crate::output::upload_signing_certificate_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("UploadSigningCertificateResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected UploadSigningCertificateResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("UploadSigningCertificateResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected UploadSigningCertificateResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Certificate") /* Certificate com.amazonaws.iam.synthetic#UploadSigningCertificateOutput$Certificate */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_signing_certificate::de_signing_certificate(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_certificate(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected UploadSigningCertificateResult tag"))
                    };
    Ok(builder)
}

