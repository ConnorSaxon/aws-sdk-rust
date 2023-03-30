// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vpn_tunnel_log_options(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::VpnTunnelLogOptions, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::VpnTunnelLogOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("cloudWatchLogOptions") /* CloudWatchLogOptions com.amazonaws.ec2#VpnTunnelLogOptions$CloudWatchLogOptions */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cloud_watch_log_options::de_cloud_watch_log_options(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cloud_watch_log_options(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

