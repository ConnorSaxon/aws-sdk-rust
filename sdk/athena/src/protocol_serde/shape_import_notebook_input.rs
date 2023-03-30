// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_notebook_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ImportNotebookInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.work_group {
        object.key("WorkGroup").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.payload {
        object.key("Payload").string(var_3.as_str());
    }
    if let Some(var_4) = &input.r#type {
        object.key("Type").string(var_4.as_str());
    }
    if let Some(var_5) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_5.as_str());
    }
    Ok(())
}

