// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_action_threshold(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ActionThreshold) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("ActionThresholdValue").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.action_threshold_value).into()));
    }
    if let Some(var_1) = &input.action_threshold_type {
        object.key("ActionThresholdType").string(var_1.as_str());
    }
    Ok(())
}

pub(crate) fn de_action_threshold<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ActionThreshold>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::action_threshold::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ActionThresholdValue" => {
                                builder = builder.set_action_threshold_value(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                );
                            }
                            "ActionThresholdType" => {
                                builder = builder.set_action_threshold_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ThresholdType::from(u.as_ref())
                                        )
                                    ).transpose()?
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

