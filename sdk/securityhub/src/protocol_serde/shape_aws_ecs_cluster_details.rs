// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_ecs_cluster_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsEcsClusterDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster_arn {
        object.key("ClusterArn").string(var_1.as_str());
    }
    if input.active_services_count != 0 {
        object.key("ActiveServicesCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.active_services_count).into()));
    }
    if let Some(var_2) = &input.capacity_providers {
        let mut array_3 = object.key("CapacityProviders").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.cluster_settings {
        let mut array_6 = object.key("ClusterSettings").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_aws_ecs_cluster_cluster_settings_details::ser_aws_ecs_cluster_cluster_settings_details(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Configuration").start_object();
        crate::protocol_serde::shape_aws_ecs_cluster_configuration_details::ser_aws_ecs_cluster_configuration_details(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.default_capacity_provider_strategy {
        let mut array_12 = object.key("DefaultCapacityProviderStrategy").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_aws_ecs_cluster_default_capacity_provider_strategy_details::ser_aws_ecs_cluster_default_capacity_provider_strategy_details(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.cluster_name {
        object.key("ClusterName").string(var_15.as_str());
    }
    if input.registered_container_instances_count != 0 {
        object.key("RegisteredContainerInstancesCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.registered_container_instances_count).into()));
    }
    if input.running_tasks_count != 0 {
        object.key("RunningTasksCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.running_tasks_count).into()));
    }
    if let Some(var_16) = &input.status {
        object.key("Status").string(var_16.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_ecs_cluster_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsEcsClusterDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_ecs_cluster_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ClusterArn" => {
                                builder = builder.set_cluster_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ActiveServicesCount" => {
                                builder = builder.set_active_services_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CapacityProviders" => {
                                builder = builder.set_capacity_providers(
                                    crate::protocol_serde::shape_non_empty_string_list::de_non_empty_string_list(tokens)?
                                );
                            }
                            "ClusterSettings" => {
                                builder = builder.set_cluster_settings(
                                    crate::protocol_serde::shape_aws_ecs_cluster_cluster_settings_list::de_aws_ecs_cluster_cluster_settings_list(tokens)?
                                );
                            }
                            "Configuration" => {
                                builder = builder.set_configuration(
                                    crate::protocol_serde::shape_aws_ecs_cluster_configuration_details::de_aws_ecs_cluster_configuration_details(tokens)?
                                );
                            }
                            "DefaultCapacityProviderStrategy" => {
                                builder = builder.set_default_capacity_provider_strategy(
                                    crate::protocol_serde::shape_aws_ecs_cluster_default_capacity_provider_strategy_list::de_aws_ecs_cluster_default_capacity_provider_strategy_list(tokens)?
                                );
                            }
                            "ClusterName" => {
                                builder = builder.set_cluster_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RegisteredContainerInstancesCount" => {
                                builder = builder.set_registered_container_instances_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "RunningTasksCount" => {
                                builder = builder.set_running_tasks_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "Status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
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

