// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_assessment_target_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AssessmentTargetFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.assessment_target_name_pattern {
        object.key("assessmentTargetNamePattern").string(var_1.as_str());
    }
    Ok(())
}

