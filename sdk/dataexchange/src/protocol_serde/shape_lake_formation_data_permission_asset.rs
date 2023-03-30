// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_lake_formation_data_permission_asset<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::LakeFormationDataPermissionAsset>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::lake_formation_data_permission_asset::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "LakeFormationDataPermissionDetails" => {
                                builder = builder.set_lake_formation_data_permission_details(
                                    crate::protocol_serde::shape_lake_formation_data_permission_details::de_lake_formation_data_permission_details(tokens)?
                                );
                            }
                            "LakeFormationDataPermissionType" => {
                                builder = builder.set_lake_formation_data_permission_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::LakeFormationDataPermissionType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Permissions" => {
                                builder = builder.set_permissions(
                                    crate::protocol_serde::shape_list_of_lf_permissions::de_list_of_lf_permissions(tokens)?
                                );
                            }
                            "RoleArn" => {
                                builder = builder.set_role_arn(
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

