// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_findings_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListFindingsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.assessment_run_arns {
        let mut array_2 = object.key("assessmentRunArns").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.filter {
        #[allow(unused_mut)]
        let mut object_5 = object.key("filter").start_object();
        crate::protocol_serde::shape_finding_filter::ser_finding_filter(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.next_token {
        object.key("nextToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    Ok(())
}

