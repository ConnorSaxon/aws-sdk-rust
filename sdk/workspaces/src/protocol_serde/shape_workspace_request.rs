// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_workspace_request(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::WorkspaceRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.directory_id {
        object.key("DirectoryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.user_name {
        object.key("UserName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.bundle_id {
        object.key("BundleId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.volume_encryption_key {
        object.key("VolumeEncryptionKey").string(var_4.as_str());
    }
    if let Some(var_5) = &input.user_volume_encryption_enabled {
        object.key("UserVolumeEncryptionEnabled").boolean(*var_5);
    }
    if let Some(var_6) = &input.root_volume_encryption_enabled {
        object.key("RootVolumeEncryptionEnabled").boolean(*var_6);
    }
    if let Some(var_7) = &input.workspace_properties {
        #[allow(unused_mut)]
        let mut object_8 = object.key("WorkspaceProperties").start_object();
        crate::protocol_serde::shape_workspace_properties::ser_workspace_properties(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.tags {
        let mut array_10 = object.key("Tags").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub(crate) fn de_workspace_request<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::WorkspaceRequest>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::workspace_request::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DirectoryId" => {
                                builder = builder.set_directory_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "UserName" => {
                                builder = builder.set_user_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "BundleId" => {
                                builder = builder.set_bundle_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "VolumeEncryptionKey" => {
                                builder = builder.set_volume_encryption_key(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "UserVolumeEncryptionEnabled" => {
                                builder = builder.set_user_volume_encryption_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "RootVolumeEncryptionEnabled" => {
                                builder = builder.set_root_volume_encryption_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "WorkspaceProperties" => {
                                builder = builder.set_workspace_properties(
                                    crate::protocol_serde::shape_workspace_properties::de_workspace_properties(tokens)?
                                );
                            }
                            "Tags" => {
                                builder = builder.set_tags(
                                    crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?
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

