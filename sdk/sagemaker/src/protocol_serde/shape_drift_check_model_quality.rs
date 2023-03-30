// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_drift_check_model_quality(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DriftCheckModelQuality) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.statistics {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Statistics").start_object();
        crate::protocol_serde::shape_metrics_source::ser_metrics_source(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.constraints {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Constraints").start_object();
        crate::protocol_serde::shape_metrics_source::ser_metrics_source(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_drift_check_model_quality<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DriftCheckModelQuality>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::drift_check_model_quality::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Statistics" => {
                                builder = builder.set_statistics(
                                    crate::protocol_serde::shape_metrics_source::de_metrics_source(tokens)?
                                );
                            }
                            "Constraints" => {
                                builder = builder.set_constraints(
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

