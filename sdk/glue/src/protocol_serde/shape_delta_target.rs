// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delta_target(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DeltaTarget) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.delta_tables {
        let mut array_2 = object.key("DeltaTables").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.connection_name {
        object.key("ConnectionName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.write_manifest {
        object.key("WriteManifest").boolean(*var_5);
    }
    if let Some(var_6) = &input.create_native_delta_table {
        object.key("CreateNativeDeltaTable").boolean(*var_6);
    }
    Ok(())
}

pub(crate) fn de_delta_target<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DeltaTarget>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::delta_target::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DeltaTables" => {
                                builder = builder.set_delta_tables(
                                    crate::protocol_serde::shape_path_list::de_path_list(tokens)?
                                );
                            }
                            "ConnectionName" => {
                                builder = builder.set_connection_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "WriteManifest" => {
                                builder = builder.set_write_manifest(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CreateNativeDeltaTable" => {
                                builder = builder.set_create_native_delta_table(
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

