// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_change(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::Change, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::Change::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Type") /* Type com.amazonaws.cloudformation#Change$Type */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::ChangeType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::ChangeType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_1);
            }
            ,
            s if s.matches("HookInvocationCount") /* HookInvocationCount com.amazonaws.cloudformation#Change$HookInvocationCount */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudformation#HookInvocationCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_hook_invocation_count(var_2);
            }
            ,
            s if s.matches("ResourceChange") /* ResourceChange com.amazonaws.cloudformation#Change$ResourceChange */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_resource_change::de_resource_change(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_resource_change(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

