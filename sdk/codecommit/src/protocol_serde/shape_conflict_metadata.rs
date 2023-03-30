// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_conflict_metadata<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ConflictMetadata>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::conflict_metadata::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "filePath" => {
                                builder = builder.set_file_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "fileSizes" => {
                                builder = builder.set_file_sizes(
                                    crate::protocol_serde::shape_file_sizes::de_file_sizes(tokens)?
                                );
                            }
                            "fileModes" => {
                                builder = builder.set_file_modes(
                                    crate::protocol_serde::shape_file_modes::de_file_modes(tokens)?
                                );
                            }
                            "objectTypes" => {
                                builder = builder.set_object_types(
                                    crate::protocol_serde::shape_object_types::de_object_types(tokens)?
                                );
                            }
                            "numberOfConflicts" => {
                                builder = builder.set_number_of_conflicts(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "isBinaryFile" => {
                                builder = builder.set_is_binary_file(
                                    crate::protocol_serde::shape_is_binary_file::de_is_binary_file(tokens)?
                                );
                            }
                            "contentConflict" => {
                                builder = builder.set_content_conflict(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "fileModeConflict" => {
                                builder = builder.set_file_mode_conflict(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "objectTypeConflict" => {
                                builder = builder.set_object_type_conflict(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "mergeOperations" => {
                                builder = builder.set_merge_operations(
                                    crate::protocol_serde::shape_merge_operations::de_merge_operations(tokens)?
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

