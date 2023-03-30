// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_archive_cdn_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ArchiveCdnSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.archive_s3_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("archiveS3Settings").start_object();
        crate::protocol_serde::shape_archive_s3_settings::ser_archive_s3_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_archive_cdn_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ArchiveCdnSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::archive_cdn_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "archiveS3Settings" => {
                                builder = builder.set_archive_s3_settings(
                                    crate::protocol_serde::shape_archive_s3_settings::de_archive_s3_settings(tokens)?
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

