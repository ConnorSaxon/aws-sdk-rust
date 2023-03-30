// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_iam_role_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsIamRoleDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.assume_role_policy_document {
        object.key("AssumeRolePolicyDocument").string(var_1.as_str());
    }
    if let Some(var_2) = &input.attached_managed_policies {
        let mut array_3 = object.key("AttachedManagedPolicies").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_aws_iam_attached_managed_policy::ser_aws_iam_attached_managed_policy(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.create_date {
        object.key("CreateDate").string(var_6.as_str());
    }
    if let Some(var_7) = &input.instance_profile_list {
        let mut array_8 = object.key("InstanceProfileList").start_array();
        for item_9 in var_7 {
             {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_aws_iam_instance_profile::ser_aws_iam_instance_profile(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.permissions_boundary {
        #[allow(unused_mut)]
        let mut object_12 = object.key("PermissionsBoundary").start_object();
        crate::protocol_serde::shape_aws_iam_permissions_boundary::ser_aws_iam_permissions_boundary(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.role_id {
        object.key("RoleId").string(var_13.as_str());
    }
    if let Some(var_14) = &input.role_name {
        object.key("RoleName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.role_policy_list {
        let mut array_16 = object.key("RolePolicyList").start_array();
        for item_17 in var_15 {
             {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::protocol_serde::shape_aws_iam_role_policy::ser_aws_iam_role_policy(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if input.max_session_duration != 0 {
        object.key("MaxSessionDuration").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_session_duration).into()));
    }
    if let Some(var_19) = &input.path {
        object.key("Path").string(var_19.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_iam_role_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsIamRoleDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_iam_role_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AssumeRolePolicyDocument" => {
                                builder = builder.set_assume_role_policy_document(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "AttachedManagedPolicies" => {
                                builder = builder.set_attached_managed_policies(
                                    crate::protocol_serde::shape_aws_iam_attached_managed_policy_list::de_aws_iam_attached_managed_policy_list(tokens)?
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
                            "InstanceProfileList" => {
                                builder = builder.set_instance_profile_list(
                                    crate::protocol_serde::shape_aws_iam_instance_profile_list::de_aws_iam_instance_profile_list(tokens)?
                                );
                            }
                            "PermissionsBoundary" => {
                                builder = builder.set_permissions_boundary(
                                    crate::protocol_serde::shape_aws_iam_permissions_boundary::de_aws_iam_permissions_boundary(tokens)?
                                );
                            }
                            "RoleId" => {
                                builder = builder.set_role_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RoleName" => {
                                builder = builder.set_role_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RolePolicyList" => {
                                builder = builder.set_role_policy_list(
                                    crate::protocol_serde::shape_aws_iam_role_policy_list::de_aws_iam_role_policy_list(tokens)?
                                );
                            }
                            "MaxSessionDuration" => {
                                builder = builder.set_max_session_duration(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
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

