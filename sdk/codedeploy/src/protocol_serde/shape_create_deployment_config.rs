// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_deployment_config_input(input: &crate::input::CreateDeploymentConfigInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_deployment_config_input::ser_create_deployment_config_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_deployment_config_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateDeploymentConfigOutput, crate::error::CreateDeploymentConfigError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateDeploymentConfigError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateDeploymentConfigError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DeploymentConfigAlreadyExistsException" => crate::error::CreateDeploymentConfigError::DeploymentConfigAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_config_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_config_already_exists_exception::de_deployment_config_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DeploymentConfigLimitExceededException" => crate::error::CreateDeploymentConfigError::DeploymentConfigLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_config_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_config_limit_exceeded_exception::de_deployment_config_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DeploymentConfigNameRequiredException" => crate::error::CreateDeploymentConfigError::DeploymentConfigNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_config_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_config_name_required_exception::de_deployment_config_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidComputePlatformException" => crate::error::CreateDeploymentConfigError::InvalidComputePlatformException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_compute_platform_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_compute_platform_exception::de_invalid_compute_platform_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDeploymentConfigNameException" => crate::error::CreateDeploymentConfigError::InvalidDeploymentConfigNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_deployment_config_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_deployment_config_name_exception::de_invalid_deployment_config_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidMinimumHealthyHostValueException" => crate::error::CreateDeploymentConfigError::InvalidMinimumHealthyHostValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_minimum_healthy_host_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_minimum_healthy_host_value_exception::de_invalid_minimum_healthy_host_value_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTrafficRoutingConfigurationException" => crate::error::CreateDeploymentConfigError::InvalidTrafficRoutingConfigurationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_traffic_routing_configuration_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_traffic_routing_configuration_exception::de_invalid_traffic_routing_configuration_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateDeploymentConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_deployment_config_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateDeploymentConfigOutput, crate::error::CreateDeploymentConfigError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_deployment_config_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_deployment_config::de_create_deployment_config(response.body().as_ref(), output).map_err(crate::error::CreateDeploymentConfigError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_deployment_config(value: &[u8], mut builder: crate::output::create_deployment_config_output::Builder) -> Result<crate::output::create_deployment_config_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "deploymentConfigId" => {
                        builder = builder.set_deployment_config_id(
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

