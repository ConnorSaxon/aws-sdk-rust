// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_detect_labels_image_quality<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DetectLabelsImageQuality>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::detect_labels_image_quality::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Brightness" => {
                                builder = builder.set_brightness(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f32_lossy())
                                );
                            }
                            "Sharpness" => {
                                builder = builder.set_sharpness(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f32_lossy())
                                );
                            }
                            "Contrast" => {
                                builder = builder.set_contrast(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f32_lossy())
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

