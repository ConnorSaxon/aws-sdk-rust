// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_deployment_config_input(input: &crate::input::GetDeploymentConfigInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_deployment_config_input::ser_get_deployment_config_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_deployment_config_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDeploymentConfigOutput, crate::error::GetDeploymentConfigError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetDeploymentConfigError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetDeploymentConfigError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DeploymentConfigDoesNotExistException" => crate::error::GetDeploymentConfigError::DeploymentConfigDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_config_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_config_does_not_exist_exception::de_deployment_config_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DeploymentConfigNameRequiredException" => crate::error::GetDeploymentConfigError::DeploymentConfigNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::deployment_config_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_deployment_config_name_required_exception::de_deployment_config_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidComputePlatformException" => crate::error::GetDeploymentConfigError::InvalidComputePlatformException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_compute_platform_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_compute_platform_exception::de_invalid_compute_platform_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDeploymentConfigNameException" => crate::error::GetDeploymentConfigError::InvalidDeploymentConfigNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_deployment_config_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_deployment_config_name_exception::de_invalid_deployment_config_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDeploymentConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetDeploymentConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_deployment_config_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDeploymentConfigOutput, crate::error::GetDeploymentConfigError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_deployment_config_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_deployment_config::de_get_deployment_config(response.body().as_ref(), output).map_err(crate::error::GetDeploymentConfigError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_deployment_config(value: &[u8], mut builder: crate::output::get_deployment_config_output::Builder) -> Result<crate::output::get_deployment_config_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "deploymentConfigInfo" => {
                        builder = builder.set_deployment_config_info(
                            crate::protocol_serde::shape_deployment_config_info::de_deployment_config_info(tokens)?
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

