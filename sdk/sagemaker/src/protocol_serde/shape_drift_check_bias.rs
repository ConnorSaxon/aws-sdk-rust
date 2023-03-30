// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_drift_check_bias(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DriftCheckBias) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.config_file {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ConfigFile").start_object();
        crate::protocol_serde::shape_file_source::ser_file_source(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.pre_training_constraints {
        #[allow(unused_mut)]
        let mut object_4 = object.key("PreTrainingConstraints").start_object();
        crate::protocol_serde::shape_metrics_source::ser_metrics_source(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.post_training_constraints {
        #[allow(unused_mut)]
        let mut object_6 = object.key("PostTrainingConstraints").start_object();
        crate::protocol_serde::shape_metrics_source::ser_metrics_source(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_drift_check_bias<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DriftCheckBias>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::drift_check_bias::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ConfigFile" => {
                                builder = builder.set_config_file(
                                    crate::protocol_serde::shape_file_source::de_file_source(tokens)?
                                );
                            }
                            "PreTrainingConstraints" => {
                                builder = builder.set_pre_training_constraints(
                                    crate::protocol_serde::shape_metrics_source::de_metrics_source(tokens)?
                                );
                            }
                            "PostTrainingConstraints" => {
                                builder = builder.set_post_training_constraints(
                                    crate::protocol_serde::shape_metrics_source::de_metrics_source(tokens)?
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

