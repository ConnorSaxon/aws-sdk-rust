// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_parallel_data_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetParallelDataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    Ok(())
}

