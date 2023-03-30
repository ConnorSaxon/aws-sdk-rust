// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_cluster_pending_updates<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ClusterPendingUpdates>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::cluster_pending_updates::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Resharding" => {
                                builder = builder.set_resharding(
                                    crate::protocol_serde::shape_resharding_status::de_resharding_status(tokens)?
                                );
                            }
                            "ACLs" => {
                                builder = builder.set_ac_ls(
                                    crate::protocol_serde::shape_ac_ls_update_status::de_ac_ls_update_status(tokens)?
                                );
                            }
                            "ServiceUpdates" => {
                                builder = builder.set_service_updates(
                                    crate::protocol_serde::shape_pending_modified_service_update_list::de_pending_modified_service_update_list(tokens)?
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

