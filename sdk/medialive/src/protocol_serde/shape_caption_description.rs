// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_caption_description(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CaptionDescription) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.accessibility {
        object.key("accessibility").string(var_1.as_str());
    }
    if let Some(var_2) = &input.caption_selector_name {
        object.key("captionSelectorName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.destination_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("destinationSettings").start_object();
        crate::protocol_serde::shape_caption_destination_settings::ser_caption_destination_settings(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.language_code {
        object.key("languageCode").string(var_5.as_str());
    }
    if let Some(var_6) = &input.language_description {
        object.key("languageDescription").string(var_6.as_str());
    }
    if let Some(var_7) = &input.name {
        object.key("name").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_caption_description<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CaptionDescription>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::caption_description::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "accessibility" => {
                                builder = builder.set_accessibility(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AccessibilityType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "captionSelectorName" => {
                                builder = builder.set_caption_selector_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "destinationSettings" => {
                                builder = builder.set_destination_settings(
                                    crate::protocol_serde::shape_caption_destination_settings::de_caption_destination_settings(tokens)?
                                );
                            }
                            "languageCode" => {
                                builder = builder.set_language_code(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "languageDescription" => {
                                builder = builder.set_language_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
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

