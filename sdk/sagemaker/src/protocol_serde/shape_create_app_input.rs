// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_app_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAppInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_id {
        object.key("DomainId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.user_profile_name {
        object.key("UserProfileName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.app_type {
        object.key("AppType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.app_name {
        object.key("AppName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.resource_spec {
        #[allow(unused_mut)]
        let mut object_10 = object.key("ResourceSpec").start_object();
        crate::protocol_serde::shape_resource_spec::ser_resource_spec(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.space_name {
        object.key("SpaceName").string(var_11.as_str());
    }
    Ok(())
}

