// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_deployment_instances_input(input: &crate::input::BatchGetDeploymentInstancesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_batch_get_deployment_instances_input::ser_batch_get_deployment_instances_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_deployment_instances_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchGetDeploymentInstancesOutput, crate::error::BatchGetDeploymentInstancesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::BatchGetDeploymentInstancesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::BatchGetDeploymentInstancesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BatchLimitExceededException" => crate::error::BatchGetDeploymentInstancesError::BatchLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::batch_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_batch_limit_exceeded_exception::de_batch_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetDeploymentInstancesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DeploymentDoesNotExistException" => crate::error::BatchGetDeploymentInstancesError::DeploymentDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_does_not_exist_exception::de_deployment_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetDeploymentInstancesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DeploymentIdRequiredException" => crate::error::BatchGetDeploymentInstancesError::DeploymentIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_id_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_id_required_exception::de_deployment_id_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetDeploymentInstancesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InstanceIdRequiredException" => crate::error::BatchGetDeploymentInstancesError::InstanceIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::instance_id_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_instance_id_required_exception::de_instance_id_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetDeploymentInstancesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidComputePlatformException" => crate::error::BatchGetDeploymentInstancesError::InvalidComputePlatformException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_compute_platform_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_compute_platform_exception::de_invalid_compute_platform_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetDeploymentInstancesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDeploymentIdException" => crate::error::BatchGetDeploymentInstancesError::InvalidDeploymentIdException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_deployment_id_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_deployment_id_exception::de_invalid_deployment_id_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetDeploymentInstancesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInstanceNameException" => crate::error::BatchGetDeploymentInstancesError::InvalidInstanceNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_instance_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_instance_name_exception::de_invalid_instance_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetDeploymentInstancesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::BatchGetDeploymentInstancesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_deployment_instances_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchGetDeploymentInstancesOutput, crate::error::BatchGetDeploymentInstancesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::batch_get_deployment_instances_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_batch_get_deployment_instances::de_batch_get_deployment_instances(response.body().as_ref(), output).map_err(crate::error::BatchGetDeploymentInstancesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_batch_get_deployment_instances(value: &[u8], mut builder: crate::output::batch_get_deployment_instances_output::Builder) -> Result<crate::output::batch_get_deployment_instances_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "instancesSummary" => {
                        builder = builder.set_instances_summary(
                            crate::protocol_serde::shape_instance_summary_list::de_instance_summary_list(tokens)?
                        );
                    }
                    "errorMessage" => {
                        builder = builder.set_error_message(
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

