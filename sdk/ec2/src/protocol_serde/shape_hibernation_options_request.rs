// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_hibernation_options_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::HibernationOptionsRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Configured");
    if let Some(var_2) = &input.configured {
        scope_1.boolean(*var_2);
    }
    Ok(())
}

