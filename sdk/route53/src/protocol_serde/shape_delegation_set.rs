// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_delegation_set(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DelegationSet, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DelegationSet::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.route53#DelegationSet$Id */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_1);
            }
            ,
            s if s.matches("CallerReference") /* CallerReference com.amazonaws.route53#DelegationSet$CallerReference */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_caller_reference(var_2);
            }
            ,
            s if s.matches("NameServers") /* NameServers com.amazonaws.route53#DelegationSet$NameServers */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_delegation_set_name_servers::de_delegation_set_name_servers(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_name_servers(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

