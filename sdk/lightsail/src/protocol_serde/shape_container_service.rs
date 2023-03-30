// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_container_service<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ContainerService>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::container_service::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "containerServiceName" => {
                                builder = builder.set_container_service_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "arn" => {
                                builder = builder.set_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "createdAt" => {
                                builder = builder.set_created_at(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "location" => {
                                builder = builder.set_location(
                                    crate::protocol_serde::shape_resource_location::de_resource_location(tokens)?
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
                            "tags" => {
                                builder = builder.set_tags(
                                    crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?
                                );
                            }
                            "power" => {
                                builder = builder.set_power(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ContainerServicePowerName::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "powerId" => {
                                builder = builder.set_power_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "state" => {
                                builder = builder.set_state(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ContainerServiceState::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "stateDetail" => {
                                builder = builder.set_state_detail(
                                    crate::protocol_serde::shape_container_service_state_detail::de_container_service_state_detail(tokens)?
                                );
                            }
                            "scale" => {
                                builder = builder.set_scale(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "currentDeployment" => {
                                builder = builder.set_current_deployment(
                                    crate::protocol_serde::shape_container_service_deployment::de_container_service_deployment(tokens)?
                                );
                            }
                            "nextDeployment" => {
                                builder = builder.set_next_deployment(
                                    crate::protocol_serde::shape_container_service_deployment::de_container_service_deployment(tokens)?
                                );
                            }
                            "isDisabled" => {
                                builder = builder.set_is_disabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "principalArn" => {
                                builder = builder.set_principal_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "privateDomainName" => {
                                builder = builder.set_private_domain_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "publicDomainNames" => {
                                builder = builder.set_public_domain_names(
                                    crate::protocol_serde::shape_container_service_public_domains::de_container_service_public_domains(tokens)?
                                );
                            }
                            "url" => {
                                builder = builder.set_url(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "privateRegistryAccess" => {
                                builder = builder.set_private_registry_access(
                                    crate::protocol_serde::shape_private_registry_access::de_private_registry_access(tokens)?
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

