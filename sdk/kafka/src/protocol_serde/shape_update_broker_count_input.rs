// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_broker_count_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateBrokerCountInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.current_version {
        object.key("currentVersion").string(var_1.as_str());
    }
     {
        object.key("targetNumberOfBrokerNodes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.target_number_of_broker_nodes).into()));
    }
    Ok(())
}

