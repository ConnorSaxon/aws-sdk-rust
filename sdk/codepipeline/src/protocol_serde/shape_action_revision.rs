// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_action_revision(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ActionRevision) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.revision_id {
        object.key("revisionId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.revision_change_id {
        object.key("revisionChangeId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.created {
        object.key("created").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub(crate) fn de_action_revision<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ActionRevision>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::action_revision::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "revisionId" => {
                                builder = builder.set_revision_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "revisionChangeId" => {
                                builder = builder.set_revision_change_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "created" => {
                                builder = builder.set_created(
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

