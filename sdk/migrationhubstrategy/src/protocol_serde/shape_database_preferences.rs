// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_database_preferences<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DatabasePreferences>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::database_preferences::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "databaseManagementPreference" => {
                                builder = builder.set_database_management_preference(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::DatabaseManagementPreference::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "databaseMigrationPreference" => {
                                builder = builder.set_database_migration_preference(
                                    crate::protocol_serde::shape_database_migration_preference::de_database_migration_preference(tokens)?
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

pub fn ser_database_preferences(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DatabasePreferences) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.database_management_preference {
        object.key("databaseManagementPreference").string(var_1.as_str());
    }
    if let Some(var_2) = &input.database_migration_preference {
        #[allow(unused_mut)]
        let mut object_3 = object.key("databaseMigrationPreference").start_object();
        crate::protocol_serde::shape_database_migration_preference::ser_database_migration_preference(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

