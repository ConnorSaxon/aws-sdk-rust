// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_solution_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SolutionConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_value_threshold {
        object.key("eventValueThreshold").string(var_1.as_str());
    }
    if let Some(var_2) = &input.hpo_config {
        #[allow(unused_mut)]
        let mut object_3 = object.key("hpoConfig").start_object();
        crate::protocol_serde::shape_hpo_config::ser_hpo_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.algorithm_hyper_parameters {
        #[allow(unused_mut)]
        let mut object_5 = object.key("algorithmHyperParameters").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.feature_transformation_parameters {
        #[allow(unused_mut)]
        let mut object_9 = object.key("featureTransformationParameters").start_object();
        for (key_10, value_11) in var_8 {
             {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    if let Some(var_12) = &input.auto_ml_config {
        #[allow(unused_mut)]
        let mut object_13 = object.key("autoMLConfig").start_object();
        crate::protocol_serde::shape_auto_ml_config::ser_auto_ml_config(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.optimization_objective {
        #[allow(unused_mut)]
        let mut object_15 = object.key("optimizationObjective").start_object();
        crate::protocol_serde::shape_optimization_objective::ser_optimization_objective(&mut object_15, var_14)?;
        object_15.finish();
    }
    Ok(())
}

pub(crate) fn de_solution_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SolutionConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::solution_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "eventValueThreshold" => {
                                builder = builder.set_event_value_threshold(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "hpoConfig" => {
                                builder = builder.set_hpo_config(
                                    crate::protocol_serde::shape_hpo_config::de_hpo_config(tokens)?
                                );
                            }
                            "algorithmHyperParameters" => {
                                builder = builder.set_algorithm_hyper_parameters(
                                    crate::protocol_serde::shape_hyper_parameters::de_hyper_parameters(tokens)?
                                );
                            }
                            "featureTransformationParameters" => {
                                builder = builder.set_feature_transformation_parameters(
                                    crate::protocol_serde::shape_feature_transformation_parameters::de_feature_transformation_parameters(tokens)?
                                );
                            }
                            "autoMLConfig" => {
                                builder = builder.set_auto_ml_config(
                                    crate::protocol_serde::shape_auto_ml_config::de_auto_ml_config(tokens)?
                                );
                            }
                            "optimizationObjective" => {
                                builder = builder.set_optimization_objective(
                                    crate::protocol_serde::shape_optimization_objective::de_optimization_objective(tokens)?
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

