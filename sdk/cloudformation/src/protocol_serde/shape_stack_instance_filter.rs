// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_stack_instance_filter(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::StackInstanceFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Name");
    if let Some(var_2) = &input.name {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Values");
    if let Some(var_4) = &input.values {
        scope_3.string(var_4);
    }
    Ok(())
}

