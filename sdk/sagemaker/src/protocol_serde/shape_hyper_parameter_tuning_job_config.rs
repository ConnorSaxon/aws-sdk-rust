// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_hyper_parameter_tuning_job_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HyperParameterTuningJobConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.strategy {
        object.key("Strategy").string(var_1.as_str());
    }
    if let Some(var_2) = &input.strategy_config {
        #[allow(unused_mut)]
        let mut object_3 = object.key("StrategyConfig").start_object();
        crate::protocol_serde::shape_hyper_parameter_tuning_job_strategy_config::ser_hyper_parameter_tuning_job_strategy_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.hyper_parameter_tuning_job_objective {
        #[allow(unused_mut)]
        let mut object_5 = object.key("HyperParameterTuningJobObjective").start_object();
        crate::protocol_serde::shape_hyper_parameter_tuning_job_objective::ser_hyper_parameter_tuning_job_objective(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.resource_limits {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ResourceLimits").start_object();
        crate::protocol_serde::shape_resource_limits::ser_resource_limits(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.parameter_ranges {
        #[allow(unused_mut)]
        let mut object_9 = object.key("ParameterRanges").start_object();
        crate::protocol_serde::shape_parameter_ranges::ser_parameter_ranges(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.training_job_early_stopping_type {
        object.key("TrainingJobEarlyStoppingType").string(var_10.as_str());
    }
    if let Some(var_11) = &input.tuning_job_completion_criteria {
        #[allow(unused_mut)]
        let mut object_12 = object.key("TuningJobCompletionCriteria").start_object();
        crate::protocol_serde::shape_tuning_job_completion_criteria::ser_tuning_job_completion_criteria(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.random_seed {
        object.key("RandomSeed").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_13).into()));
    }
    Ok(())
}

pub(crate) fn de_hyper_parameter_tuning_job_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::HyperParameterTuningJobConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::hyper_parameter_tuning_job_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Strategy" => {
                                builder = builder.set_strategy(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HyperParameterTuningJobStrategyType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "StrategyConfig" => {
                                builder = builder.set_strategy_config(
                                    crate::protocol_serde::shape_hyper_parameter_tuning_job_strategy_config::de_hyper_parameter_tuning_job_strategy_config(tokens)?
                                );
                            }
                            "HyperParameterTuningJobObjective" => {
                                builder = builder.set_hyper_parameter_tuning_job_objective(
                                    crate::protocol_serde::shape_hyper_parameter_tuning_job_objective::de_hyper_parameter_tuning_job_objective(tokens)?
                                );
                            }
                            "ResourceLimits" => {
                                builder = builder.set_resource_limits(
                                    crate::protocol_serde::shape_resource_limits::de_resource_limits(tokens)?
                                );
                            }
                            "ParameterRanges" => {
                                builder = builder.set_parameter_ranges(
                                    crate::protocol_serde::shape_parameter_ranges::de_parameter_ranges(tokens)?
                                );
                            }
                            "TrainingJobEarlyStoppingType" => {
                                builder = builder.set_training_job_early_stopping_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TrainingJobEarlyStoppingType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "TuningJobCompletionCriteria" => {
                                builder = builder.set_tuning_job_completion_criteria(
                                    crate::protocol_serde::shape_tuning_job_completion_criteria::de_tuning_job_completion_criteria(tokens)?
                                );
                            }
                            "RandomSeed" => {
                                builder = builder.set_random_seed(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

