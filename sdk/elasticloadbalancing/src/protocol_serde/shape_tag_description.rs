// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_tag_description(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::TagDescription, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::TagDescription::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LoadBalancerName") /* LoadBalancerName com.amazonaws.elasticloadbalancing#TagDescription$LoadBalancerName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_load_balancer_name(var_1);
            }
            ,
            s if s.matches("Tags") /* Tags com.amazonaws.elasticloadbalancing#TagDescription$Tags */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

