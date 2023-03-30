// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_hls_content_protection(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HlsContentProtection) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.method {
        object.key("Method").string(var_1.as_str());
    }
    if let Some(var_2) = &input.key {
        object.key("Key").string(var_2.as_str());
    }
    if let Some(var_3) = &input.key_md5 {
        object.key("KeyMd5").string(var_3.as_str());
    }
    if let Some(var_4) = &input.initialization_vector {
        object.key("InitializationVector").string(var_4.as_str());
    }
    if let Some(var_5) = &input.license_acquisition_url {
        object.key("LicenseAcquisitionUrl").string(var_5.as_str());
    }
    if let Some(var_6) = &input.key_storage_policy {
        object.key("KeyStoragePolicy").string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_hls_content_protection<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::HlsContentProtection>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::hls_content_protection::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Method" => {
                                builder = builder.set_method(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Key" => {
                                builder = builder.set_key(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "KeyMd5" => {
                                builder = builder.set_key_md5(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "InitializationVector" => {
                                builder = builder.set_initialization_vector(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "LicenseAcquisitionUrl" => {
                                builder = builder.set_license_acquisition_url(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "KeyStoragePolicy" => {
                                builder = builder.set_key_storage_policy(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
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

