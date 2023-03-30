// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_table_conditional_formatting_option(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TableConditionalFormattingOption) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cell {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Cell").start_object();
        crate::protocol_serde::shape_table_cell_conditional_formatting::ser_table_cell_conditional_formatting(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.row {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Row").start_object();
        crate::protocol_serde::shape_table_row_conditional_formatting::ser_table_row_conditional_formatting(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_table_conditional_formatting_option<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TableConditionalFormattingOption>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::table_conditional_formatting_option::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Cell" => {
                                builder = builder.set_cell(
                                    crate::protocol_serde::shape_table_cell_conditional_formatting::de_table_cell_conditional_formatting(tokens)?
                                );
                            }
                            "Row" => {
                                builder = builder.set_row(
                                    crate::protocol_serde::shape_table_row_conditional_formatting::de_table_row_conditional_formatting(tokens)?
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

