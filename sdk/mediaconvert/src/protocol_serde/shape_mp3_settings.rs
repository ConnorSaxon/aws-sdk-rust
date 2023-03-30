// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_mp3_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Mp3Settings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.bitrate != 0 {
        object.key("bitrate").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.bitrate).into()));
    }
    if input.channels != 0 {
        object.key("channels").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.channels).into()));
    }
    if let Some(var_1) = &input.rate_control_mode {
        object.key("rateControlMode").string(var_1.as_str());
    }
    if input.sample_rate != 0 {
        object.key("sampleRate").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.sample_rate).into()));
    }
    if input.vbr_quality != 0 {
        object.key("vbrQuality").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.vbr_quality).into()));
    }
    Ok(())
}

pub(crate) fn de_mp3_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Mp3Settings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::mp3_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "bitrate" => {
                                builder = builder.set_bitrate(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "channels" => {
                                builder = builder.set_channels(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "rateControlMode" => {
                                builder = builder.set_rate_control_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::Mp3RateControlMode::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "sampleRate" => {
                                builder = builder.set_sample_rate(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "vbrQuality" => {
                                builder = builder.set_vbr_quality(
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

