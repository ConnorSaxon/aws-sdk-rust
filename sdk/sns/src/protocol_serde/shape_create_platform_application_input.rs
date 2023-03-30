// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_platform_application_input_input(input: &crate::input::CreatePlatformApplicationInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreatePlatformApplication", "2010-03-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Name");
    if let Some(var_2) = &input.name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Platform");
    if let Some(var_4) = &input.platform {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Attributes");
    if let Some(var_6) = &input.attributes {
        let mut map_7 = scope_5.start_map(false, "key", "value");
        for (key_8, value_9) in var_6 {
            #[allow(unused_mut)]
            let mut entry_10 = map_7.entry(key_8);
             {
                entry_10.string(value_9);
            }
        }
        map_7.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

