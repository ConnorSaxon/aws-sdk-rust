// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_target_platform(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TargetPlatform) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.os {
        object.key("Os").string(var_1.as_str());
    }
    if let Some(var_2) = &input.arch {
        object.key("Arch").string(var_2.as_str());
    }
    if let Some(var_3) = &input.accelerator {
        object.key("Accelerator").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_target_platform<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TargetPlatform>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::target_platform::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Os" => {
                                builder = builder.set_os(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TargetPlatformOs::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Arch" => {
                                builder = builder.set_arch(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TargetPlatformArch::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Accelerator" => {
                                builder = builder.set_accelerator(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TargetPlatformAccelerator::from(u.as_ref())
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

