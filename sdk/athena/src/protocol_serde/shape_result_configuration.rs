// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_result_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ResultConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.output_location {
        object.key("OutputLocation").string(var_1.as_str());
    }
    if let Some(var_2) = &input.encryption_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("EncryptionConfiguration").start_object();
        crate::protocol_serde::shape_encryption_configuration::ser_encryption_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.expected_bucket_owner {
        object.key("ExpectedBucketOwner").string(var_4.as_str());
    }
    if let Some(var_5) = &input.acl_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("AclConfiguration").start_object();
        crate::protocol_serde::shape_acl_configuration::ser_acl_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_result_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ResultConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::result_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "OutputLocation" => {
                                builder = builder.set_output_location(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "EncryptionConfiguration" => {
                                builder = builder.set_encryption_configuration(
                                    crate::protocol_serde::shape_encryption_configuration::de_encryption_configuration(tokens)?
                                );
                            }
                            "ExpectedBucketOwner" => {
                                builder = builder.set_expected_bucket_owner(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "AclConfiguration" => {
                                builder = builder.set_acl_configuration(
                                    crate::protocol_serde::shape_acl_configuration::de_acl_configuration(tokens)?
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

