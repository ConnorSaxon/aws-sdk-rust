// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_projects_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListProjectsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.filters {
        let mut array_2 = object.key("filters").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_project_list_filter::ser_project_list_filter(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    if let Some(var_6) = &input.next_token {
        object.key("nextToken").string(var_6.as_str());
    }
    Ok(())
}

