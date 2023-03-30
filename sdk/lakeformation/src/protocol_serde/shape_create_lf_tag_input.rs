// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_lf_tag_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLfTagInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.catalog_id {
        object.key("CatalogId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tag_key {
        object.key("TagKey").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tag_values {
        let mut array_4 = object.key("TagValues").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}

