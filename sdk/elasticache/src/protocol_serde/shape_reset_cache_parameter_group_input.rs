// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_reset_cache_parameter_group_input_input(input: &crate::input::ResetCacheParameterGroupInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ResetCacheParameterGroup", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CacheParameterGroupName");
    if let Some(var_2) = &input.cache_parameter_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ResetAllParameters");
    if input.reset_all_parameters {
        scope_3.boolean(input.reset_all_parameters);
    }
    #[allow(unused_mut)]
    let mut scope_4 = writer.prefix("ParameterNameValues");
    if let Some(var_5) = &input.parameter_name_values {
        let mut list_7 = scope_4.start_list(false, Some("ParameterNameValue"));
        for item_6 in var_5 {
            #[allow(unused_mut)]
            let mut entry_8 = list_7.entry();
            crate::protocol_serde::shape_parameter_name_value::ser_parameter_name_value(entry_8, item_6)?;
        }
        list_7.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

