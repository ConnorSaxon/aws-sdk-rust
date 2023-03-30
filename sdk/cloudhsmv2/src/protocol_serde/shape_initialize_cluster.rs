// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_initialize_cluster_input(input: &crate::input::InitializeClusterInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_initialize_cluster_input::ser_initialize_cluster_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_initialize_cluster_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InitializeClusterOutput, crate::error::InitializeClusterError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::InitializeClusterError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::InitializeClusterError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CloudHsmAccessDeniedException" => crate::error::InitializeClusterError::CloudHsmAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_access_denied_exception::de_cloud_hsm_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InitializeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmInternalFailureException" => crate::error::InitializeClusterError::CloudHsmInternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_internal_failure_exception::de_cloud_hsm_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InitializeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmInvalidRequestException" => crate::error::InitializeClusterError::CloudHsmInvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_invalid_request_exception::de_cloud_hsm_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InitializeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmResourceNotFoundException" => crate::error::InitializeClusterError::CloudHsmResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_resource_not_found_exception::de_cloud_hsm_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InitializeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmServiceException" => crate::error::InitializeClusterError::CloudHsmServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_service_exception::de_cloud_hsm_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InitializeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::InitializeClusterError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_initialize_cluster_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InitializeClusterOutput, crate::error::InitializeClusterError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::initialize_cluster_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_initialize_cluster::de_initialize_cluster(response.body().as_ref(), output).map_err(crate::error::InitializeClusterError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_initialize_cluster(value: &[u8], mut builder: crate::output::initialize_cluster_output::Builder) -> Result<crate::output::initialize_cluster_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "State" => {
                        builder = builder.set_state(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ClusterState::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "StateMessage" => {
                        builder = builder.set_state_message(
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

