// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_provisioning_artifacts_for_service_action_input(input: &crate::input::ListProvisioningArtifactsForServiceActionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_provisioning_artifacts_for_service_action_input::ser_list_provisioning_artifacts_for_service_action_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_provisioning_artifacts_for_service_action_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListProvisioningArtifactsForServiceActionOutput, crate::error::ListProvisioningArtifactsForServiceActionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListProvisioningArtifactsForServiceActionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListProvisioningArtifactsForServiceActionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParametersException" => crate::error::ListProvisioningArtifactsForServiceActionError::InvalidParametersException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameters_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameters_exception::de_invalid_parameters_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListProvisioningArtifactsForServiceActionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::ListProvisioningArtifactsForServiceActionError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListProvisioningArtifactsForServiceActionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListProvisioningArtifactsForServiceActionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_provisioning_artifacts_for_service_action_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListProvisioningArtifactsForServiceActionOutput, crate::error::ListProvisioningArtifactsForServiceActionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_provisioning_artifacts_for_service_action_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_provisioning_artifacts_for_service_action::de_list_provisioning_artifacts_for_service_action(response.body().as_ref(), output).map_err(crate::error::ListProvisioningArtifactsForServiceActionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_provisioning_artifacts_for_service_action(value: &[u8], mut builder: crate::output::list_provisioning_artifacts_for_service_action_output::Builder) -> Result<crate::output::list_provisioning_artifacts_for_service_action_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ProvisioningArtifactViews" => {
                        builder = builder.set_provisioning_artifact_views(
                            crate::protocol_serde::shape_provisioning_artifact_views::de_provisioning_artifact_views(tokens)?
                        );
                    }
                    "NextPageToken" => {
                        builder = builder.set_next_page_token(
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

