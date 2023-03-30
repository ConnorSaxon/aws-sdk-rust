// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tag_resource_input_input(input: &crate::input::TagResourceInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "TagResource", "2010-08-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ResourceARN");
    if let Some(var_2) = &input.resource_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Tags");
    if let Some(var_4) = &input.tags {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_7, item_5)?;
        }
        list_6.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

