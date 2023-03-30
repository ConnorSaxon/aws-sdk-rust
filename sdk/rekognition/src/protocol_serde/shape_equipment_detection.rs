// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_equipment_detection<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::EquipmentDetection>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::equipment_detection::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "BoundingBox" => {
                                builder = builder.set_bounding_box(
                                    crate::protocol_serde::shape_bounding_box::de_bounding_box(tokens)?
                                );
                            }
                            "Confidence" => {
                                builder = builder.set_confidence(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f32_lossy())
                                );
                            }
                            "Type" => {
                                builder = builder.set_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ProtectiveEquipmentType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "CoversBodyPart" => {
                                builder = builder.set_covers_body_part(
                                    crate::protocol_serde::shape_covers_body_part::de_covers_body_part(tokens)?
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

