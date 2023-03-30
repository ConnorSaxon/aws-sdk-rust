// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_android_push_notification_template(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AndroidPushNotificationTemplate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("Action").string(var_1.as_str());
    }
    if let Some(var_2) = &input.body {
        object.key("Body").string(var_2.as_str());
    }
    if let Some(var_3) = &input.image_icon_url {
        object.key("ImageIconUrl").string(var_3.as_str());
    }
    if let Some(var_4) = &input.image_url {
        object.key("ImageUrl").string(var_4.as_str());
    }
    if let Some(var_5) = &input.raw_content {
        object.key("RawContent").string(var_5.as_str());
    }
    if let Some(var_6) = &input.small_image_icon_url {
        object.key("SmallImageIconUrl").string(var_6.as_str());
    }
    if let Some(var_7) = &input.sound {
        object.key("Sound").string(var_7.as_str());
    }
    if let Some(var_8) = &input.title {
        object.key("Title").string(var_8.as_str());
    }
    if let Some(var_9) = &input.url {
        object.key("Url").string(var_9.as_str());
    }
    Ok(())
}

pub(crate) fn de_android_push_notification_template<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AndroidPushNotificationTemplate>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::android_push_notification_template::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Action" => {
                                builder = builder.set_action(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::Action::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Body" => {
                                builder = builder.set_body(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ImageIconUrl" => {
                                builder = builder.set_image_icon_url(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ImageUrl" => {
                                builder = builder.set_image_url(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RawContent" => {
                                builder = builder.set_raw_content(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SmallImageIconUrl" => {
                                builder = builder.set_small_image_icon_url(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Sound" => {
                                builder = builder.set_sound(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Title" => {
                                builder = builder.set_title(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Url" => {
                                builder = builder.set_url(
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

