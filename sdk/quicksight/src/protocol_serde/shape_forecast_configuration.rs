// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_forecast_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ForecastConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.forecast_properties {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ForecastProperties").start_object();
        crate::protocol_serde::shape_time_based_forecast_properties::ser_time_based_forecast_properties(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.scenario {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Scenario").start_object();
        crate::protocol_serde::shape_forecast_scenario::ser_forecast_scenario(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_forecast_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ForecastConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::forecast_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ForecastProperties" => {
                                builder = builder.set_forecast_properties(
                                    crate::protocol_serde::shape_time_based_forecast_properties::de_time_based_forecast_properties(tokens)?
                                );
                            }
                            "Scenario" => {
                                builder = builder.set_scenario(
                                    crate::protocol_serde::shape_forecast_scenario::de_forecast_scenario(tokens)?
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

