// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_h265_qvbr_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::H265QvbrSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.max_average_bitrate != 0 {
        object.key("maxAverageBitrate").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_average_bitrate).into()));
    }
    if input.qvbr_quality_level != 0 {
        object.key("qvbrQualityLevel").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.qvbr_quality_level).into()));
    }
    if input.qvbr_quality_level_fine_tune != 0.0 {
        object.key("qvbrQualityLevelFineTune").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.qvbr_quality_level_fine_tune).into()));
    }
    Ok(())
}

pub(crate) fn de_h265_qvbr_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::H265QvbrSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::h265_qvbr_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "maxAverageBitrate" => {
                                builder = builder.set_max_average_bitrate(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "qvbrQualityLevel" => {
                                builder = builder.set_qvbr_quality_level(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "qvbrQualityLevelFineTune" => {
                                builder = builder.set_qvbr_quality_level_fine_tune(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
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

