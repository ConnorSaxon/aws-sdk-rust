// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_classification_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateClassificationJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.job_status {
        object.key("jobStatus").string(var_1.as_str());
    }
    Ok(())
}

