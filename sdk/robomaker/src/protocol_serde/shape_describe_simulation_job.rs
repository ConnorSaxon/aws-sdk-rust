// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_simulation_job_input(input: &crate::input::DescribeSimulationJobInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_simulation_job_input::ser_describe_simulation_job_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_simulation_job_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeSimulationJobOutput, crate::error::DescribeSimulationJobError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeSimulationJobError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeSimulationJobError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::DescribeSimulationJobError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeSimulationJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::error::DescribeSimulationJobError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeSimulationJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::DescribeSimulationJobError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeSimulationJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::error::DescribeSimulationJobError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeSimulationJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeSimulationJobError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_simulation_job_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeSimulationJobOutput, crate::error::DescribeSimulationJobError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_simulation_job_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_simulation_job::de_describe_simulation_job(response.body().as_ref(), output).map_err(crate::error::DescribeSimulationJobError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_simulation_job(value: &[u8], mut builder: crate::output::describe_simulation_job_output::Builder) -> Result<crate::output::describe_simulation_job_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "arn" => {
                        builder = builder.set_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "clientRequestToken" => {
                        builder = builder.set_client_request_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "compute" => {
                        builder = builder.set_compute(
                            crate::protocol_serde::shape_compute_response::de_compute_response(tokens)?
                        );
                    }
                    "dataSources" => {
                        builder = builder.set_data_sources(
                            crate::protocol_serde::shape_data_sources::de_data_sources(tokens)?
                        );
                    }
                    "failureBehavior" => {
                        builder = builder.set_failure_behavior(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::FailureBehavior::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "failureCode" => {
                        builder = builder.set_failure_code(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::SimulationJobErrorCode::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "failureReason" => {
                        builder = builder.set_failure_reason(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "iamRole" => {
                        builder = builder.set_iam_role(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "lastStartedAt" => {
                        builder = builder.set_last_started_at(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "lastUpdatedAt" => {
                        builder = builder.set_last_updated_at(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "loggingConfig" => {
                        builder = builder.set_logging_config(
                            crate::protocol_serde::shape_logging_config::de_logging_config(tokens)?
                        );
                    }
                    "maxJobDurationInSeconds" => {
                        builder = builder.set_max_job_duration_in_seconds(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i64::try_from)
                                                .transpose()?
                        );
                    }
                    "name" => {
                        builder = builder.set_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "networkInterface" => {
                        builder = builder.set_network_interface(
                            crate::protocol_serde::shape_network_interface::de_network_interface(tokens)?
                        );
                    }
                    "outputLocation" => {
                        builder = builder.set_output_location(
                            crate::protocol_serde::shape_output_location::de_output_location(tokens)?
                        );
                    }
                    "robotApplications" => {
                        builder = builder.set_robot_applications(
                            crate::protocol_serde::shape_robot_application_configs::de_robot_application_configs(tokens)?
                        );
                    }
                    "simulationApplications" => {
                        builder = builder.set_simulation_applications(
                            crate::protocol_serde::shape_simulation_application_configs::de_simulation_application_configs(tokens)?
                        );
                    }
                    "simulationTimeMillis" => {
                        builder = builder.set_simulation_time_millis(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i64::try_from)
                                                .transpose()?
                        );
                    }
                    "status" => {
                        builder = builder.set_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::SimulationJobStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "tags" => {
                        builder = builder.set_tags(
                            crate::protocol_serde::shape_tag_map::de_tag_map(tokens)?
                        );
                    }
                    "vpcConfig" => {
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

