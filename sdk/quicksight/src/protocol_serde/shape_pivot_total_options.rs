// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_pivot_total_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PivotTotalOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.totals_visibility {
        object.key("TotalsVisibility").string(var_1.as_str());
    }
    if let Some(var_2) = &input.placement {
        object.key("Placement").string(var_2.as_str());
    }
    if let Some(var_3) = &input.scroll_status {
        object.key("ScrollStatus").string(var_3.as_str());
    }
    if let Some(var_4) = &input.custom_label {
        object.key("CustomLabel").string(var_4.as_str());
    }
    if let Some(var_5) = &input.total_cell_style {
        #[allow(unused_mut)]
        let mut object_6 = object.key("TotalCellStyle").start_object();
        crate::protocol_serde::shape_table_cell_style::ser_table_cell_style(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.value_cell_style {
        #[allow(unused_mut)]
        let mut object_8 = object.key("ValueCellStyle").start_object();
        crate::protocol_serde::shape_table_cell_style::ser_table_cell_style(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.metric_header_cell_style {
        #[allow(unused_mut)]
        let mut object_10 = object.key("MetricHeaderCellStyle").start_object();
        crate::protocol_serde::shape_table_cell_style::ser_table_cell_style(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}

pub(crate) fn de_pivot_total_options<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PivotTotalOptions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::pivot_total_options::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "TotalsVisibility" => {
                                builder = builder.set_totals_visibility(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::Visibility::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Placement" => {
                                builder = builder.set_placement(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TableTotalsPlacement::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ScrollStatus" => {
                                builder = builder.set_scroll_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TableTotalsScrollStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "CustomLabel" => {
                                builder = builder.set_custom_label(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TotalCellStyle" => {
                                builder = builder.set_total_cell_style(
                                    crate::protocol_serde::shape_table_cell_style::de_table_cell_style(tokens)?
                                );
                            }
                            "ValueCellStyle" => {
                                builder = builder.set_value_cell_style(
                                    crate::protocol_serde::shape_table_cell_style::de_table_cell_style(tokens)?
                                );
                            }
                            "MetricHeaderCellStyle" => {
                                builder = builder.set_metric_header_cell_style(
                                    crate::protocol_serde::shape_table_cell_style::de_table_cell_style(tokens)?
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

