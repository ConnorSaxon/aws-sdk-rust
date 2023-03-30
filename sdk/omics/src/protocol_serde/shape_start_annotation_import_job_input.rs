// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_annotation_import_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartAnnotationImportJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.destination_name {
        object.key("destinationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.format_options {
        #[allow(unused_mut)]
        let mut object_3 = object.key("formatOptions").start_object();
        crate::protocol_serde::shape_format_options::ser_format_options(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.items {
        let mut array_5 = object.key("items").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_annotation_import_item_source::ser_annotation_import_item_source(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.role_arn {
        object.key("roleArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.run_left_normalization {
        object.key("runLeftNormalization").boolean(*var_9);
    }
    Ok(())
}

