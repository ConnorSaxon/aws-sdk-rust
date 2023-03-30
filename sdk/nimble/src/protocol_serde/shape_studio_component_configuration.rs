// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_studio_component_configuration(object_2: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StudioComponentConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::StudioComponentConfiguration::ActiveDirectoryConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_1 = object_2.key("activeDirectoryConfiguration").start_object();
                crate::protocol_serde::shape_active_directory_configuration::ser_active_directory_configuration(&mut object_1, inner)?;
                object_1.finish();
            }
        },
        crate::model::StudioComponentConfiguration::ComputeFarmConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_2 = object_2.key("computeFarmConfiguration").start_object();
                crate::protocol_serde::shape_compute_farm_configuration::ser_compute_farm_configuration(&mut object_2, inner)?;
                object_2.finish();
            }
        },
        crate::model::StudioComponentConfiguration::LicenseServiceConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_3 = object_2.key("licenseServiceConfiguration").start_object();
                crate::protocol_serde::shape_license_service_configuration::ser_license_service_configuration(&mut object_3, inner)?;
                object_3.finish();
            }
        },
        crate::model::StudioComponentConfiguration::SharedFileSystemConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_4 = object_2.key("sharedFileSystemConfiguration").start_object();
                crate::protocol_serde::shape_shared_file_system_configuration::ser_shared_file_system_configuration(&mut object_4, inner)?;
                object_4.finish();
            }
        },
        crate::model::StudioComponentConfiguration::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("StudioComponentConfiguration"))
    }
    Ok(())
}

pub(crate) fn de_studio_component_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::StudioComponentConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
                                Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        if variant.is_some() {
                                                            return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("encountered mixed variants in union"));
                                                        }
                        variant = match key.to_unescaped()?.as_ref() {
                            "activeDirectoryConfiguration" => {
                                Some(crate::model::StudioComponentConfiguration::ActiveDirectoryConfiguration(
                                    crate::protocol_serde::shape_active_directory_configuration::de_active_directory_configuration(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'activeDirectoryConfiguration' cannot be null"))?
                                ))
                            }
                            "computeFarmConfiguration" => {
                                Some(crate::model::StudioComponentConfiguration::ComputeFarmConfiguration(
                                    crate::protocol_serde::shape_compute_farm_configuration::de_compute_farm_configuration(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'computeFarmConfiguration' cannot be null"))?
                                ))
                            }
                            "licenseServiceConfiguration" => {
                                Some(crate::model::StudioComponentConfiguration::LicenseServiceConfiguration(
                                    crate::protocol_serde::shape_license_service_configuration::de_license_service_configuration(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'licenseServiceConfiguration' cannot be null"))?
                                ))
                            }
                            "sharedFileSystemConfiguration" => {
                                Some(crate::model::StudioComponentConfiguration::SharedFileSystemConfiguration(
                                    crate::protocol_serde::shape_shared_file_system_configuration::de_shared_file_system_configuration(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'sharedFileSystemConfiguration' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                      aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::model::StudioComponentConfiguration::Unknown)
                                                                    }
                        };
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
        }
        _ => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
    }
    Ok(variant)
}

