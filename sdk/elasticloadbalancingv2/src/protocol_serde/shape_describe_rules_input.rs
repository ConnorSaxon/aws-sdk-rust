// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_rules_input_input(input: &crate::input::DescribeRulesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeRules", "2015-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ListenerArn");
    if let Some(var_2) = &input.listener_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("RuleArns");
    if let Some(var_4) = &input.rule_arns {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("Marker");
    if let Some(var_9) = &input.marker {
        scope_8.string(var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("PageSize");
    if let Some(var_11) = &input.page_size {
        scope_10.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_11).into()));
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

