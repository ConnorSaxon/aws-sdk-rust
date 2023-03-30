// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_destination_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDestinationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.additional_fixed_properties {
        object.key("additionalFixedProperties").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.site {
        object.key("site").string(var_4.as_str());
    }
    if let Some(var_5) = &input.state {
        object.key("state").string(var_5.as_str());
    }
    Ok(())
}

