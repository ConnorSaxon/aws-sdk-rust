// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_option_group_option_setting(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::OptionGroupOptionSetting, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::OptionGroupOptionSetting::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("SettingName") /* SettingName com.amazonaws.rds#OptionGroupOptionSetting$SettingName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_setting_name(var_1);
            }
            ,
            s if s.matches("SettingDescription") /* SettingDescription com.amazonaws.rds#OptionGroupOptionSetting$SettingDescription */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_setting_description(var_2);
            }
            ,
            s if s.matches("DefaultValue") /* DefaultValue com.amazonaws.rds#OptionGroupOptionSetting$DefaultValue */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_default_value(var_3);
            }
            ,
            s if s.matches("ApplyType") /* ApplyType com.amazonaws.rds#OptionGroupOptionSetting$ApplyType */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_apply_type(var_4);
            }
            ,
            s if s.matches("AllowedValues") /* AllowedValues com.amazonaws.rds#OptionGroupOptionSetting$AllowedValues */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_allowed_values(var_5);
            }
            ,
            s if s.matches("IsModifiable") /* IsModifiable com.amazonaws.rds#OptionGroupOptionSetting$IsModifiable */ =>  {
                let var_6 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_modifiable(var_6);
            }
            ,
            s if s.matches("IsRequired") /* IsRequired com.amazonaws.rds#OptionGroupOptionSetting$IsRequired */ =>  {
                let var_7 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_required(var_7);
            }
            ,
            s if s.matches("MinimumEngineVersionPerAllowedValue") /* MinimumEngineVersionPerAllowedValue com.amazonaws.rds#OptionGroupOptionSetting$MinimumEngineVersionPerAllowedValue */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_minimum_engine_version_per_allowed_value_list::de_minimum_engine_version_per_allowed_value_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_minimum_engine_version_per_allowed_value(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

