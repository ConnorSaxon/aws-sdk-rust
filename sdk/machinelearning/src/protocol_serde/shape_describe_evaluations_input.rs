// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_evaluations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeEvaluationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.filter_variable {
        object.key("FilterVariable").string(var_1.as_str());
    }
    if let Some(var_2) = &input.eq {
        object.key("EQ").string(var_2.as_str());
    }
    if let Some(var_3) = &input.gt {
        object.key("GT").string(var_3.as_str());
    }
    if let Some(var_4) = &input.lt {
        object.key("LT").string(var_4.as_str());
    }
    if let Some(var_5) = &input.ge {
        object.key("GE").string(var_5.as_str());
    }
    if let Some(var_6) = &input.le {
        object.key("LE").string(var_6.as_str());
    }
    if let Some(var_7) = &input.ne {
        object.key("NE").string(var_7.as_str());
    }
    if let Some(var_8) = &input.prefix {
        object.key("Prefix").string(var_8.as_str());
    }
    if let Some(var_9) = &input.sort_order {
        object.key("SortOrder").string(var_9.as_str());
    }
    if let Some(var_10) = &input.next_token {
        object.key("NextToken").string(var_10.as_str());
    }
    if let Some(var_11) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_11).into()));
    }
    Ok(())
}

