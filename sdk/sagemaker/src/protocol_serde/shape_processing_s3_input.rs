// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_processing_s3_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ProcessingS3Input) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.s3_uri {
        object.key("S3Uri").string(var_1.as_str());
    }
    if let Some(var_2) = &input.local_path {
        object.key("LocalPath").string(var_2.as_str());
    }
    if let Some(var_3) = &input.s3_data_type {
        object.key("S3DataType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.s3_input_mode {
        object.key("S3InputMode").string(var_4.as_str());
    }
    if let Some(var_5) = &input.s3_data_distribution_type {
        object.key("S3DataDistributionType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.s3_compression_type {
        object.key("S3CompressionType").string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_processing_s3_input<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ProcessingS3Input>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::processing_s3_input::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "S3Uri" => {
                                builder = builder.set_s3_uri(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "LocalPath" => {
                                builder = builder.set_local_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "S3DataType" => {
                                builder = builder.set_s3_data_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ProcessingS3DataType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "S3InputMode" => {
                                builder = builder.set_s3_input_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ProcessingS3InputMode::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "S3DataDistributionType" => {
                                builder = builder.set_s3_data_distribution_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ProcessingS3DataDistributionType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "S3CompressionType" => {
                                builder = builder.set_s3_compression_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ProcessingS3CompressionType::from(u.as_ref())
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

