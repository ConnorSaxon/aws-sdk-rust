// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_archive_group_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ArchiveGroupSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.archive_cdn_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("archiveCdnSettings").start_object();
        crate::protocol_serde::shape_archive_cdn_settings::ser_archive_cdn_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.destination {
        #[allow(unused_mut)]
        let mut object_4 = object.key("destination").start_object();
        crate::protocol_serde::shape_output_location_ref::ser_output_location_ref(&mut object_4, var_3)?;
        object_4.finish();
    }
    if input.rollover_interval != 0 {
        object.key("rolloverInterval").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.rollover_interval).into()));
    }
    Ok(())
}

pub(crate) fn de_archive_group_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ArchiveGroupSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::archive_group_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "archiveCdnSettings" => {
                                builder = builder.set_archive_cdn_settings(
                                    crate::protocol_serde::shape_archive_cdn_settings::de_archive_cdn_settings(tokens)?
                                );
                            }
                            "destination" => {
                                builder = builder.set_destination(
                                    crate::protocol_serde::shape_output_location_ref::de_output_location_ref(tokens)?
                                );
                            }
                            "rolloverInterval" => {
                                builder = builder.set_rollover_interval(
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

