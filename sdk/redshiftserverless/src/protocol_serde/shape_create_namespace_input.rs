// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateNamespaceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.namespace_name {
        object.key("namespaceName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.admin_username {
        object.key("adminUsername").string(var_2.as_str());
    }
    if let Some(var_3) = &input.admin_user_password {
        object.key("adminUserPassword").string(var_3.as_str());
    }
    if let Some(var_4) = &input.db_name {
        object.key("dbName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.default_iam_role_arn {
        object.key("defaultIamRoleArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.iam_roles {
        let mut array_8 = object.key("iamRoles").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.log_exports {
        let mut array_11 = object.key("logExports").start_array();
        for item_12 in var_10 {
             {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.tags {
        let mut array_14 = object.key("tags").start_array();
        for item_15 in var_13 {
             {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    Ok(())
}

