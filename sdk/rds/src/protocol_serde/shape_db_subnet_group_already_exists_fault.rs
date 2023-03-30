// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn de_db_subnet_group_already_exists_fault_xml_err(inp: &[u8], mut builder: crate::error::db_subnet_group_already_exists_fault::Builder) -> Result<crate::error::db_subnet_group_already_exists_fault::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    if inp.is_empty() { return Ok(builder) }
    let mut document = aws_smithy_xml::decode::Document::try_from(inp)?;
                            #[allow(unused_mut)]
                            let mut error_decoder = crate::rest_xml_wrapped_errors::error_scope(&mut document)?;
    while let Some(mut tag) = error_decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("message") /* message com.amazonaws.rds#DBSubnetGroupAlreadyExistsFault$message */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_message(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

