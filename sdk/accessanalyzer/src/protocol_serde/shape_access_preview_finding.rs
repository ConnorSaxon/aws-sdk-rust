// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_access_preview_finding<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AccessPreviewFinding>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::access_preview_finding::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "id" => {
                                builder = builder.set_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "existingFindingId" => {
                                builder = builder.set_existing_finding_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "existingFindingStatus" => {
                                builder = builder.set_existing_finding_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::FindingStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "principal" => {
                                builder = builder.set_principal(
                                    crate::protocol_serde::shape_principal_map::de_principal_map(tokens)?
                                );
                            }
                            "action" => {
                                builder = builder.set_action(
                                    crate::protocol_serde::shape_action_list::de_action_list(tokens)?
                                );
                            }
                            "condition" => {
                                builder = builder.set_condition(
                                    crate::protocol_serde::shape_condition_key_map::de_condition_key_map(tokens)?
                                );
                            }
                            "resource" => {
                                builder = builder.set_resource(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "isPublic" => {
                                builder = builder.set_is_public(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "resourceType" => {
                                builder = builder.set_resource_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ResourceType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "createdAt" => {
                                builder = builder.set_created_at(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::DateTime)?
                                );
                            }
                            "changeType" => {
                                builder = builder.set_change_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::FindingChangeType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::FindingStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "resourceOwnerAccount" => {
                                builder = builder.set_resource_owner_account(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "error" => {
                                builder = builder.set_error(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "sources" => {
                                builder = builder.set_sources(
                                    crate::protocol_serde::shape_finding_source_list::de_finding_source_list(tokens)?
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

