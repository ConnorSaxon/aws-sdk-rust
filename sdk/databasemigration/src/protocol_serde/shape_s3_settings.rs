// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3Settings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.service_access_role_arn {
        object.key("ServiceAccessRoleArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.external_table_definition {
        object.key("ExternalTableDefinition").string(var_2.as_str());
    }
    if let Some(var_3) = &input.csv_row_delimiter {
        object.key("CsvRowDelimiter").string(var_3.as_str());
    }
    if let Some(var_4) = &input.csv_delimiter {
        object.key("CsvDelimiter").string(var_4.as_str());
    }
    if let Some(var_5) = &input.bucket_folder {
        object.key("BucketFolder").string(var_5.as_str());
    }
    if let Some(var_6) = &input.bucket_name {
        object.key("BucketName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.compression_type {
        object.key("CompressionType").string(var_7.as_str());
    }
    if let Some(var_8) = &input.encryption_mode {
        object.key("EncryptionMode").string(var_8.as_str());
    }
    if let Some(var_9) = &input.server_side_encryption_kms_key_id {
        object.key("ServerSideEncryptionKmsKeyId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.data_format {
        object.key("DataFormat").string(var_10.as_str());
    }
    if let Some(var_11) = &input.encoding_type {
        object.key("EncodingType").string(var_11.as_str());
    }
    if let Some(var_12) = &input.dict_page_size_limit {
        object.key("DictPageSizeLimit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    if let Some(var_13) = &input.row_group_length {
        object.key("RowGroupLength").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_13).into()));
    }
    if let Some(var_14) = &input.data_page_size {
        object.key("DataPageSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_14).into()));
    }
    if let Some(var_15) = &input.parquet_version {
        object.key("ParquetVersion").string(var_15.as_str());
    }
    if let Some(var_16) = &input.enable_statistics {
        object.key("EnableStatistics").boolean(*var_16);
    }
    if let Some(var_17) = &input.include_op_for_full_load {
        object.key("IncludeOpForFullLoad").boolean(*var_17);
    }
    if let Some(var_18) = &input.cdc_inserts_only {
        object.key("CdcInsertsOnly").boolean(*var_18);
    }
    if let Some(var_19) = &input.timestamp_column_name {
        object.key("TimestampColumnName").string(var_19.as_str());
    }
    if let Some(var_20) = &input.parquet_timestamp_in_millisecond {
        object.key("ParquetTimestampInMillisecond").boolean(*var_20);
    }
    if let Some(var_21) = &input.cdc_inserts_and_updates {
        object.key("CdcInsertsAndUpdates").boolean(*var_21);
    }
    if let Some(var_22) = &input.date_partition_enabled {
        object.key("DatePartitionEnabled").boolean(*var_22);
    }
    if let Some(var_23) = &input.date_partition_sequence {
        object.key("DatePartitionSequence").string(var_23.as_str());
    }
    if let Some(var_24) = &input.date_partition_delimiter {
        object.key("DatePartitionDelimiter").string(var_24.as_str());
    }
    if let Some(var_25) = &input.use_csv_no_sup_value {
        object.key("UseCsvNoSupValue").boolean(*var_25);
    }
    if let Some(var_26) = &input.csv_no_sup_value {
        object.key("CsvNoSupValue").string(var_26.as_str());
    }
    if let Some(var_27) = &input.preserve_transactions {
        object.key("PreserveTransactions").boolean(*var_27);
    }
    if let Some(var_28) = &input.cdc_path {
        object.key("CdcPath").string(var_28.as_str());
    }
    if let Some(var_29) = &input.use_task_start_time_for_full_load_timestamp {
        object.key("UseTaskStartTimeForFullLoadTimestamp").boolean(*var_29);
    }
    if let Some(var_30) = &input.canned_acl_for_objects {
        object.key("CannedAclForObjects").string(var_30.as_str());
    }
    if let Some(var_31) = &input.add_column_name {
        object.key("AddColumnName").boolean(*var_31);
    }
    if let Some(var_32) = &input.cdc_max_batch_interval {
        object.key("CdcMaxBatchInterval").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_32).into()));
    }
    if let Some(var_33) = &input.cdc_min_file_size {
        object.key("CdcMinFileSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_33).into()));
    }
    if let Some(var_34) = &input.csv_null_value {
        object.key("CsvNullValue").string(var_34.as_str());
    }
    if let Some(var_35) = &input.ignore_header_rows {
        object.key("IgnoreHeaderRows").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_35).into()));
    }
    if let Some(var_36) = &input.max_file_size {
        object.key("MaxFileSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_36).into()));
    }
    if let Some(var_37) = &input.rfc4180 {
        object.key("Rfc4180").boolean(*var_37);
    }
    if let Some(var_38) = &input.date_partition_timezone {
        object.key("DatePartitionTimezone").string(var_38.as_str());
    }
    if let Some(var_39) = &input.add_trailing_padding_character {
        object.key("AddTrailingPaddingCharacter").boolean(*var_39);
    }
    if let Some(var_40) = &input.expected_bucket_owner {
        object.key("ExpectedBucketOwner").string(var_40.as_str());
    }
    Ok(())
}

pub(crate) fn de_s3_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::S3Settings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::s3_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ServiceAccessRoleArn" => {
                                builder = builder.set_service_access_role_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ExternalTableDefinition" => {
                                builder = builder.set_external_table_definition(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CsvRowDelimiter" => {
                                builder = builder.set_csv_row_delimiter(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CsvDelimiter" => {
                                builder = builder.set_csv_delimiter(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "BucketFolder" => {
                                builder = builder.set_bucket_folder(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "BucketName" => {
                                builder = builder.set_bucket_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CompressionType" => {
                                builder = builder.set_compression_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CompressionTypeValue::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "EncryptionMode" => {
                                builder = builder.set_encryption_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::EncryptionModeValue::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ServerSideEncryptionKmsKeyId" => {
                                builder = builder.set_server_side_encryption_kms_key_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DataFormat" => {
                                builder = builder.set_data_format(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::DataFormatValue::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "EncodingType" => {
                                builder = builder.set_encoding_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::EncodingTypeValue::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "DictPageSizeLimit" => {
                                builder = builder.set_dict_page_size_limit(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "RowGroupLength" => {
                                builder = builder.set_row_group_length(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "DataPageSize" => {
                                builder = builder.set_data_page_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "ParquetVersion" => {
                                builder = builder.set_parquet_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ParquetVersionValue::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "EnableStatistics" => {
                                builder = builder.set_enable_statistics(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "IncludeOpForFullLoad" => {
                                builder = builder.set_include_op_for_full_load(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CdcInsertsOnly" => {
                                builder = builder.set_cdc_inserts_only(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "TimestampColumnName" => {
                                builder = builder.set_timestamp_column_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ParquetTimestampInMillisecond" => {
                                builder = builder.set_parquet_timestamp_in_millisecond(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CdcInsertsAndUpdates" => {
                                builder = builder.set_cdc_inserts_and_updates(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "DatePartitionEnabled" => {
                                builder = builder.set_date_partition_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "DatePartitionSequence" => {
                                builder = builder.set_date_partition_sequence(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::DatePartitionSequenceValue::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "DatePartitionDelimiter" => {
                                builder = builder.set_date_partition_delimiter(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::DatePartitionDelimiterValue::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "UseCsvNoSupValue" => {
                                builder = builder.set_use_csv_no_sup_value(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CsvNoSupValue" => {
                                builder = builder.set_csv_no_sup_value(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "PreserveTransactions" => {
                                builder = builder.set_preserve_transactions(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CdcPath" => {
                                builder = builder.set_cdc_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "UseTaskStartTimeForFullLoadTimestamp" => {
                                builder = builder.set_use_task_start_time_for_full_load_timestamp(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CannedAclForObjects" => {
                                builder = builder.set_canned_acl_for_objects(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CannedAclForObjectsValue::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "AddColumnName" => {
                                builder = builder.set_add_column_name(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CdcMaxBatchInterval" => {
                                builder = builder.set_cdc_max_batch_interval(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CdcMinFileSize" => {
                                builder = builder.set_cdc_min_file_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CsvNullValue" => {
                                builder = builder.set_csv_null_value(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "IgnoreHeaderRows" => {
                                builder = builder.set_ignore_header_rows(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "MaxFileSize" => {
                                builder = builder.set_max_file_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "Rfc4180" => {
                                builder = builder.set_rfc4180(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "DatePartitionTimezone" => {
                                builder = builder.set_date_partition_timezone(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "AddTrailingPaddingCharacter" => {
                                builder = builder.set_add_trailing_padding_character(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "ExpectedBucketOwner" => {
                                builder = builder.set_expected_bucket_owner(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

