// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_excel_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ExcelOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.sheet_names {
        let mut array_2 = object.key("SheetNames").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.sheet_indexes {
        let mut array_5 = object.key("SheetIndexes").start_array();
        for item_6 in var_4 {
             {
                array_5.value().number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*item_6).into()));
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.header_row {
        object.key("HeaderRow").boolean(*var_7);
    }
    Ok(())
}

pub(crate) fn de_excel_options<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ExcelOptions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::excel_options::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SheetNames" => {
                                builder = builder.set_sheet_names(
                                    crate::protocol_serde::shape_sheet_name_list::de_sheet_name_list(tokens)?
                                );
                            }
                            "SheetIndexes" => {
                                builder = builder.set_sheet_indexes(
                                    crate::protocol_serde::shape_sheet_index_list::de_sheet_index_list(tokens)?
                                );
                            }
                            "HeaderRow" => {
                                builder = builder.set_header_row(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

