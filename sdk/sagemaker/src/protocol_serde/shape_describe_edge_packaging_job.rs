// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_edge_packaging_job_input(input: &crate::input::DescribeEdgePackagingJobInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_edge_packaging_job_input::ser_describe_edge_packaging_job_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_edge_packaging_job_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeEdgePackagingJobOutput, crate::error::DescribeEdgePackagingJobError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeEdgePackagingJobError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeEdgePackagingJobError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ResourceNotFound" => crate::error::DescribeEdgePackagingJobError::ResourceNotFound({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found::de_resource_not_found_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeEdgePackagingJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeEdgePackagingJobError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_edge_packaging_job_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeEdgePackagingJobOutput, crate::error::DescribeEdgePackagingJobError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_edge_packaging_job_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_edge_packaging_job::de_describe_edge_packaging_job(response.body().as_ref(), output).map_err(crate::error::DescribeEdgePackagingJobError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_edge_packaging_job(value: &[u8], mut builder: crate::output::describe_edge_packaging_job_output::Builder) -> Result<crate::output::describe_edge_packaging_job_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "EdgePackagingJobArn" => {
                        builder = builder.set_edge_packaging_job_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "EdgePackagingJobName" => {
                        builder = builder.set_edge_packaging_job_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "CompilationJobName" => {
                        builder = builder.set_compilation_job_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ModelName" => {
                        builder = builder.set_model_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ModelVersion" => {
                        builder = builder.set_model_version(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "RoleArn" => {
                        builder = builder.set_role_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "OutputConfig" => {
                        builder = builder.set_output_config(
                            crate::protocol_serde::shape_edge_output_config::de_edge_output_config(tokens)?
                        );
                    }
                    "ResourceKey" => {
                        builder = builder.set_resource_key(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "EdgePackagingJobStatus" => {
                        builder = builder.set_edge_packaging_job_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::EdgePackagingJobStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "EdgePackagingJobStatusMessage" => {
                        builder = builder.set_edge_packaging_job_status_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "CreationTime" => {
                        builder = builder.set_creation_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "LastModifiedTime" => {
                        builder = builder.set_last_modified_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "ModelArtifact" => {
                        builder = builder.set_model_artifact(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ModelSignature" => {
                        builder = builder.set_model_signature(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "PresetDeploymentOutput" => {
                        builder = builder.set_preset_deployment_output(
                            crate::protocol_serde::shape_edge_preset_deployment_output::de_edge_preset_deployment_output(tokens)?
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

