// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_burnin_destination_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BurninDestinationSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.alignment {
        object.key("alignment").string(var_1.as_str());
    }
    if let Some(var_2) = &input.apply_font_color {
        object.key("applyFontColor").string(var_2.as_str());
    }
    if let Some(var_3) = &input.background_color {
        object.key("backgroundColor").string(var_3.as_str());
    }
    if input.background_opacity != 0 {
        object.key("backgroundOpacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.background_opacity).into()));
    }
    if let Some(var_4) = &input.fallback_font {
        object.key("fallbackFont").string(var_4.as_str());
    }
    if let Some(var_5) = &input.font_color {
        object.key("fontColor").string(var_5.as_str());
    }
    if input.font_opacity != 0 {
        object.key("fontOpacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.font_opacity).into()));
    }
    if input.font_resolution != 0 {
        object.key("fontResolution").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.font_resolution).into()));
    }
    if let Some(var_6) = &input.font_script {
        object.key("fontScript").string(var_6.as_str());
    }
    if input.font_size != 0 {
        object.key("fontSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.font_size).into()));
    }
    if let Some(var_7) = &input.hex_font_color {
        object.key("hexFontColor").string(var_7.as_str());
    }
    if let Some(var_8) = &input.outline_color {
        object.key("outlineColor").string(var_8.as_str());
    }
    if input.outline_size != 0 {
        object.key("outlineSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.outline_size).into()));
    }
    if let Some(var_9) = &input.shadow_color {
        object.key("shadowColor").string(var_9.as_str());
    }
    if input.shadow_opacity != 0 {
        object.key("shadowOpacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.shadow_opacity).into()));
    }
    if input.shadow_x_offset != 0 {
        object.key("shadowXOffset").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.shadow_x_offset).into()));
    }
    if input.shadow_y_offset != 0 {
        object.key("shadowYOffset").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.shadow_y_offset).into()));
    }
    if let Some(var_10) = &input.style_passthrough {
        object.key("stylePassthrough").string(var_10.as_str());
    }
    if let Some(var_11) = &input.teletext_spacing {
        object.key("teletextSpacing").string(var_11.as_str());
    }
    if input.x_position != 0 {
        object.key("xPosition").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.x_position).into()));
    }
    if input.y_position != 0 {
        object.key("yPosition").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.y_position).into()));
    }
    Ok(())
}

pub(crate) fn de_burnin_destination_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::BurninDestinationSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::burnin_destination_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "alignment" => {
                                builder = builder.set_alignment(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::BurninSubtitleAlignment::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "applyFontColor" => {
                                builder = builder.set_apply_font_color(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::BurninSubtitleApplyFontColor::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "backgroundColor" => {
                                builder = builder.set_background_color(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::BurninSubtitleBackgroundColor::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "backgroundOpacity" => {
                                builder = builder.set_background_opacity(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "fallbackFont" => {
                                builder = builder.set_fallback_font(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::BurninSubtitleFallbackFont::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "fontColor" => {
                                builder = builder.set_font_color(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::BurninSubtitleFontColor::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "fontOpacity" => {
                                builder = builder.set_font_opacity(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "fontResolution" => {
                                builder = builder.set_font_resolution(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "fontScript" => {
                                builder = builder.set_font_script(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::FontScript::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "fontSize" => {
                                builder = builder.set_font_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "hexFontColor" => {
                                builder = builder.set_hex_font_color(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "outlineColor" => {
                                builder = builder.set_outline_color(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::BurninSubtitleOutlineColor::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "outlineSize" => {
                                builder = builder.set_outline_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "shadowColor" => {
                                builder = builder.set_shadow_color(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::BurninSubtitleShadowColor::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "shadowOpacity" => {
                                builder = builder.set_shadow_opacity(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "shadowXOffset" => {
                                builder = builder.set_shadow_x_offset(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "shadowYOffset" => {
                                builder = builder.set_shadow_y_offset(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "stylePassthrough" => {
                                builder = builder.set_style_passthrough(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::BurnInSubtitleStylePassthrough::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "teletextSpacing" => {
                                builder = builder.set_teletext_spacing(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::BurninSubtitleTeletextSpacing::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "xPosition" => {
                                builder = builder.set_x_position(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "yPosition" => {
                                builder = builder.set_y_position(
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

