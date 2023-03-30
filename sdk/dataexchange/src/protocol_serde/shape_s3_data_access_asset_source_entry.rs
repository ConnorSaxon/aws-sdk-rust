// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_data_access_asset_source_entry(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3DataAccessAssetSourceEntry) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.bucket {
        object.key("Bucket").string(var_1.as_str());
    }
    if let Some(var_2) = &input.key_prefixes {
        let mut array_3 = object.key("KeyPrefixes").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.keys {
        let mut array_6 = object.key("Keys").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub(crate) fn de_s3_data_access_asset_source_entry<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::S3DataAccessAssetSourceEntry>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::s3_data_access_asset_source_entry::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Bucket" => {
                                builder = builder.set_bucket(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "KeyPrefixes" => {
                                builder = builder.set_key_prefixes(
                                    crate::protocol_serde::shape_list_of__string::de_list_of__string(tokens)?
                                );
                            }
                            "Keys" => {
                                builder = builder.set_keys(
                                    crate::protocol_serde::shape_list_of__string::de_list_of__string(tokens)?
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

