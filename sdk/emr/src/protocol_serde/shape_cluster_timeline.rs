// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_cluster_timeline<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ClusterTimeline>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::cluster_timeline::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CreationDateTime" => {
                                builder = builder.set_creation_date_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "ReadyDateTime" => {
                                builder = builder.set_ready_date_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "EndDateTime" => {
                                builder = builder.set_end_date_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
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

