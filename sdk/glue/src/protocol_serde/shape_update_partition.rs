// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_partition_input(input: &crate::input::UpdatePartitionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_partition_input::ser_update_partition_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_partition_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdatePartitionOutput, crate::error::UpdatePartitionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdatePartitionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdatePartitionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EntityNotFoundException" => crate::error::UpdatePartitionError::EntityNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_found_exception::de_entity_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePartitionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "GlueEncryptionException" => crate::error::UpdatePartitionError::GlueEncryptionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::glue_encryption_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_glue_encryption_exception::de_glue_encryption_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePartitionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceException" => crate::error::UpdatePartitionError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePartitionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::error::UpdatePartitionError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePartitionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationTimeoutException" => crate::error::UpdatePartitionError::OperationTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_timeout_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_timeout_exception::de_operation_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePartitionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdatePartitionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_partition_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdatePartitionOutput, crate::error::UpdatePartitionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_partition_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

