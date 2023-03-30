// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_custom_line_item_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateCustomLineItemInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.billing_group_arn {
        object.key("BillingGroupArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.billing_period_range {
        #[allow(unused_mut)]
        let mut object_3 = object.key("BillingPeriodRange").start_object();
        crate::protocol_serde::shape_custom_line_item_billing_period_range::ser_custom_line_item_billing_period_range(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.charge_details {
        #[allow(unused_mut)]
        let mut object_5 = object.key("ChargeDetails").start_object();
        crate::protocol_serde::shape_custom_line_item_charge_details::ser_custom_line_item_charge_details(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.name {
        object.key("Name").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        #[allow(unused_mut)]
        let mut object_9 = object.key("Tags").start_object();
        for (key_10, value_11) in var_8 {
             {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}

