// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_key_metadata<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::KeyMetadata>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::key_metadata::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AWSAccountId" => {
                                builder = builder.set_aws_account_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "KeyId" => {
                                builder = builder.set_key_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
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
                            "CreationDate" => {
                                builder = builder.set_creation_date(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "Enabled" => {
                                builder = builder.set_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "KeyUsage" => {
                                builder = builder.set_key_usage(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::KeyUsageType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "KeyState" => {
                                builder = builder.set_key_state(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::KeyState::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "DeletionDate" => {
                                builder = builder.set_deletion_date(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "ValidTo" => {
                                builder = builder.set_valid_to(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "Origin" => {
                                builder = builder.set_origin(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::OriginType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "CustomKeyStoreId" => {
                                builder = builder.set_custom_key_store_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CloudHsmClusterId" => {
                                builder = builder.set_cloud_hsm_cluster_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ExpirationModel" => {
                                builder = builder.set_expiration_model(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ExpirationModelType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "KeyManager" => {
                                builder = builder.set_key_manager(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::KeyManagerType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "CustomerMasterKeySpec" => {
                                builder = builder.set_customer_master_key_spec(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CustomerMasterKeySpec::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "KeySpec" => {
                                builder = builder.set_key_spec(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::KeySpec::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "EncryptionAlgorithms" => {
                                builder = builder.set_encryption_algorithms(
                                    crate::protocol_serde::shape_encryption_algorithm_spec_list::de_encryption_algorithm_spec_list(tokens)?
                                );
                            }
                            "SigningAlgorithms" => {
                                builder = builder.set_signing_algorithms(
                                    crate::protocol_serde::shape_signing_algorithm_spec_list::de_signing_algorithm_spec_list(tokens)?
                                );
                            }
                            "MultiRegion" => {
                                builder = builder.set_multi_region(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "MultiRegionConfiguration" => {
                                builder = builder.set_multi_region_configuration(
                                    crate::protocol_serde::shape_multi_region_configuration::de_multi_region_configuration(tokens)?
                                );
                            }
                            "PendingDeletionWindowInDays" => {
                                builder = builder.set_pending_deletion_window_in_days(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "MacAlgorithms" => {
                                builder = builder.set_mac_algorithms(
                                    crate::protocol_serde::shape_mac_algorithm_spec_list::de_mac_algorithm_spec_list(tokens)?
                                );
                            }
                            "XksKeyConfiguration" => {
                                builder = builder.set_xks_key_configuration(
                                    crate::protocol_serde::shape_xks_key_configuration_type::de_xks_key_configuration_type(tokens)?
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

