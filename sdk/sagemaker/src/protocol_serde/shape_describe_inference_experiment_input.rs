// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_inference_experiment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeInferenceExperimentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    Ok(())
}

