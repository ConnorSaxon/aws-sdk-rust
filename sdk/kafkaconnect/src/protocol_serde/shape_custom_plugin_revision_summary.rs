// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_custom_plugin_revision_summary<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CustomPluginRevisionSummary>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::custom_plugin_revision_summary::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "contentType" => {
                                builder = builder.set_content_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CustomPluginContentType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "creationTime" => {
                                builder = builder.set_creation_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::DateTime)?
                                );
                            }
                            "description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "fileDescription" => {
                                builder = builder.set_file_description(
                                    crate::protocol_serde::shape_custom_plugin_file_description::de_custom_plugin_file_description(tokens)?
                                );
                            }
                            "location" => {
                                builder = builder.set_location(
                                    crate::protocol_serde::shape_custom_plugin_location_description::de_custom_plugin_location_description(tokens)?
                                );
                            }
                            "revision" => {
                                builder = builder.set_revision(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
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

