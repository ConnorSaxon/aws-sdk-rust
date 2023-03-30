// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_thing_indexing_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ThingIndexingConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::thing_indexing_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "thingIndexingMode" => {
                                builder = builder.set_thing_indexing_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ThingIndexingMode::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "thingConnectivityIndexingMode" => {
                                builder = builder.set_thing_connectivity_indexing_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ThingConnectivityIndexingMode::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "deviceDefenderIndexingMode" => {
                                builder = builder.set_device_defender_indexing_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::DeviceDefenderIndexingMode::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "namedShadowIndexingMode" => {
                                builder = builder.set_named_shadow_indexing_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::NamedShadowIndexingMode::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "managedFields" => {
                                builder = builder.set_managed_fields(
                                    crate::protocol_serde::shape_fields::de_fields(tokens)?
                                );
                            }
                            "customFields" => {
                                builder = builder.set_custom_fields(
                                    crate::protocol_serde::shape_fields::de_fields(tokens)?
                                );
                            }
                            "filter" => {
                                builder = builder.set_filter(
                                    crate::protocol_serde::shape_indexing_filter::de_indexing_filter(tokens)?
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

pub fn ser_thing_indexing_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ThingIndexingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.thing_indexing_mode {
        object.key("thingIndexingMode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.thing_connectivity_indexing_mode {
        object.key("thingConnectivityIndexingMode").string(var_2.as_str());
    }
    if let Some(var_3) = &input.device_defender_indexing_mode {
        object.key("deviceDefenderIndexingMode").string(var_3.as_str());
    }
    if let Some(var_4) = &input.named_shadow_indexing_mode {
        object.key("namedShadowIndexingMode").string(var_4.as_str());
    }
    if let Some(var_5) = &input.managed_fields {
        let mut array_6 = object.key("managedFields").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_field::ser_field(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.custom_fields {
        let mut array_10 = object.key("customFields").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_field::ser_field(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.filter {
        #[allow(unused_mut)]
        let mut object_14 = object.key("filter").start_object();
        crate::protocol_serde::shape_indexing_filter::ser_indexing_filter(&mut object_14, var_13)?;
        object_14.finish();
    }
    Ok(())
}

