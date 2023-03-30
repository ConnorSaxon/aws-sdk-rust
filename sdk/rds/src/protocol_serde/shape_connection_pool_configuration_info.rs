// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_connection_pool_configuration_info(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ConnectionPoolConfigurationInfo, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ConnectionPoolConfigurationInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("MaxConnectionsPercent") /* MaxConnectionsPercent com.amazonaws.rds#ConnectionPoolConfigurationInfo$MaxConnectionsPercent */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.rds#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_connections_percent(var_1);
            }
            ,
            s if s.matches("MaxIdleConnectionsPercent") /* MaxIdleConnectionsPercent com.amazonaws.rds#ConnectionPoolConfigurationInfo$MaxIdleConnectionsPercent */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.rds#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_idle_connections_percent(var_2);
            }
            ,
            s if s.matches("ConnectionBorrowTimeout") /* ConnectionBorrowTimeout com.amazonaws.rds#ConnectionPoolConfigurationInfo$ConnectionBorrowTimeout */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.rds#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_connection_borrow_timeout(var_3);
            }
            ,
            s if s.matches("SessionPinningFilters") /* SessionPinningFilters com.amazonaws.rds#ConnectionPoolConfigurationInfo$SessionPinningFilters */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_string_list::de_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_session_pinning_filters(var_4);
            }
            ,
            s if s.matches("InitQuery") /* InitQuery com.amazonaws.rds#ConnectionPoolConfigurationInfo$InitQuery */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_init_query(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

