// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_mapping_parameters<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::MappingParameters>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::mapping_parameters::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "JSONMappingParameters" => {
                                builder = builder.set_json_mapping_parameters(
                                    crate::protocol_serde::shape_json_mapping_parameters::de_json_mapping_parameters(tokens)?
                                );
                            }
                            "CSVMappingParameters" => {
                                builder = builder.set_csv_mapping_parameters(
                                    crate::protocol_serde::shape_csv_mapping_parameters::de_csv_mapping_parameters(tokens)?
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

pub fn ser_mapping_parameters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MappingParameters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.json_mapping_parameters {
        #[allow(unused_mut)]
        let mut object_2 = object.key("JSONMappingParameters").start_object();
        crate::protocol_serde::shape_json_mapping_parameters::ser_json_mapping_parameters(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.csv_mapping_parameters {
        #[allow(unused_mut)]
        let mut object_4 = object.key("CSVMappingParameters").start_object();
        crate::protocol_serde::shape_csv_mapping_parameters::ser_csv_mapping_parameters(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

