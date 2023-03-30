// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_connector_definition_version(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConnectorDefinitionVersion) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.connectors {
        let mut array_2 = object.key("Connectors").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_connector::ser_connector(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub(crate) fn de_connector_definition_version<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ConnectorDefinitionVersion>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::connector_definition_version::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Connectors" => {
                                builder = builder.set_connectors(
                                    crate::protocol_serde::shape___list_of_connector::de___list_of_connector(tokens)?
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

