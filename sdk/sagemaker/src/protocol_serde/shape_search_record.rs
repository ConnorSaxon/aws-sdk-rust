// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_search_record<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SearchRecord>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::search_record::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "TrainingJob" => {
                                builder = builder.set_training_job(
                                    crate::protocol_serde::shape_training_job::de_training_job(tokens)?
                                );
                            }
                            "Experiment" => {
                                builder = builder.set_experiment(
                                    crate::protocol_serde::shape_experiment::de_experiment(tokens)?
                                );
                            }
                            "Trial" => {
                                builder = builder.set_trial(
                                    crate::protocol_serde::shape_trial::de_trial(tokens)?
                                );
                            }
                            "TrialComponent" => {
                                builder = builder.set_trial_component(
                                    crate::protocol_serde::shape_trial_component::de_trial_component(tokens)?
                                );
                            }
                            "Endpoint" => {
                                builder = builder.set_endpoint(
                                    crate::protocol_serde::shape_endpoint::de_endpoint(tokens)?
                                );
                            }
                            "ModelPackage" => {
                                builder = builder.set_model_package(
                                    crate::protocol_serde::shape_model_package::de_model_package(tokens)?
                                );
                            }
                            "ModelPackageGroup" => {
                                builder = builder.set_model_package_group(
                                    crate::protocol_serde::shape_model_package_group::de_model_package_group(tokens)?
                                );
                            }
                            "Pipeline" => {
                                builder = builder.set_pipeline(
                                    crate::protocol_serde::shape_pipeline::de_pipeline(tokens)?
                                );
                            }
                            "PipelineExecution" => {
                                builder = builder.set_pipeline_execution(
                                    crate::protocol_serde::shape_pipeline_execution::de_pipeline_execution(tokens)?
                                );
                            }
                            "FeatureGroup" => {
                                builder = builder.set_feature_group(
                                    crate::protocol_serde::shape_feature_group::de_feature_group(tokens)?
                                );
                            }
                            "Project" => {
                                builder = builder.set_project(
                                    crate::protocol_serde::shape_project::de_project(tokens)?
                                );
                            }
                            "FeatureMetadata" => {
                                builder = builder.set_feature_metadata(
                                    crate::protocol_serde::shape_feature_metadata::de_feature_metadata(tokens)?
                                );
                            }
                            "HyperParameterTuningJob" => {
                                builder = builder.set_hyper_parameter_tuning_job(
                                    crate::protocol_serde::shape_hyper_parameter_tuning_job_search_entity::de_hyper_parameter_tuning_job_search_entity(tokens)?
                                );
                            }
                            "Model" => {
                                builder = builder.set_model(
                                    crate::protocol_serde::shape_model_dashboard_model::de_model_dashboard_model(tokens)?
                                );
                            }
                            "ModelCard" => {
                                builder = builder.set_model_card(
                                    crate::protocol_serde::shape_model_card::de_model_card(tokens)?
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

