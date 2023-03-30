// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attendee_capabilities(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AttendeeCapabilities) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.audio {
        object.key("Audio").string(var_1.as_str());
    }
    if let Some(var_2) = &input.video {
        object.key("Video").string(var_2.as_str());
    }
    if let Some(var_3) = &input.content {
        object.key("Content").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_attendee_capabilities<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AttendeeCapabilities>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::attendee_capabilities::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Audio" => {
                                builder = builder.set_audio(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::MediaCapabilities::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Video" => {
                                builder = builder.set_video(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::MediaCapabilities::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Content" => {
                                builder = builder.set_content(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::MediaCapabilities::from(u.as_ref())
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

