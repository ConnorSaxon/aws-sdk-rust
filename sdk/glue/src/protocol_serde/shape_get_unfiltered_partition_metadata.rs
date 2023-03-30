// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_unfiltered_partition_metadata_input(input: &crate::input::GetUnfilteredPartitionMetadataInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_unfiltered_partition_metadata_input::ser_get_unfiltered_partition_metadata_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_unfiltered_partition_metadata_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetUnfilteredPartitionMetadataOutput, crate::error::GetUnfilteredPartitionMetadataError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetUnfilteredPartitionMetadataError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetUnfilteredPartitionMetadataError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EntityNotFoundException" => crate::error::GetUnfilteredPartitionMetadataError::EntityNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_found_exception::de_entity_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetUnfilteredPartitionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "GlueEncryptionException" => crate::error::GetUnfilteredPartitionMetadataError::GlueEncryptionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::glue_encryption_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_glue_encryption_exception::de_glue_encryption_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetUnfilteredPartitionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceException" => crate::error::GetUnfilteredPartitionMetadataError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetUnfilteredPartitionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::error::GetUnfilteredPartitionMetadataError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetUnfilteredPartitionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationTimeoutException" => crate::error::GetUnfilteredPartitionMetadataError::OperationTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_timeout_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_timeout_exception::de_operation_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetUnfilteredPartitionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PermissionTypeMismatchException" => crate::error::GetUnfilteredPartitionMetadataError::PermissionTypeMismatchException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::permission_type_mismatch_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_permission_type_mismatch_exception::de_permission_type_mismatch_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetUnfilteredPartitionMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetUnfilteredPartitionMetadataError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_unfiltered_partition_metadata_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetUnfilteredPartitionMetadataOutput, crate::error::GetUnfilteredPartitionMetadataError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_unfiltered_partition_metadata_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_unfiltered_partition_metadata::de_get_unfiltered_partition_metadata(response.body().as_ref(), output).map_err(crate::error::GetUnfilteredPartitionMetadataError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_unfiltered_partition_metadata(value: &[u8], mut builder: crate::output::get_unfiltered_partition_metadata_output::Builder) -> Result<crate::output::get_unfiltered_partition_metadata_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Partition" => {
                        builder = builder.set_partition(
                            crate::protocol_serde::shape_partition::de_partition(tokens)?
                        );
                    }
                    "AuthorizedColumns" => {
                        builder = builder.set_authorized_columns(
                            crate::protocol_serde::shape_name_string_list::de_name_string_list(tokens)?
                        );
                    }
                    "IsRegisteredWithLakeFormation" => {
                        builder = builder.set_is_registered_with_lake_formation(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

