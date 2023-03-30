// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_updated_image_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateUpdatedImageInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.existing_image_name {
        object.key("existingImageName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.new_image_name {
        object.key("newImageName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.new_image_description {
        object.key("newImageDescription").string(var_3.as_str());
    }
    if let Some(var_4) = &input.new_image_display_name {
        object.key("newImageDisplayName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.new_image_tags {
        #[allow(unused_mut)]
        let mut object_6 = object.key("newImageTags").start_object();
        for (key_7, value_8) in var_5 {
             {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    if input.dry_run {
        object.key("dryRun").boolean(input.dry_run);
    }
    Ok(())
}

