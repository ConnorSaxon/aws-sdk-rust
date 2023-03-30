// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_bar_chart_visual(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BarChartVisual) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.visual_id {
        object.key("VisualId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.title {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Title").start_object();
        crate::protocol_serde::shape_visual_title_label_options::ser_visual_title_label_options(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.subtitle {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Subtitle").start_object();
        crate::protocol_serde::shape_visual_subtitle_label_options::ser_visual_subtitle_label_options(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.chart_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ChartConfiguration").start_object();
        crate::protocol_serde::shape_bar_chart_configuration::ser_bar_chart_configuration(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.actions {
        let mut array_9 = object.key("Actions").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_visual_custom_action::ser_visual_custom_action(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.column_hierarchies {
        let mut array_13 = object.key("ColumnHierarchies").start_array();
        for item_14 in var_12 {
             {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_column_hierarchy::ser_column_hierarchy(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    Ok(())
}

pub(crate) fn de_bar_chart_visual<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::BarChartVisual>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::bar_chart_visual::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "VisualId" => {
                                builder = builder.set_visual_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Title" => {
                                builder = builder.set_title(
                                    crate::protocol_serde::shape_visual_title_label_options::de_visual_title_label_options(tokens)?
                                );
                            }
                            "Subtitle" => {
                                builder = builder.set_subtitle(
                                    crate::protocol_serde::shape_visual_subtitle_label_options::de_visual_subtitle_label_options(tokens)?
                                );
                            }
                            "ChartConfiguration" => {
                                builder = builder.set_chart_configuration(
                                    crate::protocol_serde::shape_bar_chart_configuration::de_bar_chart_configuration(tokens)?
                                );
                            }
                            "Actions" => {
                                builder = builder.set_actions(
                                    crate::protocol_serde::shape_visual_custom_action_list::de_visual_custom_action_list(tokens)?
                                );
                            }
                            "ColumnHierarchies" => {
                                builder = builder.set_column_hierarchies(
                                    crate::protocol_serde::shape_column_hierarchy_list::de_column_hierarchy_list(tokens)?
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

