// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_video_selector_color_space_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VideoSelectorColorSpaceSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.hdr10_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("hdr10Settings").start_object();
        crate::protocol_serde::shape_hdr10_settings::ser_hdr10_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_video_selector_color_space_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::VideoSelectorColorSpaceSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::video_selector_color_space_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "hdr10Settings" => {
                                builder = builder.set_hdr10_settings(
                                    crate::protocol_serde::shape_hdr10_settings::de_hdr10_settings(tokens)?
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

