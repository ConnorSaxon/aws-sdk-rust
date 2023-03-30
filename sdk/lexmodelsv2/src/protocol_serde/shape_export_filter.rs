// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_export_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ExportFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.values {
        let mut array_3 = object.key("values").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.operator {
        object.key("operator").string(var_5.as_str());
    }
    Ok(())
}

