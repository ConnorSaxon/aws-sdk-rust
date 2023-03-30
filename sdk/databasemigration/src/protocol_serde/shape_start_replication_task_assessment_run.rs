// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_replication_task_assessment_run_input(input: &crate::input::StartReplicationTaskAssessmentRunInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_replication_task_assessment_run_input::ser_start_replication_task_assessment_run_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_replication_task_assessment_run_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartReplicationTaskAssessmentRunOutput, crate::error::StartReplicationTaskAssessmentRunError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartReplicationTaskAssessmentRunError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedFault" => crate::error::StartReplicationTaskAssessmentRunError::AccessDeniedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_fault::de_access_denied_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceStateFault" => crate::error::StartReplicationTaskAssessmentRunError::InvalidResourceStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_resource_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_state_fault::de_invalid_resource_state_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSAccessDeniedFault" => crate::error::StartReplicationTaskAssessmentRunError::KmsAccessDeniedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::kms_access_denied_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_access_denied_fault::de_kms_access_denied_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSDisabledFault" => crate::error::StartReplicationTaskAssessmentRunError::KmsDisabledFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::kms_disabled_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_disabled_fault::de_kms_disabled_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSFault" => crate::error::StartReplicationTaskAssessmentRunError::KmsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::kms_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_fault::de_kms_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSInvalidStateFault" => crate::error::StartReplicationTaskAssessmentRunError::KmsInvalidStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::kms_invalid_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_invalid_state_fault::de_kms_invalid_state_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSKeyNotAccessibleFault" => crate::error::StartReplicationTaskAssessmentRunError::KmsKeyNotAccessibleFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::kms_key_not_accessible_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_key_not_accessible_fault::de_kms_key_not_accessible_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSNotFoundFault" => crate::error::StartReplicationTaskAssessmentRunError::KmsNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::kms_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_not_found_fault::de_kms_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceAlreadyExistsFault" => crate::error::StartReplicationTaskAssessmentRunError::ResourceAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_already_exists_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_already_exists_fault::de_resource_already_exists_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundFault" => crate::error::StartReplicationTaskAssessmentRunError::ResourceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_fault::de_resource_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "S3AccessDeniedFault" => crate::error::StartReplicationTaskAssessmentRunError::S3AccessDeniedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::s3_access_denied_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_s3_access_denied_fault::de_s3_access_denied_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "S3ResourceNotFoundFault" => crate::error::StartReplicationTaskAssessmentRunError::S3ResourceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::s3_resource_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_s3_resource_not_found_fault::de_s3_resource_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::StartReplicationTaskAssessmentRunError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_replication_task_assessment_run_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartReplicationTaskAssessmentRunOutput, crate::error::StartReplicationTaskAssessmentRunError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_replication_task_assessment_run_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_start_replication_task_assessment_run::de_start_replication_task_assessment_run(response.body().as_ref(), output).map_err(crate::error::StartReplicationTaskAssessmentRunError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_start_replication_task_assessment_run(value: &[u8], mut builder: crate::output::start_replication_task_assessment_run_output::Builder) -> Result<crate::output::start_replication_task_assessment_run_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ReplicationTaskAssessmentRun" => {
                        builder = builder.set_replication_task_assessment_run(
                            crate::protocol_serde::shape_replication_task_assessment_run::de_replication_task_assessment_run(tokens)?
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

