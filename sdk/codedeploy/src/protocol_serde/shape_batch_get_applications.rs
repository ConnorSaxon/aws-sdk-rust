// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_applications_input(input: &crate::input::BatchGetApplicationsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_batch_get_applications_input::ser_batch_get_applications_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_applications_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchGetApplicationsOutput, crate::error::BatchGetApplicationsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::BatchGetApplicationsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::BatchGetApplicationsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ApplicationDoesNotExistException" => crate::error::BatchGetApplicationsError::ApplicationDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::application_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_application_does_not_exist_exception::de_application_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetApplicationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ApplicationNameRequiredException" => crate::error::BatchGetApplicationsError::ApplicationNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::application_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_application_name_required_exception::de_application_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetApplicationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "BatchLimitExceededException" => crate::error::BatchGetApplicationsError::BatchLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::batch_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_batch_limit_exceeded_exception::de_batch_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetApplicationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidApplicationNameException" => crate::error::BatchGetApplicationsError::InvalidApplicationNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_application_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_application_name_exception::de_invalid_application_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetApplicationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::BatchGetApplicationsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_applications_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchGetApplicationsOutput, crate::error::BatchGetApplicationsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::batch_get_applications_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_batch_get_applications::de_batch_get_applications(response.body().as_ref(), output).map_err(crate::error::BatchGetApplicationsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_batch_get_applications(value: &[u8], mut builder: crate::output::batch_get_applications_output::Builder) -> Result<crate::output::batch_get_applications_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "applicationsInfo" => {
                        builder = builder.set_applications_info(
                            crate::protocol_serde::shape_applications_info_list::de_applications_info_list(tokens)?
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

