// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_copy_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CopyAction) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.lifecycle {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Lifecycle").start_object();
        crate::protocol_serde::shape_lifecycle::ser_lifecycle(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.destination_backup_vault_arn {
        object.key("DestinationBackupVaultArn").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_copy_action<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CopyAction>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::copy_action::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Lifecycle" => {
                                builder = builder.set_lifecycle(
                                    crate::protocol_serde::shape_lifecycle::de_lifecycle(tokens)?
                                );
                            }
                            "DestinationBackupVaultArn" => {
                                builder = builder.set_destination_backup_vault_arn(
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

