// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_import_jobs_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListImportJobsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.import_destination_type {
        object.key("ImportDestinationType").string(var_1.as_str());
    }
    Ok(())
}

