// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_network_interface_permission_state(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::NetworkInterfacePermissionState, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::NetworkInterfacePermissionState::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("state") /* State com.amazonaws.ec2#NetworkInterfacePermissionState$State */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::NetworkInterfacePermissionStateCode, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::NetworkInterfacePermissionStateCode::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_1);
            }
            ,
            s if s.matches("statusMessage") /* StatusMessage com.amazonaws.ec2#NetworkInterfacePermissionState$StatusMessage */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_message(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

