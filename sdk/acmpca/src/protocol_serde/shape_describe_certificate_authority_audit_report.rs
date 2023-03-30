// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_certificate_authority_audit_report_input(input: &crate::input::DescribeCertificateAuthorityAuditReportInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_certificate_authority_audit_report_input::ser_describe_certificate_authority_audit_report_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_certificate_authority_audit_report_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeCertificateAuthorityAuditReportOutput, crate::error::DescribeCertificateAuthorityAuditReportError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeCertificateAuthorityAuditReportError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeCertificateAuthorityAuditReportError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidArgsException" => crate::error::DescribeCertificateAuthorityAuditReportError::InvalidArgsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_args_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_args_exception::de_invalid_args_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeCertificateAuthorityAuditReportError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArnException" => crate::error::DescribeCertificateAuthorityAuditReportError::InvalidArnException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_arn_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_arn_exception::de_invalid_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeCertificateAuthorityAuditReportError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::DescribeCertificateAuthorityAuditReportError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeCertificateAuthorityAuditReportError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeCertificateAuthorityAuditReportError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_certificate_authority_audit_report_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeCertificateAuthorityAuditReportOutput, crate::error::DescribeCertificateAuthorityAuditReportError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_certificate_authority_audit_report_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_certificate_authority_audit_report::de_describe_certificate_authority_audit_report(response.body().as_ref(), output).map_err(crate::error::DescribeCertificateAuthorityAuditReportError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_certificate_authority_audit_report(value: &[u8], mut builder: crate::output::describe_certificate_authority_audit_report_output::Builder) -> Result<crate::output::describe_certificate_authority_audit_report_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "AuditReportStatus" => {
                        builder = builder.set_audit_report_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::AuditReportStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "S3BucketName" => {
                        builder = builder.set_s3_bucket_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "S3Key" => {
                        builder = builder.set_s3_key(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "CreatedAt" => {
                        builder = builder.set_created_at(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

