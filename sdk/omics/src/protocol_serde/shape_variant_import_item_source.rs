// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_variant_import_item_source(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VariantImportItemSource) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.source {
        object.key("source").string(var_1.as_str());
    }
    Ok(())
}

