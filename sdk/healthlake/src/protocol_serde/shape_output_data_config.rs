// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_output_data_config(object_3: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OutputDataConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::OutputDataConfig::S3Configuration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_1 = object_3.key("S3Configuration").start_object();
                crate::protocol_serde::shape_s3_configuration::ser_s3_configuration(&mut object_1, inner)?;
                object_1.finish();
            }
        },
        crate::model::OutputDataConfig::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("OutputDataConfig"))
    }
    Ok(())
}

pub(crate) fn de_output_data_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::OutputDataConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
                                Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        if variant.is_some() {
                                                            return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("encountered mixed variants in union"));
                                                        }
                        variant = match key.to_unescaped()?.as_ref() {
                            "S3Configuration" => {
                                Some(crate::model::OutputDataConfig::S3Configuration(
                                    crate::protocol_serde::shape_s3_configuration::de_s3_configuration(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'S3Configuration' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                      aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::model::OutputDataConfig::Unknown)
                                                                    }
                        };
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
        }
        _ => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
    }
    Ok(variant)
}

