// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_event_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EventSelector) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.read_write_type {
        object.key("ReadWriteType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.include_management_events {
        object.key("IncludeManagementEvents").boolean(*var_2);
    }
    if let Some(var_3) = &input.data_resources {
        let mut array_4 = object.key("DataResources").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_data_resource::ser_data_resource(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.exclude_management_event_sources {
        let mut array_8 = object.key("ExcludeManagementEventSources").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    Ok(())
}

pub(crate) fn de_event_selector<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::EventSelector>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::event_selector::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ReadWriteType" => {
                                builder = builder.set_read_write_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ReadWriteType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "IncludeManagementEvents" => {
                                builder = builder.set_include_management_events(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "DataResources" => {
                                builder = builder.set_data_resources(
                                    crate::protocol_serde::shape_data_resources::de_data_resources(tokens)?
                                );
                            }
                            "ExcludeManagementEventSources" => {
                                builder = builder.set_exclude_management_event_sources(
                                    crate::protocol_serde::shape_exclude_management_event_sources::de_exclude_management_event_sources(tokens)?
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

