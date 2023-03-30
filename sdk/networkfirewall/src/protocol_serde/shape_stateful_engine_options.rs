// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stateful_engine_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StatefulEngineOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.rule_order {
        object.key("RuleOrder").string(var_1.as_str());
    }
    if let Some(var_2) = &input.stream_exception_policy {
        object.key("StreamExceptionPolicy").string(var_2.as_str());
    }
    Ok(())
}

pub(crate) fn de_stateful_engine_options<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::StatefulEngineOptions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::stateful_engine_options::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "RuleOrder" => {
                                builder = builder.set_rule_order(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::RuleOrder::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "StreamExceptionPolicy" => {
                                builder = builder.set_stream_exception_policy(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::StreamExceptionPolicy::from(u.as_ref())
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

