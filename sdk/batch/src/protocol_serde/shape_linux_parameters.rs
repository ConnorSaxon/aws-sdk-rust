// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_linux_parameters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LinuxParameters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.devices {
        let mut array_2 = object.key("devices").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_device::ser_device(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.init_process_enabled {
        object.key("initProcessEnabled").boolean(*var_5);
    }
    if let Some(var_6) = &input.shared_memory_size {
        object.key("sharedMemorySize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    if let Some(var_7) = &input.tmpfs {
        let mut array_8 = object.key("tmpfs").start_array();
        for item_9 in var_7 {
             {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_tmpfs::ser_tmpfs(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.max_swap {
        object.key("maxSwap").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_11).into()));
    }
    if let Some(var_12) = &input.swappiness {
        object.key("swappiness").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    Ok(())
}

pub(crate) fn de_linux_parameters<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::LinuxParameters>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::linux_parameters::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "devices" => {
                                builder = builder.set_devices(
                                    crate::protocol_serde::shape_devices_list::de_devices_list(tokens)?
                                );
                            }
                            "initProcessEnabled" => {
                                builder = builder.set_init_process_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "sharedMemorySize" => {
                                builder = builder.set_shared_memory_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "tmpfs" => {
                                builder = builder.set_tmpfs(
                                    crate::protocol_serde::shape_tmpfs_list::de_tmpfs_list(tokens)?
                                );
                            }
                            "maxSwap" => {
                                builder = builder.set_max_swap(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "swappiness" => {
                                builder = builder.set_swappiness(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
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

