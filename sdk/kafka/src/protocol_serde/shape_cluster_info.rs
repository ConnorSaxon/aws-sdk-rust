// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_cluster_info<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ClusterInfo>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::cluster_info::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "activeOperationArn" => {
                                builder = builder.set_active_operation_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "brokerNodeGroupInfo" => {
                                builder = builder.set_broker_node_group_info(
                                    crate::protocol_serde::shape_broker_node_group_info::de_broker_node_group_info(tokens)?
                                );
                            }
                            "clientAuthentication" => {
                                builder = builder.set_client_authentication(
                                    crate::protocol_serde::shape_client_authentication::de_client_authentication(tokens)?
                                );
                            }
                            "clusterArn" => {
                                builder = builder.set_cluster_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "clusterName" => {
                                builder = builder.set_cluster_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "creationTime" => {
                                builder = builder.set_creation_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::DateTime)?
                                );
                            }
                            "currentBrokerSoftwareInfo" => {
                                builder = builder.set_current_broker_software_info(
                                    crate::protocol_serde::shape_broker_software_info::de_broker_software_info(tokens)?
                                );
                            }
                            "currentVersion" => {
                                builder = builder.set_current_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "encryptionInfo" => {
                                builder = builder.set_encryption_info(
                                    crate::protocol_serde::shape_encryption_info::de_encryption_info(tokens)?
                                );
                            }
                            "enhancedMonitoring" => {
                                builder = builder.set_enhanced_monitoring(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::EnhancedMonitoring::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "openMonitoring" => {
                                builder = builder.set_open_monitoring(
                                    crate::protocol_serde::shape_open_monitoring::de_open_monitoring(tokens)?
                                );
                            }
                            "loggingInfo" => {
                                builder = builder.set_logging_info(
                                    crate::protocol_serde::shape_logging_info::de_logging_info(tokens)?
                                );
                            }
                            "numberOfBrokerNodes" => {
                                builder = builder.set_number_of_broker_nodes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "state" => {
                                builder = builder.set_state(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ClusterState::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "stateInfo" => {
                                builder = builder.set_state_info(
                                    crate::protocol_serde::shape_state_info::de_state_info(tokens)?
                                );
                            }
                            "tags" => {
                                builder = builder.set_tags(
                                    crate::protocol_serde::shape___map_of__string::de___map_of__string(tokens)?
                                );
                            }
                            "zookeeperConnectString" => {
                                builder = builder.set_zookeeper_connect_string(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "zookeeperConnectStringTls" => {
                                builder = builder.set_zookeeper_connect_string_tls(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "storageMode" => {
                                builder = builder.set_storage_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::StorageMode::from(u.as_ref())
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

