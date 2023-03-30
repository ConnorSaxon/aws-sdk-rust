// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tree_map_aggregated_field_wells(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TreeMapAggregatedFieldWells) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.groups {
        let mut array_2 = object.key("Groups").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_dimension_field::ser_dimension_field(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.sizes {
        let mut array_6 = object.key("Sizes").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_measure_field::ser_measure_field(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.colors {
        let mut array_10 = object.key("Colors").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_measure_field::ser_measure_field(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub(crate) fn de_tree_map_aggregated_field_wells<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TreeMapAggregatedFieldWells>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::tree_map_aggregated_field_wells::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Groups" => {
                                builder = builder.set_groups(
                                    crate::protocol_serde::shape_tree_map_dimension_field_list::de_tree_map_dimension_field_list(tokens)?
                                );
                            }
                            "Sizes" => {
                                builder = builder.set_sizes(
                                    crate::protocol_serde::shape_tree_map_measure_field_list::de_tree_map_measure_field_list(tokens)?
                                );
                            }
                            "Colors" => {
                                builder = builder.set_colors(
                                    crate::protocol_serde::shape_tree_map_measure_field_list::de_tree_map_measure_field_list(tokens)?
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

