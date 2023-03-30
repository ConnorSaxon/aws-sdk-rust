// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_deployment_targets_input(input: &crate::input::ListDeploymentTargetsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_deployment_targets_input::ser_list_deployment_targets_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_deployment_targets_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListDeploymentTargetsOutput, crate::error::ListDeploymentTargetsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListDeploymentTargetsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DeploymentDoesNotExistException" => crate::error::ListDeploymentTargetsError::DeploymentDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_does_not_exist_exception::de_deployment_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DeploymentIdRequiredException" => crate::error::ListDeploymentTargetsError::DeploymentIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_id_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_id_required_exception::de_deployment_id_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DeploymentNotStartedException" => crate::error::ListDeploymentTargetsError::DeploymentNotStartedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_not_started_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_not_started_exception::de_deployment_not_started_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDeploymentIdException" => crate::error::ListDeploymentTargetsError::InvalidDeploymentIdException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_deployment_id_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_deployment_id_exception::de_invalid_deployment_id_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDeploymentInstanceTypeException" => crate::error::ListDeploymentTargetsError::InvalidDeploymentInstanceTypeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_deployment_instance_type_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_deployment_instance_type_exception::de_invalid_deployment_instance_type_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInstanceStatusException" => crate::error::ListDeploymentTargetsError::InvalidInstanceStatusException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_instance_status_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_instance_status_exception::de_invalid_instance_status_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInstanceTypeException" => crate::error::ListDeploymentTargetsError::InvalidInstanceTypeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_instance_type_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_instance_type_exception::de_invalid_instance_type_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidNextTokenException" => crate::error::ListDeploymentTargetsError::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListDeploymentTargetsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_deployment_targets_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListDeploymentTargetsOutput, crate::error::ListDeploymentTargetsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_deployment_targets_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_deployment_targets::de_list_deployment_targets(response.body().as_ref(), output).map_err(crate::error::ListDeploymentTargetsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_deployment_targets(value: &[u8], mut builder: crate::output::list_deployment_targets_output::Builder) -> Result<crate::output::list_deployment_targets_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "targetIds" => {
                        builder = builder.set_target_ids(
                            crate::protocol_serde::shape_target_id_list::de_target_id_list(tokens)?
                        );
                    }
                    "nextToken" => {
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

