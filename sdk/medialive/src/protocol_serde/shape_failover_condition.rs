// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_failover_condition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FailoverCondition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.failover_condition_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("failoverConditionSettings").start_object();
        crate::protocol_serde::shape_failover_condition_settings::ser_failover_condition_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_failover_condition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::FailoverCondition>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::failover_condition::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "failoverConditionSettings" => {
                                builder = builder.set_failover_condition_settings(
                                    crate::protocol_serde::shape_failover_condition_settings::de_failover_condition_settings(tokens)?
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

