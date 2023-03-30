// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rds_db_snapshot_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RdsDbSnapshotConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_2 = object.key("attributes").start_object();
        for (key_3, value_4) in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_5 = object_2.key(key_3.as_str()).start_object();
                crate::protocol_serde::shape_rds_db_snapshot_attribute_value::ser_rds_db_snapshot_attribute_value(&mut object_5, value_4)?;
                object_5.finish();
            }
        }
        object_2.finish();
    }
    if let Some(var_6) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_rds_db_snapshot_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RdsDbSnapshotConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::rds_db_snapshot_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "attributes" => {
                                builder = builder.set_attributes(
                                    crate::protocol_serde::shape_rds_db_snapshot_attributes_map::de_rds_db_snapshot_attributes_map(tokens)?
                                );
                            }
                            "kmsKeyId" => {
                                builder = builder.set_kms_key_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
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

