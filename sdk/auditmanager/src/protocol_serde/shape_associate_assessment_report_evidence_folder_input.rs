// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_assessment_report_evidence_folder_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateAssessmentReportEvidenceFolderInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.evidence_folder_id {
        object.key("evidenceFolderId").string(var_1.as_str());
    }
    Ok(())
}

