// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_catalog_output(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DataCatalogOutput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.catalog_id {
        object.key("CatalogId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.database_name {
        object.key("DatabaseName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.table_name {
        object.key("TableName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.s3_options {
        #[allow(unused_mut)]
        let mut object_5 = object.key("S3Options").start_object();
        crate::protocol_serde::shape_s3_table_output_options::ser_s3_table_output_options(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.database_options {
        #[allow(unused_mut)]
        let mut object_7 = object.key("DatabaseOptions").start_object();
        crate::protocol_serde::shape_database_table_output_options::ser_database_table_output_options(&mut object_7, var_6)?;
        object_7.finish();
    }
    if input.overwrite {
        object.key("Overwrite").boolean(input.overwrite);
    }
    Ok(())
}

pub(crate) fn de_data_catalog_output<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DataCatalogOutput>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::data_catalog_output::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CatalogId" => {
                                builder = builder.set_catalog_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DatabaseName" => {
                                builder = builder.set_database_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TableName" => {
                                builder = builder.set_table_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "S3Options" => {
                                builder = builder.set_s3_options(
                                    crate::protocol_serde::shape_s3_table_output_options::de_s3_table_output_options(tokens)?
                                );
                            }
                            "DatabaseOptions" => {
                                builder = builder.set_database_options(
                                    crate::protocol_serde::shape_database_table_output_options::de_database_table_output_options(tokens)?
                                );
                            }
                            "Overwrite" => {
                                builder = builder.set_overwrite(
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

