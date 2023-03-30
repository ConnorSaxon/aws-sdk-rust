// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deserializer(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Deserializer) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.open_x_json_ser_de {
        #[allow(unused_mut)]
        let mut object_2 = object.key("OpenXJsonSerDe").start_object();
        crate::protocol_serde::shape_open_x_json_ser_de::ser_open_x_json_ser_de(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.hive_json_ser_de {
        #[allow(unused_mut)]
        let mut object_4 = object.key("HiveJsonSerDe").start_object();
        crate::protocol_serde::shape_hive_json_ser_de::ser_hive_json_ser_de(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_deserializer<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Deserializer>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::deserializer::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "OpenXJsonSerDe" => {
                                builder = builder.set_open_x_json_ser_de(
                                    crate::protocol_serde::shape_open_x_json_ser_de::de_open_x_json_ser_de(tokens)?
                                );
                            }
                            "HiveJsonSerDe" => {
                                builder = builder.set_hive_json_ser_de(
                                    crate::protocol_serde::shape_hive_json_ser_de::de_hive_json_ser_de(tokens)?
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

