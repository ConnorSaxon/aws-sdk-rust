// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_domain_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DomainConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::domain_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "EngineVersion" => {
                                builder = builder.set_engine_version(
                                    crate::protocol_serde::shape_version_status::de_version_status(tokens)?
                                );
                            }
                            "ClusterConfig" => {
                                builder = builder.set_cluster_config(
                                    crate::protocol_serde::shape_cluster_config_status::de_cluster_config_status(tokens)?
                                );
                            }
                            "EBSOptions" => {
                                builder = builder.set_ebs_options(
                                    crate::protocol_serde::shape_ebs_options_status::de_ebs_options_status(tokens)?
                                );
                            }
                            "AccessPolicies" => {
                                builder = builder.set_access_policies(
                                    crate::protocol_serde::shape_access_policies_status::de_access_policies_status(tokens)?
                                );
                            }
                            "SnapshotOptions" => {
                                builder = builder.set_snapshot_options(
                                    crate::protocol_serde::shape_snapshot_options_status::de_snapshot_options_status(tokens)?
                                );
                            }
                            "VPCOptions" => {
                                builder = builder.set_vpc_options(
                                    crate::protocol_serde::shape_vpc_derived_info_status::de_vpc_derived_info_status(tokens)?
                                );
                            }
                            "CognitoOptions" => {
                                builder = builder.set_cognito_options(
                                    crate::protocol_serde::shape_cognito_options_status::de_cognito_options_status(tokens)?
                                );
                            }
                            "EncryptionAtRestOptions" => {
                                builder = builder.set_encryption_at_rest_options(
                                    crate::protocol_serde::shape_encryption_at_rest_options_status::de_encryption_at_rest_options_status(tokens)?
                                );
                            }
                            "NodeToNodeEncryptionOptions" => {
                                builder = builder.set_node_to_node_encryption_options(
                                    crate::protocol_serde::shape_node_to_node_encryption_options_status::de_node_to_node_encryption_options_status(tokens)?
                                );
                            }
                            "AdvancedOptions" => {
                                builder = builder.set_advanced_options(
                                    crate::protocol_serde::shape_advanced_options_status::de_advanced_options_status(tokens)?
                                );
                            }
                            "LogPublishingOptions" => {
                                builder = builder.set_log_publishing_options(
                                    crate::protocol_serde::shape_log_publishing_options_status::de_log_publishing_options_status(tokens)?
                                );
                            }
                            "DomainEndpointOptions" => {
                                builder = builder.set_domain_endpoint_options(
                                    crate::protocol_serde::shape_domain_endpoint_options_status::de_domain_endpoint_options_status(tokens)?
                                );
                            }
                            "AdvancedSecurityOptions" => {
                                builder = builder.set_advanced_security_options(
                                    crate::protocol_serde::shape_advanced_security_options_status::de_advanced_security_options_status(tokens)?
                                );
                            }
                            "AutoTuneOptions" => {
                                builder = builder.set_auto_tune_options(
                                    crate::protocol_serde::shape_auto_tune_options_status::de_auto_tune_options_status(tokens)?
                                );
                            }
                            "ChangeProgressDetails" => {
                                builder = builder.set_change_progress_details(
                                    crate::protocol_serde::shape_change_progress_details::de_change_progress_details(tokens)?
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

