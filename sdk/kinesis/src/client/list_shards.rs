// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListShards`](crate::client::fluent_builders::ListShards) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stream_name(impl Into<String>)`](crate::client::fluent_builders::ListShards::stream_name) / [`set_stream_name(Option<String>)`](crate::client::fluent_builders::ListShards::set_stream_name): <p>The name of the data stream whose shards you want to list. </p>  <p>You cannot specify this parameter if you specify the <code>NextToken</code> parameter.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListShards::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListShards::set_next_token): <p>When the number of shards in the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of shards in the data stream, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListShards</code> to list the next set of shards.</p>  <p>Don't specify <code>StreamName</code> or <code>StreamCreationTimestamp</code> if you specify <code>NextToken</code> because the latter unambiguously identifies the stream.</p>  <p>You can optionally specify a value for the <code>MaxResults</code> parameter when you specify <code>NextToken</code>. If you specify a <code>MaxResults</code> value that is less than the number of shards that the operation returns if you don't specify <code>MaxResults</code>, the response will contain a new <code>NextToken</code> value. You can use the new <code>NextToken</code> value in a subsequent call to the <code>ListShards</code> operation.</p> <important>   <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListShards</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListShards</code>, you get <code>ExpiredNextTokenException</code>.</p>  </important>
    ///   - [`exclusive_start_shard_id(impl Into<String>)`](crate::client::fluent_builders::ListShards::exclusive_start_shard_id) / [`set_exclusive_start_shard_id(Option<String>)`](crate::client::fluent_builders::ListShards::set_exclusive_start_shard_id): <p>Specify this parameter to indicate that you want to list the shards starting with the shard whose ID immediately follows <code>ExclusiveStartShardId</code>.</p>  <p>If you don't specify this parameter, the default behavior is for <code>ListShards</code> to list the shards starting with the first one in the stream.</p>  <p>You cannot specify this parameter if you specify <code>NextToken</code>.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListShards::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListShards::set_max_results): <p>The maximum number of shards to return in a single call to <code>ListShards</code>. The maximum number of shards to return in a single call. The default value is 1000. If you specify a value greater than 1000, at most 1000 results are returned. </p>  <p>When the number of shards to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListShards</code> to list the next set of shards.</p>
    ///   - [`stream_creation_timestamp(DateTime)`](crate::client::fluent_builders::ListShards::stream_creation_timestamp) / [`set_stream_creation_timestamp(Option<DateTime>)`](crate::client::fluent_builders::ListShards::set_stream_creation_timestamp): <p>Specify this input parameter to distinguish data streams that have the same name. For example, if you create a data stream and then delete it, and you later create another data stream with the same name, you can use this input parameter to specify which of the two streams you want to list the shards for.</p>  <p>You cannot specify this parameter if you specify the <code>NextToken</code> parameter.</p>
    ///   - [`shard_filter(ShardFilter)`](crate::client::fluent_builders::ListShards::shard_filter) / [`set_shard_filter(Option<ShardFilter>)`](crate::client::fluent_builders::ListShards::set_shard_filter): <p>Enables you to filter out the response of the <code>ListShards</code> API. You can only specify one filter at a time. </p>  <p>If you use the <code>ShardFilter</code> parameter when invoking the ListShards API, the <code>Type</code> is the required property and must be specified. If you specify the <code>AT_TRIM_HORIZON</code>, <code>FROM_TRIM_HORIZON</code>, or <code>AT_LATEST</code> types, you do not need to specify either the <code>ShardId</code> or the <code>Timestamp</code> optional properties. </p>  <p>If you specify the <code>AFTER_SHARD_ID</code> type, you must also provide the value for the optional <code>ShardId</code> property. The <code>ShardId</code> property is identical in fuctionality to the <code>ExclusiveStartShardId</code> parameter of the <code>ListShards</code> API. When <code>ShardId</code> property is specified, the response includes the shards starting with the shard whose ID immediately follows the <code>ShardId</code> that you provided. </p>  <p>If you specify the <code>AT_TIMESTAMP</code> or <code>FROM_TIMESTAMP_ID</code> type, you must also provide the value for the optional <code>Timestamp</code> property. If you specify the AT_TIMESTAMP type, then all shards that were open at the provided timestamp are returned. If you specify the FROM_TIMESTAMP type, then all shards starting from the provided timestamp to TIP are returned. </p>
    ///   - [`stream_arn(impl Into<String>)`](crate::client::fluent_builders::ListShards::stream_arn) / [`set_stream_arn(Option<String>)`](crate::client::fluent_builders::ListShards::set_stream_arn): <p>The ARN of the stream.</p>
                            /// - On success, responds with [`ListShardsOutput`](crate::output::ListShardsOutput) with field(s):
    ///   - [`shards(Option<Vec<Shard>>)`](crate::output::ListShardsOutput::shards): <p>An array of JSON objects. Each object represents one shard and specifies the IDs of the shard, the shard's parent, and the shard that's adjacent to the shard's parent. Each object also contains the starting and ending hash keys and the starting and ending sequence numbers for the shard.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListShardsOutput::next_token): <p>When the number of shards in the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of shards in the data stream, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListShards</code> to list the next set of shards. For more information about the use of this pagination token when calling the <code>ListShards</code> operation, see <code>ListShardsInput$NextToken</code>.</p> <important>   <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListShards</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListShards</code>, you get <code>ExpiredNextTokenException</code>.</p>  </important>
                            /// - On failure, responds with [`SdkError<ListShardsError>`](crate::error::ListShardsError)
    pub fn list_shards(&self) -> crate::client::fluent_builders::ListShards {
                                crate::client::fluent_builders::ListShards::new(self.handle.clone())
                            }
}

