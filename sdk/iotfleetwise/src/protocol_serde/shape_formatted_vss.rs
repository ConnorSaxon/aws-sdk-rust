// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_formatted_vss(object_4: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FormattedVss) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::FormattedVss::VssJson(inner) => {
             {
                object_4.key("vssJson").string(inner.as_str());
            }
        },
        crate::model::FormattedVss::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("FormattedVss"))
    }
    Ok(())
}

