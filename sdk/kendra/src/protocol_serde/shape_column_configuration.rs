// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_column_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ColumnConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.document_id_column_name {
        object.key("DocumentIdColumnName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.document_data_column_name {
        object.key("DocumentDataColumnName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.document_title_column_name {
        object.key("DocumentTitleColumnName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.field_mappings {
        let mut array_5 = object.key("FieldMappings").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.change_detecting_columns {
        let mut array_9 = object.key("ChangeDetectingColumns").start_array();
        for item_10 in var_8 {
             {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    Ok(())
}

pub(crate) fn de_column_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ColumnConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::column_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DocumentIdColumnName" => {
                                builder = builder.set_document_id_column_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DocumentDataColumnName" => {
                                builder = builder.set_document_data_column_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DocumentTitleColumnName" => {
                                builder = builder.set_document_title_column_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "FieldMappings" => {
                                builder = builder.set_field_mappings(
                                    crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(tokens)?
                                );
                            }
                            "ChangeDetectingColumns" => {
                                builder = builder.set_change_detecting_columns(
                                    crate::protocol_serde::shape_change_detecting_columns::de_change_detecting_columns(tokens)?
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

