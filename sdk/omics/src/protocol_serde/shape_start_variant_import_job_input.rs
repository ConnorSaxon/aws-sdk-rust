// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_variant_import_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartVariantImportJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.destination_name {
        object.key("destinationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.items {
        let mut array_3 = object.key("items").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_variant_import_item_source::ser_variant_import_item_source(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.role_arn {
        object.key("roleArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.run_left_normalization {
        object.key("runLeftNormalization").boolean(*var_7);
    }
    Ok(())
}

