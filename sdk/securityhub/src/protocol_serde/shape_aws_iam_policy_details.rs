// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_iam_policy_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsIamPolicyDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.attachment_count != 0 {
        object.key("AttachmentCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.attachment_count).into()));
    }
    if let Some(var_1) = &input.create_date {
        object.key("CreateDate").string(var_1.as_str());
    }
    if let Some(var_2) = &input.default_version_id {
        object.key("DefaultVersionId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if input.is_attachable {
        object.key("IsAttachable").boolean(input.is_attachable);
    }
    if let Some(var_4) = &input.path {
        object.key("Path").string(var_4.as_str());
    }
    if input.permissions_boundary_usage_count != 0 {
        object.key("PermissionsBoundaryUsageCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.permissions_boundary_usage_count).into()));
    }
    if let Some(var_5) = &input.policy_id {
        object.key("PolicyId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.policy_name {
        object.key("PolicyName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.policy_version_list {
        let mut array_8 = object.key("PolicyVersionList").start_array();
        for item_9 in var_7 {
             {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_aws_iam_policy_version::ser_aws_iam_policy_version(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.update_date {
        object.key("UpdateDate").string(var_11.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_iam_policy_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsIamPolicyDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_iam_policy_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AttachmentCount" => {
                                builder = builder.set_attachment_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CreateDate" => {
                                builder = builder.set_create_date(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DefaultVersionId" => {
                                builder = builder.set_default_version_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "IsAttachable" => {
                                builder = builder.set_is_attachable(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "Path" => {
                                builder = builder.set_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "PermissionsBoundaryUsageCount" => {
                                builder = builder.set_permissions_boundary_usage_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "PolicyId" => {
                                builder = builder.set_policy_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "PolicyName" => {
                                builder = builder.set_policy_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "PolicyVersionList" => {
                                builder = builder.set_policy_version_list(
                                    crate::protocol_serde::shape_aws_iam_policy_version_list::de_aws_iam_policy_version_list(tokens)?
                                );
                            }
                            "UpdateDate" => {
                                builder = builder.set_update_date(
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

