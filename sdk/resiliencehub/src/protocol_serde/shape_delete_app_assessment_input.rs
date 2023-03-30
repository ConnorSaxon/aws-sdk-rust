// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_app_assessment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteAppAssessmentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.assessment_arn {
        object.key("assessmentArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    Ok(())
}

