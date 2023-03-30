// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_source_servers_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSourceServersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.filters {
        #[allow(unused_mut)]
        let mut object_2 = object.key("filters").start_object();
        crate::protocol_serde::shape_describe_source_servers_request_filters::ser_describe_source_servers_request_filters(&mut object_2, var_1)?;
        object_2.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_3) = &input.next_token {
        object.key("nextToken").string(var_3.as_str());
    }
    Ok(())
}

