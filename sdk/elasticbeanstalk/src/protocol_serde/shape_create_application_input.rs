// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_application_input_input(input: &crate::input::CreateApplicationInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateApplication", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ApplicationName");
    if let Some(var_2) = &input.application_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Description");
    if let Some(var_4) = &input.description {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ResourceLifecycleConfig");
    if let Some(var_6) = &input.resource_lifecycle_config {
        crate::protocol_serde::shape_application_resource_lifecycle_config::ser_application_resource_lifecycle_config(scope_5, var_6)?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Tags");
    if let Some(var_8) = &input.tags {
        let mut list_10 = scope_7.start_list(false, None);
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_11, item_9)?;
        }
        list_10.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

