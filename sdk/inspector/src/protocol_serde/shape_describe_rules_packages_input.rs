// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_rules_packages_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeRulesPackagesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.rules_package_arns {
        let mut array_2 = object.key("rulesPackageArns").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.locale {
        object.key("locale").string(var_4.as_str());
    }
    Ok(())
}

