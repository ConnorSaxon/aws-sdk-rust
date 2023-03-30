// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_metric_comparison_computation(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MetricComparisonComputation) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.computation_id {
        object.key("ComputationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.time {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Time").start_object();
        crate::protocol_serde::shape_dimension_field::ser_dimension_field(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.from_value {
        #[allow(unused_mut)]
        let mut object_6 = object.key("FromValue").start_object();
        crate::protocol_serde::shape_measure_field::ser_measure_field(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.target_value {
        #[allow(unused_mut)]
        let mut object_8 = object.key("TargetValue").start_object();
        crate::protocol_serde::shape_measure_field::ser_measure_field(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}

pub(crate) fn de_metric_comparison_computation<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::MetricComparisonComputation>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::metric_comparison_computation::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ComputationId" => {
                                builder = builder.set_computation_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Time" => {
                                builder = builder.set_time(
                                    crate::protocol_serde::shape_dimension_field::de_dimension_field(tokens)?
                                );
                            }
                            "FromValue" => {
                                builder = builder.set_from_value(
                                    crate::protocol_serde::shape_measure_field::de_measure_field(tokens)?
                                );
                            }
                            "TargetValue" => {
                                builder = builder.set_target_value(
                                    crate::protocol_serde::shape_measure_field::de_measure_field(tokens)?
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

