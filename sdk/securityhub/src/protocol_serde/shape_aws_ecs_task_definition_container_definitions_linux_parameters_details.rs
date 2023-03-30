// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_ecs_task_definition_container_definitions_linux_parameters_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.capabilities {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Capabilities").start_object();
        crate::protocol_serde::shape_aws_ecs_task_definition_container_definitions_linux_parameters_capabilities_details::ser_aws_ecs_task_definition_container_definitions_linux_parameters_capabilities_details(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.devices {
        let mut array_4 = object.key("Devices").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_aws_ecs_task_definition_container_definitions_linux_parameters_devices_details::ser_aws_ecs_task_definition_container_definitions_linux_parameters_devices_details(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if input.init_process_enabled {
        object.key("InitProcessEnabled").boolean(input.init_process_enabled);
    }
    if input.max_swap != 0 {
        object.key("MaxSwap").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_swap).into()));
    }
    if input.shared_memory_size != 0 {
        object.key("SharedMemorySize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.shared_memory_size).into()));
    }
    if input.swappiness != 0 {
        object.key("Swappiness").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.swappiness).into()));
    }
    if let Some(var_7) = &input.tmpfs {
        let mut array_8 = object.key("Tmpfs").start_array();
        for item_9 in var_7 {
             {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_aws_ecs_task_definition_container_definitions_linux_parameters_tmpfs_details::ser_aws_ecs_task_definition_container_definitions_linux_parameters_tmpfs_details(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_ecs_task_definition_container_definitions_linux_parameters_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_ecs_task_definition_container_definitions_linux_parameters_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Capabilities" => {
                                builder = builder.set_capabilities(
                                    crate::protocol_serde::shape_aws_ecs_task_definition_container_definitions_linux_parameters_capabilities_details::de_aws_ecs_task_definition_container_definitions_linux_parameters_capabilities_details(tokens)?
                                );
                            }
                            "Devices" => {
                                builder = builder.set_devices(
                                    crate::protocol_serde::shape_aws_ecs_task_definition_container_definitions_linux_parameters_devices_list::de_aws_ecs_task_definition_container_definitions_linux_parameters_devices_list(tokens)?
                                );
                            }
                            "InitProcessEnabled" => {
                                builder = builder.set_init_process_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "MaxSwap" => {
                                builder = builder.set_max_swap(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "SharedMemorySize" => {
                                builder = builder.set_shared_memory_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "Swappiness" => {
                                builder = builder.set_swappiness(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "Tmpfs" => {
                                builder = builder.set_tmpfs(
                                    crate::protocol_serde::shape_aws_ecs_task_definition_container_definitions_linux_parameters_tmpfs_list::de_aws_ecs_task_definition_container_definitions_linux_parameters_tmpfs_list(tokens)?
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

