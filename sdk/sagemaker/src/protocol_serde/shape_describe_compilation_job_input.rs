// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_compilation_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeCompilationJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.compilation_job_name {
        object.key("CompilationJobName").string(var_1.as_str());
    }
    Ok(())
}

