// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_custom_line_item_flat_charge_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UpdateCustomLineItemFlatChargeDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.charge_value {
        object.key("ChargeValue").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_1).into()));
    }
    Ok(())
}

