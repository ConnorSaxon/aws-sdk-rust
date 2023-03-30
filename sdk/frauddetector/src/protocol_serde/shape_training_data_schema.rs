// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_training_data_schema(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TrainingDataSchema) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.model_variables {
        let mut array_2 = object.key("modelVariables").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.label_schema {
        #[allow(unused_mut)]
        let mut object_5 = object.key("labelSchema").start_object();
        crate::protocol_serde::shape_label_schema::ser_label_schema(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_training_data_schema<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TrainingDataSchema>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::training_data_schema::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "modelVariables" => {
                                builder = builder.set_model_variables(
                                    crate::protocol_serde::shape_list_of_strings::de_list_of_strings(tokens)?
                                );
                            }
                            "labelSchema" => {
                                builder = builder.set_label_schema(
                                    crate::protocol_serde::shape_label_schema::de_label_schema(tokens)?
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

