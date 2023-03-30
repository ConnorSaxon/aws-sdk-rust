// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_training_job_input(input: &crate::input::DescribeTrainingJobInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_training_job_input::ser_describe_training_job_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_training_job_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeTrainingJobOutput, crate::error::DescribeTrainingJobError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeTrainingJobError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeTrainingJobError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ResourceNotFound" => crate::error::DescribeTrainingJobError::ResourceNotFound({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found::de_resource_not_found_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeTrainingJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeTrainingJobError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_training_job_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeTrainingJobOutput, crate::error::DescribeTrainingJobError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_training_job_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_training_job::de_describe_training_job(response.body().as_ref(), output).map_err(crate::error::DescribeTrainingJobError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_training_job(value: &[u8], mut builder: crate::output::describe_training_job_output::Builder) -> Result<crate::output::describe_training_job_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "TrainingJobName" => {
                        builder = builder.set_training_job_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "TrainingJobArn" => {
                        builder = builder.set_training_job_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "TuningJobArn" => {
                        builder = builder.set_tuning_job_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "LabelingJobArn" => {
                        builder = builder.set_labeling_job_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "AutoMLJobArn" => {
                        builder = builder.set_auto_ml_job_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ModelArtifacts" => {
                        builder = builder.set_model_artifacts(
                            crate::protocol_serde::shape_model_artifacts::de_model_artifacts(tokens)?
                        );
                    }
                    "TrainingJobStatus" => {
                        builder = builder.set_training_job_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::TrainingJobStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "SecondaryStatus" => {
                        builder = builder.set_secondary_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::SecondaryStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "FailureReason" => {
                        builder = builder.set_failure_reason(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "HyperParameters" => {
                        builder = builder.set_hyper_parameters(
                            crate::protocol_serde::shape_hyper_parameters::de_hyper_parameters(tokens)?
                        );
                    }
                    "AlgorithmSpecification" => {
                        builder = builder.set_algorithm_specification(
                            crate::protocol_serde::shape_algorithm_specification::de_algorithm_specification(tokens)?
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
                    "InputDataConfig" => {
                        builder = builder.set_input_data_config(
                            crate::protocol_serde::shape_input_data_config::de_input_data_config(tokens)?
                        );
                    }
                    "OutputDataConfig" => {
                        builder = builder.set_output_data_config(
                            crate::protocol_serde::shape_output_data_config::de_output_data_config(tokens)?
                        );
                    }
                    "ResourceConfig" => {
                        builder = builder.set_resource_config(
                            crate::protocol_serde::shape_resource_config::de_resource_config(tokens)?
                        );
                    }
                    "VpcConfig" => {
                        builder = builder.set_vpc_config(
                            crate::protocol_serde::shape_vpc_config::de_vpc_config(tokens)?
                        );
                    }
                    "StoppingCondition" => {
                        builder = builder.set_stopping_condition(
                            crate::protocol_serde::shape_stopping_condition::de_stopping_condition(tokens)?
                        );
                    }
                    "CreationTime" => {
                        builder = builder.set_creation_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "TrainingStartTime" => {
                        builder = builder.set_training_start_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "TrainingEndTime" => {
                        builder = builder.set_training_end_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "LastModifiedTime" => {
                        builder = builder.set_last_modified_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "SecondaryStatusTransitions" => {
                        builder = builder.set_secondary_status_transitions(
                            crate::protocol_serde::shape_secondary_status_transitions::de_secondary_status_transitions(tokens)?
                        );
                    }
                    "FinalMetricDataList" => {
                        builder = builder.set_final_metric_data_list(
                            crate::protocol_serde::shape_final_metric_data_list::de_final_metric_data_list(tokens)?
                        );
                    }
                    "EnableNetworkIsolation" => {
                        builder = builder.set_enable_network_isolation(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                        );
                    }
                    "EnableInterContainerTrafficEncryption" => {
                        builder = builder.set_enable_inter_container_traffic_encryption(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                        );
                    }
                    "EnableManagedSpotTraining" => {
                        builder = builder.set_enable_managed_spot_training(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                        );
                    }
                    "CheckpointConfig" => {
                        builder = builder.set_checkpoint_config(
                            crate::protocol_serde::shape_checkpoint_config::de_checkpoint_config(tokens)?
                        );
                    }
                    "TrainingTimeInSeconds" => {
                        builder = builder.set_training_time_in_seconds(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "BillableTimeInSeconds" => {
                        builder = builder.set_billable_time_in_seconds(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "DebugHookConfig" => {
                        builder = builder.set_debug_hook_config(
                            crate::protocol_serde::shape_debug_hook_config::de_debug_hook_config(tokens)?
                        );
                    }
                    "ExperimentConfig" => {
                        builder = builder.set_experiment_config(
                            crate::protocol_serde::shape_experiment_config::de_experiment_config(tokens)?
                        );
                    }
                    "DebugRuleConfigurations" => {
                        builder = builder.set_debug_rule_configurations(
                            crate::protocol_serde::shape_debug_rule_configurations::de_debug_rule_configurations(tokens)?
                        );
                    }
                    "TensorBoardOutputConfig" => {
                        builder = builder.set_tensor_board_output_config(
                            crate::protocol_serde::shape_tensor_board_output_config::de_tensor_board_output_config(tokens)?
                        );
                    }
                    "DebugRuleEvaluationStatuses" => {
                        builder = builder.set_debug_rule_evaluation_statuses(
                            crate::protocol_serde::shape_debug_rule_evaluation_statuses::de_debug_rule_evaluation_statuses(tokens)?
                        );
                    }
                    "ProfilerConfig" => {
                        builder = builder.set_profiler_config(
                            crate::protocol_serde::shape_profiler_config::de_profiler_config(tokens)?
                        );
                    }
                    "ProfilerRuleConfigurations" => {
                        builder = builder.set_profiler_rule_configurations(
                            crate::protocol_serde::shape_profiler_rule_configurations::de_profiler_rule_configurations(tokens)?
                        );
                    }
                    "ProfilerRuleEvaluationStatuses" => {
                        builder = builder.set_profiler_rule_evaluation_statuses(
                            crate::protocol_serde::shape_profiler_rule_evaluation_statuses::de_profiler_rule_evaluation_statuses(tokens)?
                        );
                    }
                    "ProfilingStatus" => {
                        builder = builder.set_profiling_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ProfilingStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "RetryStrategy" => {
                        builder = builder.set_retry_strategy(
                            crate::protocol_serde::shape_retry_strategy::de_retry_strategy(tokens)?
                        );
                    }
                    "Environment" => {
                        builder = builder.set_environment(
                            crate::protocol_serde::shape_training_environment_map::de_training_environment_map(tokens)?
                        );
                    }
                    "WarmPoolStatus" => {
                        builder = builder.set_warm_pool_status(
                            crate::protocol_serde::shape_warm_pool_status::de_warm_pool_status(tokens)?
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

