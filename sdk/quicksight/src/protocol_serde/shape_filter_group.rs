// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_filter_group(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FilterGroup) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.filter_group_id {
        object.key("FilterGroupId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filters {
        let mut array_3 = object.key("Filters").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_filter::ser_filter(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.scope_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ScopeConfiguration").start_object();
        crate::protocol_serde::shape_filter_scope_configuration::ser_filter_scope_configuration(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.status {
        object.key("Status").string(var_8.as_str());
    }
    if let Some(var_9) = &input.cross_dataset {
        object.key("CrossDataset").string(var_9.as_str());
    }
    Ok(())
}

pub(crate) fn de_filter_group<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::FilterGroup>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::filter_group::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "FilterGroupId" => {
                                builder = builder.set_filter_group_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Filters" => {
                                builder = builder.set_filters(
                                    crate::protocol_serde::shape_filter_list::de_filter_list(tokens)?
                                );
                            }
                            "ScopeConfiguration" => {
                                builder = builder.set_scope_configuration(
                                    crate::protocol_serde::shape_filter_scope_configuration::de_filter_scope_configuration(tokens)?
                                );
                            }
                            "Status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::WidgetStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "CrossDataset" => {
                                builder = builder.set_cross_dataset(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CrossDatasetTypes::from(u.as_ref())
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

