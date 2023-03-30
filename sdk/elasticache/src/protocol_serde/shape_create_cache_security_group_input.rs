// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_cache_security_group_input_input(input: &crate::input::CreateCacheSecurityGroupInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateCacheSecurityGroup", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CacheSecurityGroupName");
    if let Some(var_2) = &input.cache_security_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Description");
    if let Some(var_4) = &input.description {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Tags");
    if let Some(var_6) = &input.tags {
        let mut list_8 = scope_5.start_list(false, Some("Tag"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_9, item_7)?;
        }
        list_8.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

