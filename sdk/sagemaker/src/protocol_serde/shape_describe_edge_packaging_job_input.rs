// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_edge_packaging_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeEdgePackagingJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.edge_packaging_job_name {
        object.key("EdgePackagingJobName").string(var_1.as_str());
    }
    Ok(())
}

