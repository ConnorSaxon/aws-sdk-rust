// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_subscriber_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSubscriberInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("AccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.budget_name {
        object.key("BudgetName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.notification {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Notification").start_object();
        crate::protocol_serde::shape_notification::ser_notification(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.subscriber {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Subscriber").start_object();
        crate::protocol_serde::shape_subscriber::ser_subscriber(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

