// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Input) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.s3_input_definition {
        #[allow(unused_mut)]
        let mut object_2 = object.key("S3InputDefinition").start_object();
        crate::protocol_serde::shape_s3_location::ser_s3_location(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.data_catalog_input_definition {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DataCatalogInputDefinition").start_object();
        crate::protocol_serde::shape_data_catalog_input_definition::ser_data_catalog_input_definition(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.database_input_definition {
        #[allow(unused_mut)]
        let mut object_6 = object.key("DatabaseInputDefinition").start_object();
        crate::protocol_serde::shape_database_input_definition::ser_database_input_definition(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.metadata {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Metadata").start_object();
        crate::protocol_serde::shape_metadata::ser_metadata(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}

pub(crate) fn de_input<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Input>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::input::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "S3InputDefinition" => {
                                builder = builder.set_s3_input_definition(
                                    crate::protocol_serde::shape_s3_location::de_s3_location(tokens)?
                                );
                            }
                            "DataCatalogInputDefinition" => {
                                builder = builder.set_data_catalog_input_definition(
                                    crate::protocol_serde::shape_data_catalog_input_definition::de_data_catalog_input_definition(tokens)?
                                );
                            }
                            "DatabaseInputDefinition" => {
                                builder = builder.set_database_input_definition(
                                    crate::protocol_serde::shape_database_input_definition::de_database_input_definition(tokens)?
                                );
                            }
                            "Metadata" => {
                                builder = builder.set_metadata(
                                    crate::protocol_serde::shape_metadata::de_metadata(tokens)?
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

