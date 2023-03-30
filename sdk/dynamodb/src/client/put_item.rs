// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutItem`](crate::client::fluent_builders::PutItem) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<String>)`](crate::client::fluent_builders::PutItem::table_name) / [`set_table_name(Option<String>)`](crate::client::fluent_builders::PutItem::set_table_name): <p>The name of the table to contain the item.</p>
    ///   - [`item(HashMap<String, AttributeValue>)`](crate::client::fluent_builders::PutItem::item) / [`set_item(Option<HashMap<String, AttributeValue>>)`](crate::client::fluent_builders::PutItem::set_item): <p>A map of attribute name/value pairs, one for each attribute. Only the primary key attributes are required; you can optionally provide other attribute name-value pairs for the item.</p>  <p>You must provide all of the attributes for the primary key. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide both values for both the partition key and the sort key.</p>  <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p>  <p>Empty String and Binary attribute values are allowed. Attribute values of type String and Binary must have a length greater than zero if the attribute is used as a key attribute for a table or index.</p>  <p>For more information about primary keys, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.CoreComponents.html#HowItWorks.CoreComponents.PrimaryKey">Primary Key</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>  <p>Each element in the <code>Item</code> map is an <code>AttributeValue</code> object.</p>
    ///   - [`expected(HashMap<String, ExpectedAttributeValue>)`](crate::client::fluent_builders::PutItem::expected) / [`set_expected(Option<HashMap<String, ExpectedAttributeValue>>)`](crate::client::fluent_builders::PutItem::set_expected): <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ///   - [`return_values(ReturnValue)`](crate::client::fluent_builders::PutItem::return_values) / [`set_return_values(Option<ReturnValue>)`](crate::client::fluent_builders::PutItem::set_return_values): <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were updated with the <code>PutItem</code> request. For <code>PutItem</code>, the valid values are:</p>  <ul>   <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li>   <li> <p> <code>ALL_OLD</code> - If <code>PutItem</code> overwrote an attribute name-value pair, then the content of the old item is returned.</p> </li>  </ul>  <p>The values returned are strongly consistent.</p>  <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p> <note>   <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>PutItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p>  </note>
    ///   - [`return_consumed_capacity(ReturnConsumedCapacity)`](crate::client::fluent_builders::PutItem::return_consumed_capacity) / [`set_return_consumed_capacity(Option<ReturnConsumedCapacity>)`](crate::client::fluent_builders::PutItem::set_return_consumed_capacity): <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>  <ul>   <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>   <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>   <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>  </ul>
    ///   - [`return_item_collection_metrics(ReturnItemCollectionMetrics)`](crate::client::fluent_builders::PutItem::return_item_collection_metrics) / [`set_return_item_collection_metrics(Option<ReturnItemCollectionMetrics>)`](crate::client::fluent_builders::PutItem::set_return_item_collection_metrics): <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    ///   - [`conditional_operator(ConditionalOperator)`](crate::client::fluent_builders::PutItem::conditional_operator) / [`set_conditional_operator(Option<ConditionalOperator>)`](crate::client::fluent_builders::PutItem::set_conditional_operator): <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ///   - [`condition_expression(impl Into<String>)`](crate::client::fluent_builders::PutItem::condition_expression) / [`set_condition_expression(Option<String>)`](crate::client::fluent_builders::PutItem::set_condition_expression): <p>A condition that must be satisfied in order for a conditional <code>PutItem</code> operation to succeed.</p>  <p>An expression can contain any of the following:</p>  <ul>   <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li>   <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li>   <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li>  </ul>  <p>For more information on condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ///   - [`expression_attribute_names(HashMap<String, String>)`](crate::client::fluent_builders::PutItem::expression_attribute_names) / [`set_expression_attribute_names(Option<HashMap<String, String>>)`](crate::client::fluent_builders::PutItem::set_expression_attribute_names): <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>  <ul>   <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li>   <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li>   <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li>  </ul>  <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>  <ul>   <li> <p> <code>Percentile</code> </p> </li>  </ul>  <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>  <ul>   <li> <p> <code>{"#P":"Percentile"}</code> </p> </li>  </ul>  <p>You could then use this substitution in an expression, as in this example:</p>  <ul>   <li> <p> <code>#P = :val</code> </p> </li>  </ul> <note>   <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>  </note>  <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ///   - [`expression_attribute_values(HashMap<String, AttributeValue>)`](crate::client::fluent_builders::PutItem::expression_attribute_values) / [`set_expression_attribute_values(Option<HashMap<String, AttributeValue>>)`](crate::client::fluent_builders::PutItem::set_expression_attribute_values): <p>One or more values that can be substituted in an expression.</p>  <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p>  <p> <code>Available | Backordered | Discontinued</code> </p>  <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>  <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p>  <p>You could then use these values in an expression, such as this:</p>  <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p>  <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
                            /// - On success, responds with [`PutItemOutput`](crate::output::PutItemOutput) with field(s):
    ///   - [`attributes(Option<HashMap<String, AttributeValue>>)`](crate::output::PutItemOutput::attributes): <p>The attribute values as they appeared before the <code>PutItem</code> operation, but only if <code>ReturnValues</code> is specified as <code>ALL_OLD</code> in the request. Each element consists of an attribute name and an attribute value.</p>
    ///   - [`consumed_capacity(Option<ConsumedCapacity>)`](crate::output::PutItemOutput::consumed_capacity): <p>The capacity units consumed by the <code>PutItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Read/Write Capacity Mode</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ///   - [`item_collection_metrics(Option<ItemCollectionMetrics>)`](crate::output::PutItemOutput::item_collection_metrics): <p>Information about item collections, if any, that were affected by the <code>PutItem</code> operation. <code>ItemCollectionMetrics</code> is only returned if the <code>ReturnItemCollectionMetrics</code> parameter was specified. If the table does not have any local secondary indexes, this information is not returned in the response.</p>  <p>Each <code>ItemCollectionMetrics</code> element consists of:</p>  <ul>   <li> <p> <code>ItemCollectionKey</code> - The partition key value of the item collection. This is the same as the partition key value of the item itself.</p> </li>   <li> <p> <code>SizeEstimateRangeGB</code> - An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p> </li>  </ul>
                            /// - On failure, responds with [`SdkError<PutItemError>`](crate::error::PutItemError)
    pub fn put_item(&self) -> crate::client::fluent_builders::PutItem {
                                crate::client::fluent_builders::PutItem::new(self.handle.clone())
                            }
}

