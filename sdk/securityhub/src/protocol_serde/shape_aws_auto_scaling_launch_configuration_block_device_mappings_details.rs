// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_auto_scaling_launch_configuration_block_device_mappings_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsAutoScalingLaunchConfigurationBlockDeviceMappingsDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.device_name {
        object.key("DeviceName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ebs {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Ebs").start_object();
        crate::protocol_serde::shape_aws_auto_scaling_launch_configuration_block_device_mappings_ebs_details::ser_aws_auto_scaling_launch_configuration_block_device_mappings_ebs_details(&mut object_3, var_2)?;
        object_3.finish();
    }
    if input.no_device {
        object.key("NoDevice").boolean(input.no_device);
    }
    if let Some(var_4) = &input.virtual_name {
        object.key("VirtualName").string(var_4.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_auto_scaling_launch_configuration_block_device_mappings_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsAutoScalingLaunchConfigurationBlockDeviceMappingsDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_auto_scaling_launch_configuration_block_device_mappings_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DeviceName" => {
                                builder = builder.set_device_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Ebs" => {
                                builder = builder.set_ebs(
                                    crate::protocol_serde::shape_aws_auto_scaling_launch_configuration_block_device_mappings_ebs_details::de_aws_auto_scaling_launch_configuration_block_device_mappings_ebs_details(tokens)?
                                );
                            }
                            "NoDevice" => {
                                builder = builder.set_no_device(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "VirtualName" => {
                                builder = builder.set_virtual_name(
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

