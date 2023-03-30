// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_search_content_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SearchContentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.search_expression {
        #[allow(unused_mut)]
        let mut object_2 = object.key("searchExpression").start_object();
        crate::protocol_serde::shape_search_expression::ser_search_expression(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

