// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_source_location<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SourceLocation>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::source_location::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AccessConfiguration" => {
                                builder = builder.set_access_configuration(
                                    crate::protocol_serde::shape_access_configuration::de_access_configuration(tokens)?
                                );
                            }
                            "Arn" => {
                                builder = builder.set_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CreationTime" => {
                                builder = builder.set_creation_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "DefaultSegmentDeliveryConfiguration" => {
                                builder = builder.set_default_segment_delivery_configuration(
                                    crate::protocol_serde::shape_default_segment_delivery_configuration::de_default_segment_delivery_configuration(tokens)?
                                );
                            }
                            "HttpConfiguration" => {
                                builder = builder.set_http_configuration(
                                    crate::protocol_serde::shape_http_configuration::de_http_configuration(tokens)?
                                );
                            }
                            "LastModifiedTime" => {
                                builder = builder.set_last_modified_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "SegmentDeliveryConfigurations" => {
                                builder = builder.set_segment_delivery_configurations(
                                    crate::protocol_serde::shape___list_of_segment_delivery_configuration::de___list_of_segment_delivery_configuration(tokens)?
                                );
                            }
                            "SourceLocationName" => {
                                builder = builder.set_source_location_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "tags" => {
                                builder = builder.set_tags(
                                    crate::protocol_serde::shape___map_of__string::de___map_of__string(tokens)?
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

