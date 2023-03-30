// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_organization_health_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeOrganizationHealthInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_ids {
        let mut array_2 = object.key("AccountIds").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.organizational_unit_ids {
        let mut array_5 = object.key("OrganizationalUnitIds").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    Ok(())
}

