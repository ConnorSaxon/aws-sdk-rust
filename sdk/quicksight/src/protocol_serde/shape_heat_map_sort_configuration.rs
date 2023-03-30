// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_heat_map_sort_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HeatMapSortConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.heat_map_row_sort {
        let mut array_2 = object.key("HeatMapRowSort").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_field_sort_options::ser_field_sort_options(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.heat_map_column_sort {
        let mut array_6 = object.key("HeatMapColumnSort").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_field_sort_options::ser_field_sort_options(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.heat_map_row_items_limit_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("HeatMapRowItemsLimitConfiguration").start_object();
        crate::protocol_serde::shape_items_limit_configuration::ser_items_limit_configuration(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.heat_map_column_items_limit_configuration {
        #[allow(unused_mut)]
        let mut object_12 = object.key("HeatMapColumnItemsLimitConfiguration").start_object();
        crate::protocol_serde::shape_items_limit_configuration::ser_items_limit_configuration(&mut object_12, var_11)?;
        object_12.finish();
    }
    Ok(())
}

pub(crate) fn de_heat_map_sort_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::HeatMapSortConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::heat_map_sort_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "HeatMapRowSort" => {
                                builder = builder.set_heat_map_row_sort(
                                    crate::protocol_serde::shape_field_sort_options_list::de_field_sort_options_list(tokens)?
                                );
                            }
                            "HeatMapColumnSort" => {
                                builder = builder.set_heat_map_column_sort(
                                    crate::protocol_serde::shape_field_sort_options_list::de_field_sort_options_list(tokens)?
                                );
                            }
                            "HeatMapRowItemsLimitConfiguration" => {
                                builder = builder.set_heat_map_row_items_limit_configuration(
                                    crate::protocol_serde::shape_items_limit_configuration::de_items_limit_configuration(tokens)?
                                );
                            }
                            "HeatMapColumnItemsLimitConfiguration" => {
                                builder = builder.set_heat_map_column_items_limit_configuration(
                                    crate::protocol_serde::shape_items_limit_configuration::de_items_limit_configuration(tokens)?
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

