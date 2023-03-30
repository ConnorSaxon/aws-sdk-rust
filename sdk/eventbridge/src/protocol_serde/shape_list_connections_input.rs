// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_connections_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListConnectionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name_prefix {
        object.key("NamePrefix").string(var_1.as_str());
    }
    if let Some(var_2) = &input.connection_state {
        object.key("ConnectionState").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    Ok(())
}

