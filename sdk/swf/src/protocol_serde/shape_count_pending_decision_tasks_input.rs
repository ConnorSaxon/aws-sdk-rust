// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_count_pending_decision_tasks_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CountPendingDecisionTasksInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain {
        object.key("domain").string(var_1.as_str());
    }
    if let Some(var_2) = &input.task_list {
        #[allow(unused_mut)]
        let mut object_3 = object.key("taskList").start_object();
        crate::protocol_serde::shape_task_list::ser_task_list(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

