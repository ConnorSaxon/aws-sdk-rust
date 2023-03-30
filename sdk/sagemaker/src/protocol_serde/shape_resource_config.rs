// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resource_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ResourceConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.instance_type {
        object.key("InstanceType").string(var_1.as_str());
    }
    if input.instance_count != 0 {
        object.key("InstanceCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.instance_count).into()));
    }
     {
        object.key("VolumeSizeInGB").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.volume_size_in_gb).into()));
    }
    if let Some(var_2) = &input.volume_kms_key_id {
        object.key("VolumeKmsKeyId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_groups {
        let mut array_4 = object.key("InstanceGroups").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_instance_group::ser_instance_group(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.keep_alive_period_in_seconds {
        object.key("KeepAlivePeriodInSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    Ok(())
}

pub(crate) fn de_resource_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ResourceConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::resource_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "InstanceType" => {
                                builder = builder.set_instance_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TrainingInstanceType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "InstanceCount" => {
                                builder = builder.set_instance_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "VolumeSizeInGB" => {
                                builder = builder.set_volume_size_in_gb(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "VolumeKmsKeyId" => {
                                builder = builder.set_volume_kms_key_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "InstanceGroups" => {
                                builder = builder.set_instance_groups(
                                    crate::protocol_serde::shape_instance_groups::de_instance_groups(tokens)?
                                );
                            }
                            "KeepAlivePeriodInSeconds" => {
                                builder = builder.set_keep_alive_period_in_seconds(
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

