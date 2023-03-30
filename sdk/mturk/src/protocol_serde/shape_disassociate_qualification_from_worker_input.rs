// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_qualification_from_worker_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociateQualificationFromWorkerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.worker_id {
        object.key("WorkerId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.qualification_type_id {
        object.key("QualificationTypeId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.reason {
        object.key("Reason").string(var_3.as_str());
    }
    Ok(())
}

