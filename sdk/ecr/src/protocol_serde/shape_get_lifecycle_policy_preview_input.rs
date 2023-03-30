// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_lifecycle_policy_preview_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetLifecyclePolicyPreviewInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.registry_id {
        object.key("registryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.repository_name {
        object.key("repositoryName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.image_ids {
        let mut array_4 = object.key("imageIds").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_image_identifier::ser_image_identifier(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.next_token {
        object.key("nextToken").string(var_7.as_str());
    }
    if let Some(var_8) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    if let Some(var_9) = &input.filter {
        #[allow(unused_mut)]
        let mut object_10 = object.key("filter").start_object();
        crate::protocol_serde::shape_lifecycle_policy_preview_filter::ser_lifecycle_policy_preview_filter(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}

