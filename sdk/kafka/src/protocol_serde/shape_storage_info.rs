// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_storage_info(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StorageInfo) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ebs_storage_info {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ebsStorageInfo").start_object();
        crate::protocol_serde::shape_ebs_storage_info::ser_ebs_storage_info(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_storage_info<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::StorageInfo>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::storage_info::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ebsStorageInfo" => {
                                builder = builder.set_ebs_storage_info(
                                    crate::protocol_serde::shape_ebs_storage_info::de_ebs_storage_info(tokens)?
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

