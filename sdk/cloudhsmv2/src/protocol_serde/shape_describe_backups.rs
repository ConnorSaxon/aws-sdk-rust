// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_backups_input(input: &crate::input::DescribeBackupsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_backups_input::ser_describe_backups_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_backups_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeBackupsOutput, crate::error::DescribeBackupsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeBackupsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeBackupsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CloudHsmAccessDeniedException" => crate::error::DescribeBackupsError::CloudHsmAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_access_denied_exception::de_cloud_hsm_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmInternalFailureException" => crate::error::DescribeBackupsError::CloudHsmInternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_internal_failure_exception::de_cloud_hsm_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmInvalidRequestException" => crate::error::DescribeBackupsError::CloudHsmInvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_invalid_request_exception::de_cloud_hsm_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmResourceNotFoundException" => crate::error::DescribeBackupsError::CloudHsmResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_resource_not_found_exception::de_cloud_hsm_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmServiceException" => crate::error::DescribeBackupsError::CloudHsmServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_service_exception::de_cloud_hsm_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmTagException" => crate::error::DescribeBackupsError::CloudHsmTagException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_tag_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_tag_exception::de_cloud_hsm_tag_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeBackupsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_backups_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeBackupsOutput, crate::error::DescribeBackupsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_backups_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_backups::de_describe_backups(response.body().as_ref(), output).map_err(crate::error::DescribeBackupsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_backups(value: &[u8], mut builder: crate::output::describe_backups_output::Builder) -> Result<crate::output::describe_backups_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Backups" => {
                        builder = builder.set_backups(
                            crate::protocol_serde::shape_backups::de_backups(tokens)?
                        );
                    }
                    "NextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
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

