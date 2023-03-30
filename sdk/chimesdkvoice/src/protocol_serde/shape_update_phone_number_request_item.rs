// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_phone_number_request_item(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UpdatePhoneNumberRequestItem) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.phone_number_id {
        object.key("PhoneNumberId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.product_type {
        object.key("ProductType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.calling_name {
        object.key("CallingName").string(var_3.as_str());
    }
    Ok(())
}

