// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sasl(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Sasl) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.scram {
        #[allow(unused_mut)]
        let mut object_2 = object.key("scram").start_object();
        crate::protocol_serde::shape_scram::ser_scram(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.iam {
        #[allow(unused_mut)]
        let mut object_4 = object.key("iam").start_object();
        crate::protocol_serde::shape_iam::ser_iam(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_sasl<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Sasl>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::sasl::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "scram" => {
                                builder = builder.set_scram(
                                    crate::protocol_serde::shape_scram::de_scram(tokens)?
                                );
                            }
                            "iam" => {
                                builder = builder.set_iam(
                                    crate::protocol_serde::shape_iam::de_iam(tokens)?
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

