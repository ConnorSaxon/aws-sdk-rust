// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_encryption_metadata(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DataEncryptionMetadata) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.allow_cleartext {
        object.key("allowCleartext").boolean(*var_1);
    }
    if let Some(var_2) = &input.allow_duplicates {
        object.key("allowDuplicates").boolean(*var_2);
    }
    if let Some(var_3) = &input.allow_joins_on_columns_with_different_names {
        object.key("allowJoinsOnColumnsWithDifferentNames").boolean(*var_3);
    }
    if let Some(var_4) = &input.preserve_nulls {
        object.key("preserveNulls").boolean(*var_4);
    }
    Ok(())
}

pub(crate) fn de_data_encryption_metadata<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DataEncryptionMetadata>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::data_encryption_metadata::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "allowCleartext" => {
                                builder = builder.set_allow_cleartext(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "allowDuplicates" => {
                                builder = builder.set_allow_duplicates(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "allowJoinsOnColumnsWithDifferentNames" => {
                                builder = builder.set_allow_joins_on_columns_with_different_names(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "preserveNulls" => {
                                builder = builder.set_preserve_nulls(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

