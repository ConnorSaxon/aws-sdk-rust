// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_property_type(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PropertyType) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attribute {
        #[allow(unused_mut)]
        let mut object_2 = object.key("attribute").start_object();
        crate::protocol_serde::shape_attribute::ser_attribute(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.measurement {
        #[allow(unused_mut)]
        let mut object_4 = object.key("measurement").start_object();
        crate::protocol_serde::shape_measurement::ser_measurement(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.transform {
        #[allow(unused_mut)]
        let mut object_6 = object.key("transform").start_object();
        crate::protocol_serde::shape_transform::ser_transform(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.metric {
        #[allow(unused_mut)]
        let mut object_8 = object.key("metric").start_object();
        crate::protocol_serde::shape_metric::ser_metric(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}

pub(crate) fn de_property_type<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PropertyType>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::property_type::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "attribute" => {
                                builder = builder.set_attribute(
                                    crate::protocol_serde::shape_attribute::de_attribute(tokens)?
                                );
                            }
                            "measurement" => {
                                builder = builder.set_measurement(
                                    crate::protocol_serde::shape_measurement::de_measurement(tokens)?
                                );
                            }
                            "transform" => {
                                builder = builder.set_transform(
                                    crate::protocol_serde::shape_transform::de_transform(tokens)?
                                );
                            }
                            "metric" => {
                                builder = builder.set_metric(
                                    crate::protocol_serde::shape_metric::de_metric(tokens)?
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

