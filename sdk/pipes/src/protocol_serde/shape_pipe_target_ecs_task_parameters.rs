// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_pipe_target_ecs_task_parameters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PipeTargetEcsTaskParameters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.task_definition_arn {
        object.key("TaskDefinitionArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.task_count {
        object.key("TaskCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.launch_type {
        object.key("LaunchType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("NetworkConfiguration").start_object();
        crate::protocol_serde::shape_network_configuration::ser_network_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.platform_version {
        object.key("PlatformVersion").string(var_6.as_str());
    }
    if let Some(var_7) = &input.group {
        object.key("Group").string(var_7.as_str());
    }
    if let Some(var_8) = &input.capacity_provider_strategy {
        let mut array_9 = object.key("CapacityProviderStrategy").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_capacity_provider_strategy_item::ser_capacity_provider_strategy_item(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if input.enable_ecs_managed_tags {
        object.key("EnableECSManagedTags").boolean(input.enable_ecs_managed_tags);
    }
    if input.enable_execute_command {
        object.key("EnableExecuteCommand").boolean(input.enable_execute_command);
    }
    if let Some(var_12) = &input.placement_constraints {
        let mut array_13 = object.key("PlacementConstraints").start_array();
        for item_14 in var_12 {
             {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_placement_constraint::ser_placement_constraint(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.placement_strategy {
        let mut array_17 = object.key("PlacementStrategy").start_array();
        for item_18 in var_16 {
             {
                #[allow(unused_mut)]
                let mut object_19 = array_17.value().start_object();
                crate::protocol_serde::shape_placement_strategy::ser_placement_strategy(&mut object_19, item_18)?;
                object_19.finish();
            }
        }
        array_17.finish();
    }
    if let Some(var_20) = &input.propagate_tags {
        object.key("PropagateTags").string(var_20.as_str());
    }
    if let Some(var_21) = &input.reference_id {
        object.key("ReferenceId").string(var_21.as_str());
    }
    if let Some(var_22) = &input.overrides {
        #[allow(unused_mut)]
        let mut object_23 = object.key("Overrides").start_object();
        crate::protocol_serde::shape_ecs_task_override::ser_ecs_task_override(&mut object_23, var_22)?;
        object_23.finish();
    }
    if let Some(var_24) = &input.tags {
        let mut array_25 = object.key("Tags").start_array();
        for item_26 in var_24 {
             {
                #[allow(unused_mut)]
                let mut object_27 = array_25.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_27, item_26)?;
                object_27.finish();
            }
        }
        array_25.finish();
    }
    Ok(())
}

pub(crate) fn de_pipe_target_ecs_task_parameters<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PipeTargetEcsTaskParameters>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::pipe_target_ecs_task_parameters::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "TaskDefinitionArn" => {
                                builder = builder.set_task_definition_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TaskCount" => {
                                builder = builder.set_task_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "LaunchType" => {
                                builder = builder.set_launch_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::LaunchType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "NetworkConfiguration" => {
                                builder = builder.set_network_configuration(
                                    crate::protocol_serde::shape_network_configuration::de_network_configuration(tokens)?
                                );
                            }
                            "PlatformVersion" => {
                                builder = builder.set_platform_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Group" => {
                                builder = builder.set_group(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CapacityProviderStrategy" => {
                                builder = builder.set_capacity_provider_strategy(
                                    crate::protocol_serde::shape_capacity_provider_strategy::de_capacity_provider_strategy(tokens)?
                                );
                            }
                            "EnableECSManagedTags" => {
                                builder = builder.set_enable_ecs_managed_tags(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "EnableExecuteCommand" => {
                                builder = builder.set_enable_execute_command(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "PlacementConstraints" => {
                                builder = builder.set_placement_constraints(
                                    crate::protocol_serde::shape_placement_constraints::de_placement_constraints(tokens)?
                                );
                            }
                            "PlacementStrategy" => {
                                builder = builder.set_placement_strategy(
                                    crate::protocol_serde::shape_placement_strategies::de_placement_strategies(tokens)?
                                );
                            }
                            "PropagateTags" => {
                                builder = builder.set_propagate_tags(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::PropagateTags::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ReferenceId" => {
                                builder = builder.set_reference_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Overrides" => {
                                builder = builder.set_overrides(
                                    crate::protocol_serde::shape_ecs_task_override::de_ecs_task_override(tokens)?
                                );
                            }
                            "Tags" => {
                                builder = builder.set_tags(
                                    crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?
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

