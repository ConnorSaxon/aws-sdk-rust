// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_workspace_image_input(input: &crate::input::CreateWorkspaceImageInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_workspace_image_input::ser_create_workspace_image_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_workspace_image_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateWorkspaceImageOutput, crate::error::CreateWorkspaceImageError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateWorkspaceImageError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateWorkspaceImageError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::CreateWorkspaceImageError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValuesException" => crate::error::CreateWorkspaceImageError::InvalidParameterValuesException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_values_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_values_exception::de_invalid_parameter_values_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceStateException" => crate::error::CreateWorkspaceImageError::InvalidResourceStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_resource_state_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_state_exception::de_invalid_resource_state_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationNotSupportedException" => crate::error::CreateWorkspaceImageError::OperationNotSupportedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_supported_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_supported_exception::de_operation_not_supported_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceAlreadyExistsException" => crate::error::CreateWorkspaceImageError::ResourceAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_already_exists_exception::de_resource_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceLimitExceededException" => crate::error::CreateWorkspaceImageError::ResourceLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_limit_exceeded_exception::de_resource_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::CreateWorkspaceImageError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateWorkspaceImageError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_workspace_image_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateWorkspaceImageOutput, crate::error::CreateWorkspaceImageError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_workspace_image_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_workspace_image::de_create_workspace_image(response.body().as_ref(), output).map_err(crate::error::CreateWorkspaceImageError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_workspace_image(value: &[u8], mut builder: crate::output::create_workspace_image_output::Builder) -> Result<crate::output::create_workspace_image_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ImageId" => {
                        builder = builder.set_image_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Name" => {
                        builder = builder.set_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Description" => {
                        builder = builder.set_description(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "OperatingSystem" => {
                        builder = builder.set_operating_system(
                            crate::protocol_serde::shape_operating_system::de_operating_system(tokens)?
                        );
                    }
                    "State" => {
                        builder = builder.set_state(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::WorkspaceImageState::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "RequiredTenancy" => {
                        builder = builder.set_required_tenancy(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::WorkspaceImageRequiredTenancy::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "Created" => {
                        builder = builder.set_created(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "OwnerAccountId" => {
                        builder = builder.set_owner_account_id(
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

