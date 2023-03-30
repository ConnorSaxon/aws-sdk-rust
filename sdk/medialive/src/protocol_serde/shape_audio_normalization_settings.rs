// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_audio_normalization_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AudioNormalizationSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.algorithm {
        object.key("algorithm").string(var_1.as_str());
    }
    if let Some(var_2) = &input.algorithm_control {
        object.key("algorithmControl").string(var_2.as_str());
    }
    if input.target_lkfs != 0.0 {
        object.key("targetLkfs").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.target_lkfs).into()));
    }
    Ok(())
}

pub(crate) fn de_audio_normalization_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AudioNormalizationSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::audio_normalization_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "algorithm" => {
                                builder = builder.set_algorithm(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AudioNormalizationAlgorithm::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "algorithmControl" => {
                                builder = builder.set_algorithm_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AudioNormalizationAlgorithmControl::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "targetLkfs" => {
                                builder = builder.set_target_lkfs(
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

