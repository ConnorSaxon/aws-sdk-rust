// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_phase2_encryption_algorithms_request_list_value(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Phase2EncryptionAlgorithmsRequestListValue) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Value");
    if let Some(var_2) = &input.value {
        scope_1.string(var_2);
    }
    Ok(())
}

