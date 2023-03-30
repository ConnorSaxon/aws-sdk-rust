// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_listener_description(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ListenerDescription, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ListenerDescription::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Listener") /* Listener com.amazonaws.elasticloadbalancing#ListenerDescription$Listener */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_listener::de_listener(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_listener(var_1);
            }
            ,
            s if s.matches("PolicyNames") /* PolicyNames com.amazonaws.elasticloadbalancing#ListenerDescription$PolicyNames */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_policy_names::de_policy_names(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_policy_names(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

