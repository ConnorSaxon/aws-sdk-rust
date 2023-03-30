// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_account_assignment_deletion_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListAccountAssignmentDeletionStatusInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.instance_arn {
        object.key("InstanceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.filter {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Filter").start_object();
        crate::protocol_serde::shape_operation_status_filter::ser_operation_status_filter(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

