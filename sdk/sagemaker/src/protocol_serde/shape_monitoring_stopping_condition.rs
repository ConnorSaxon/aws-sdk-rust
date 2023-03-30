// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_monitoring_stopping_condition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MonitoringStoppingCondition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("MaxRuntimeInSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_runtime_in_seconds).into()));
    }
    Ok(())
}

pub(crate) fn de_monitoring_stopping_condition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::MonitoringStoppingCondition>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::monitoring_stopping_condition::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "MaxRuntimeInSeconds" => {
                                builder = builder.set_max_runtime_in_seconds(
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

