// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_preset_watermark(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PresetWatermark) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("Id").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_width {
        object.key("MaxWidth").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_height {
        object.key("MaxHeight").string(var_3.as_str());
    }
    if let Some(var_4) = &input.sizing_policy {
        object.key("SizingPolicy").string(var_4.as_str());
    }
    if let Some(var_5) = &input.horizontal_align {
        object.key("HorizontalAlign").string(var_5.as_str());
    }
    if let Some(var_6) = &input.horizontal_offset {
        object.key("HorizontalOffset").string(var_6.as_str());
    }
    if let Some(var_7) = &input.vertical_align {
        object.key("VerticalAlign").string(var_7.as_str());
    }
    if let Some(var_8) = &input.vertical_offset {
        object.key("VerticalOffset").string(var_8.as_str());
    }
    if let Some(var_9) = &input.opacity {
        object.key("Opacity").string(var_9.as_str());
    }
    if let Some(var_10) = &input.target {
        object.key("Target").string(var_10.as_str());
    }
    Ok(())
}

pub(crate) fn de_preset_watermark<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PresetWatermark>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::preset_watermark::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Id" => {
                                builder = builder.set_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "MaxWidth" => {
                                builder = builder.set_max_width(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "MaxHeight" => {
                                builder = builder.set_max_height(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SizingPolicy" => {
                                builder = builder.set_sizing_policy(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "HorizontalAlign" => {
                                builder = builder.set_horizontal_align(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "HorizontalOffset" => {
                                builder = builder.set_horizontal_offset(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "VerticalAlign" => {
                                builder = builder.set_vertical_align(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "VerticalOffset" => {
                                builder = builder.set_vertical_offset(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Opacity" => {
                                builder = builder.set_opacity(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Target" => {
                                builder = builder.set_target(
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

