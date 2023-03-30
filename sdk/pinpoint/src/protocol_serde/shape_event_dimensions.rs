// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_event_dimensions(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EventDimensions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Attributes").start_object();
        for (key_3, value_4) in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_5 = object_2.key(key_3.as_str()).start_object();
                crate::protocol_serde::shape_attribute_dimension::ser_attribute_dimension(&mut object_5, value_4)?;
                object_5.finish();
            }
        }
        object_2.finish();
    }
    if let Some(var_6) = &input.event_type {
        #[allow(unused_mut)]
        let mut object_7 = object.key("EventType").start_object();
        crate::protocol_serde::shape_set_dimension::ser_set_dimension(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.metrics {
        #[allow(unused_mut)]
        let mut object_9 = object.key("Metrics").start_object();
        for (key_10, value_11) in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_12 = object_9.key(key_10.as_str()).start_object();
                crate::protocol_serde::shape_metric_dimension::ser_metric_dimension(&mut object_12, value_11)?;
                object_12.finish();
            }
        }
        object_9.finish();
    }
    Ok(())
}

pub(crate) fn de_event_dimensions<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::EventDimensions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::event_dimensions::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Attributes" => {
                                builder = builder.set_attributes(
                                    crate::protocol_serde::shape_map_of_attribute_dimension::de_map_of_attribute_dimension(tokens)?
                                );
                            }
                            "EventType" => {
                                builder = builder.set_event_type(
                                    crate::protocol_serde::shape_set_dimension::de_set_dimension(tokens)?
                                );
                            }
                            "Metrics" => {
                                builder = builder.set_metrics(
                                    crate::protocol_serde::shape_map_of_metric_dimension::de_map_of_metric_dimension(tokens)?
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

