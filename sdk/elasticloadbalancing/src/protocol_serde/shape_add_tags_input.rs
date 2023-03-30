// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_tags_input_input(input: &crate::input::AddTagsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AddTags", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LoadBalancerNames");
    if let Some(var_2) = &input.load_balancer_names {
        let mut list_4 = scope_1.start_list(false, None);
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("Tags");
    if let Some(var_7) = &input.tags {
        let mut list_9 = scope_6.start_list(false, None);
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_10, item_8)?;
        }
        list_9.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

