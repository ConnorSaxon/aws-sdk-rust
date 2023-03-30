// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_interconnect_loa_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeInterconnectLoaInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.interconnect_id {
        object.key("interconnectId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.provider_name {
        object.key("providerName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.loa_content_type {
        object.key("loaContentType").string(var_3.as_str());
    }
    Ok(())
}

