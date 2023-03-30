// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_function_input(input: &crate::input::CreateFunctionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_function_input::ser_create_function_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_function_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateFunctionOutput, crate::error::CreateFunctionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateFunctionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateFunctionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CodeSigningConfigNotFoundException" => crate::error::CreateFunctionError::CodeSigningConfigNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::code_signing_config_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_code_signing_config_not_found_exception::de_code_signing_config_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CodeStorageExceededException" => crate::error::CreateFunctionError::CodeStorageExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::code_storage_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_code_storage_exceeded_exception::de_code_storage_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CodeVerificationFailedException" => crate::error::CreateFunctionError::CodeVerificationFailedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::code_verification_failed_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_code_verification_failed_exception::de_code_verification_failed_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCodeSignatureException" => crate::error::CreateFunctionError::InvalidCodeSignatureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_code_signature_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_code_signature_exception::de_invalid_code_signature_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::error::CreateFunctionError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceConflictException" => crate::error::CreateFunctionError::ResourceConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_conflict_exception::de_resource_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::CreateFunctionError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::CreateFunctionError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::CreateFunctionError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::error::CreateFunctionError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateFunctionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_function_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateFunctionOutput, crate::error::CreateFunctionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_function_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_function::de_create_function(response.body().as_ref(), output).map_err(crate::error::CreateFunctionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_function(value: &[u8], mut builder: crate::output::create_function_output::Builder) -> Result<crate::output::create_function_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Architectures" => {
                        builder = builder.set_architectures(
                            crate::protocol_serde::shape_architectures_list::de_architectures_list(tokens)?
                        );
                    }
                    "CodeSha256" => {
                        builder = builder.set_code_sha256(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "CodeSize" => {
                        builder = builder.set_code_size(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i64::try_from)
                                                .transpose()?
                        );
                    }
                    "DeadLetterConfig" => {
                        builder = builder.set_dead_letter_config(
                            crate::protocol_serde::shape_dead_letter_config::de_dead_letter_config(tokens)?
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
                    "Environment" => {
                        builder = builder.set_environment(
                            crate::protocol_serde::shape_environment_response::de_environment_response(tokens)?
                        );
                    }
                    "EphemeralStorage" => {
                        builder = builder.set_ephemeral_storage(
                            crate::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(tokens)?
                        );
                    }
                    "FileSystemConfigs" => {
                        builder = builder.set_file_system_configs(
                            crate::protocol_serde::shape_file_system_config_list::de_file_system_config_list(tokens)?
                        );
                    }
                    "FunctionArn" => {
                        builder = builder.set_function_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "FunctionName" => {
                        builder = builder.set_function_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Handler" => {
                        builder = builder.set_handler(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ImageConfigResponse" => {
                        builder = builder.set_image_config_response(
                            crate::protocol_serde::shape_image_config_response::de_image_config_response(tokens)?
                        );
                    }
                    "KMSKeyArn" => {
                        builder = builder.set_kms_key_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "LastModified" => {
                        builder = builder.set_last_modified(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "LastUpdateStatus" => {
                        builder = builder.set_last_update_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::LastUpdateStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "LastUpdateStatusReason" => {
                        builder = builder.set_last_update_status_reason(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "LastUpdateStatusReasonCode" => {
                        builder = builder.set_last_update_status_reason_code(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::LastUpdateStatusReasonCode::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "Layers" => {
                        builder = builder.set_layers(
                            crate::protocol_serde::shape_layers_reference_list::de_layers_reference_list(tokens)?
                        );
                    }
                    "MasterArn" => {
                        builder = builder.set_master_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "MemorySize" => {
                        builder = builder.set_memory_size(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "PackageType" => {
                        builder = builder.set_package_type(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::PackageType::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "RevisionId" => {
                        builder = builder.set_revision_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Role" => {
                        builder = builder.set_role(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Runtime" => {
                        builder = builder.set_runtime(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::Runtime::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "RuntimeVersionConfig" => {
                        builder = builder.set_runtime_version_config(
                            crate::protocol_serde::shape_runtime_version_config::de_runtime_version_config(tokens)?
                        );
                    }
                    "SigningJobArn" => {
                        builder = builder.set_signing_job_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "SigningProfileVersionArn" => {
                        builder = builder.set_signing_profile_version_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "SnapStart" => {
                        builder = builder.set_snap_start(
                            crate::protocol_serde::shape_snap_start_response::de_snap_start_response(tokens)?
                        );
                    }
                    "State" => {
                        builder = builder.set_state(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::State::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "StateReason" => {
                        builder = builder.set_state_reason(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "StateReasonCode" => {
                        builder = builder.set_state_reason_code(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::StateReasonCode::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "Timeout" => {
                        builder = builder.set_timeout(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "TracingConfig" => {
                        builder = builder.set_tracing_config(
                            crate::protocol_serde::shape_tracing_config_response::de_tracing_config_response(tokens)?
                        );
                    }
                    "Version" => {
                        builder = builder.set_version(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "VpcConfig" => {
                        builder = builder.set_vpc_config(
                            crate::protocol_serde::shape_vpc_config_response::de_vpc_config_response(tokens)?
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

