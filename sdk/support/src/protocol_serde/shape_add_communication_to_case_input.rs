// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_communication_to_case_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddCommunicationToCaseInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.case_id {
        object.key("caseId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.communication_body {
        object.key("communicationBody").string(var_2.as_str());
    }
    if let Some(var_3) = &input.cc_email_addresses {
        let mut array_4 = object.key("ccEmailAddresses").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.attachment_set_id {
        object.key("attachmentSetId").string(var_6.as_str());
    }
    Ok(())
}

