// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_funnel_chart_sort_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FunnelChartSortConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.category_sort {
        let mut array_2 = object.key("CategorySort").start_array();
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
    if let Some(var_5) = &input.category_items_limit {
        #[allow(unused_mut)]
        let mut object_6 = object.key("CategoryItemsLimit").start_object();
        crate::protocol_serde::shape_items_limit_configuration::ser_items_limit_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_funnel_chart_sort_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::FunnelChartSortConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::funnel_chart_sort_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CategorySort" => {
                                builder = builder.set_category_sort(
                                    crate::protocol_serde::shape_field_sort_options_list::de_field_sort_options_list(tokens)?
                                );
                            }
                            "CategoryItemsLimit" => {
                                builder = builder.set_category_items_limit(
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

