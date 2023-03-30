// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_position_coordinates(object_9: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PositionCoordinates) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::PositionCoordinates::CartesianCoordinates(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_1 = object_9.key("cartesianCoordinates").start_object();
                crate::protocol_serde::shape_cartesian_coordinates::ser_cartesian_coordinates(&mut object_1, inner)?;
                object_1.finish();
            }
        },
        crate::model::PositionCoordinates::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("PositionCoordinates"))
    }
    Ok(())
}

pub(crate) fn de_position_coordinates<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PositionCoordinates>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
                                Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        if variant.is_some() {
                                                            return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("encountered mixed variants in union"));
                                                        }
                        variant = match key.to_unescaped()?.as_ref() {
                            "cartesianCoordinates" => {
                                Some(crate::model::PositionCoordinates::CartesianCoordinates(
                                    crate::protocol_serde::shape_cartesian_coordinates::de_cartesian_coordinates(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'cartesianCoordinates' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                      aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::model::PositionCoordinates::Unknown)
                                                                    }
                        };
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
        }
        _ => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
    }
    Ok(variant)
}

