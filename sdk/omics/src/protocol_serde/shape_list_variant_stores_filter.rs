// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_variant_stores_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ListVariantStoresFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.status {
        object.key("status").string(var_1.as_str());
    }
    Ok(())
}

