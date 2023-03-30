// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_association_execution_targets_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeAssociationExecutionTargetsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.association_id {
        object.key("AssociationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.execution_id {
        object.key("ExecutionId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.filters {
        let mut array_4 = object.key("Filters").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_association_execution_targets_filter::ser_association_execution_targets_filter(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    if let Some(var_8) = &input.next_token {
        object.key("NextToken").string(var_8.as_str());
    }
    Ok(())
}

