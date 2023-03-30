// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vpc_security_group_membership(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::VpcSecurityGroupMembership, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::VpcSecurityGroupMembership::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("VpcSecurityGroupId") /* VpcSecurityGroupId com.amazonaws.redshift#VpcSecurityGroupMembership$VpcSecurityGroupId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_security_group_id(var_1);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.redshift#VpcSecurityGroupMembership$Status */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

