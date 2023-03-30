// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_budget_action_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateBudgetActionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("AccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.budget_name {
        object.key("BudgetName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.action_id {
        object.key("ActionId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.notification_type {
        object.key("NotificationType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.action_threshold {
        #[allow(unused_mut)]
        let mut object_6 = object.key("ActionThreshold").start_object();
        crate::protocol_serde::shape_action_threshold::ser_action_threshold(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.definition {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Definition").start_object();
        crate::protocol_serde::shape_definition::ser_definition(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_9.as_str());
    }
    if let Some(var_10) = &input.approval_model {
        object.key("ApprovalModel").string(var_10.as_str());
    }
    if let Some(var_11) = &input.subscribers {
        let mut array_12 = object.key("Subscribers").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_subscriber::ser_subscriber(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    Ok(())
}

