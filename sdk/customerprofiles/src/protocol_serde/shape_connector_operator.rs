// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_connector_operator(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConnectorOperator) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.marketo {
        object.key("Marketo").string(var_1.as_str());
    }
    if let Some(var_2) = &input.s3 {
        object.key("S3").string(var_2.as_str());
    }
    if let Some(var_3) = &input.salesforce {
        object.key("Salesforce").string(var_3.as_str());
    }
    if let Some(var_4) = &input.service_now {
        object.key("ServiceNow").string(var_4.as_str());
    }
    if let Some(var_5) = &input.zendesk {
        object.key("Zendesk").string(var_5.as_str());
    }
    Ok(())
}

