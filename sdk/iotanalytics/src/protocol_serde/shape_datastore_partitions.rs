// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_datastore_partitions(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DatastorePartitions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.partitions {
        let mut array_2 = object.key("partitions").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_datastore_partition::ser_datastore_partition(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub(crate) fn de_datastore_partitions<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DatastorePartitions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::datastore_partitions::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "partitions" => {
                                builder = builder.set_partitions(
                                    crate::protocol_serde::shape_partitions::de_partitions(tokens)?
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

