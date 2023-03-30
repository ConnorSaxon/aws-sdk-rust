// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendMessage`](crate::client::fluent_builders::SendMessage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`queue_url(impl Into<String>)`](crate::client::fluent_builders::SendMessage::queue_url) / [`set_queue_url(Option<String>)`](crate::client::fluent_builders::SendMessage::set_queue_url): <p>The URL of the Amazon SQS queue to which a message is sent.</p>  <p>Queue URLs and names are case-sensitive.</p>
    ///   - [`message_body(impl Into<String>)`](crate::client::fluent_builders::SendMessage::message_body) / [`set_message_body(Option<String>)`](crate::client::fluent_builders::SendMessage::set_message_body): <p>The message to send. The minimum size is one character. The maximum size is 256 KB.</p> <important>   <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed:</p>   <p> <code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code> </p>   <p>Any characters not included in this list will be rejected. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p>  </important>
    ///   - [`delay_seconds(i32)`](crate::client::fluent_builders::SendMessage::delay_seconds) / [`set_delay_seconds(i32)`](crate::client::fluent_builders::SendMessage::set_delay_seconds): <p> The length of time, in seconds, for which to delay a specific message. Valid values: 0 to 900. Maximum: 15 minutes. Messages with a positive <code>DelaySeconds</code> value become available for processing after the delay period is finished. If you don't specify a value, the default value for the queue applies. </p> <note>   <p>When you set <code>FifoQueue</code>, you can't set <code>DelaySeconds</code> per message. You can set this parameter only on a queue level.</p>  </note>
    ///   - [`message_attributes(HashMap<String, MessageAttributeValue>)`](crate::client::fluent_builders::SendMessage::message_attributes) / [`set_message_attributes(Option<HashMap<String, MessageAttributeValue>>)`](crate::client::fluent_builders::SendMessage::set_message_attributes): <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html#sqs-message-attributes">Amazon SQS message attributes</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    ///   - [`message_system_attributes(HashMap<MessageSystemAttributeNameForSends, MessageSystemAttributeValue>)`](crate::client::fluent_builders::SendMessage::message_system_attributes) / [`set_message_system_attributes(Option<HashMap<MessageSystemAttributeNameForSends, MessageSystemAttributeValue>>)`](crate::client::fluent_builders::SendMessage::set_message_system_attributes): <p>The message system attribute to send. Each message system attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>.</p> <important>   <ul>    <li> <p>Currently, the only supported message system attribute is <code>AWSTraceHeader</code>. Its type must be <code>String</code> and its value must be a correctly formatted X-Ray trace header string.</p> </li>    <li> <p>The size of a message system attribute doesn't count towards the total size of a message.</p> </li>   </ul>  </important>
    ///   - [`message_deduplication_id(impl Into<String>)`](crate::client::fluent_builders::SendMessage::message_deduplication_id) / [`set_message_deduplication_id(Option<String>)`](crate::client::fluent_builders::SendMessage::set_message_deduplication_id): <p>This parameter applies only to FIFO (first-in-first-out) queues.</p>  <p>The token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any messages sent with the same <code>MessageDeduplicationId</code> are accepted successfully but aren't delivered during the 5-minute deduplication interval. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html"> Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>.</p>  <ul>   <li> <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p>    <ul>     <li> <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p> </li>     <li> <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message). </p> </li>     <li> <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p> </li>     <li> <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p> </li>    </ul> </li>   <li> <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p> </li>   <li> <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered. </p> </li>  </ul> <note>   <p>The <code>MessageDeduplicationId</code> is available to the consumer of the message (this can be useful for troubleshooting delivery issues).</p>   <p>If a message is sent successfully but the acknowledgement is lost and the message is resent with the same <code>MessageDeduplicationId</code> after the deduplication interval, Amazon SQS can't detect duplicate messages.</p>   <p>Amazon SQS continues to keep track of the message deduplication ID even after the message is received and deleted.</p>  </note>  <p>The maximum length of <code>MessageDeduplicationId</code> is 128 characters. <code>MessageDeduplicationId</code> can contain alphanumeric characters (<code>a-z</code>, <code>A-Z</code>, <code>0-9</code>) and punctuation (<code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~</code>).</p>  <p>For best practices of using <code>MessageDeduplicationId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagededuplicationid-property.html">Using the MessageDeduplicationId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    ///   - [`message_group_id(impl Into<String>)`](crate::client::fluent_builders::SendMessage::message_group_id) / [`set_message_group_id(Option<String>)`](crate::client::fluent_builders::SendMessage::set_message_group_id): <p>This parameter applies only to FIFO (first-in-first-out) queues.</p>  <p>The tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). To interleave multiple ordered streams within a single queue, use <code>MessageGroupId</code> values (for example, session data for multiple users). In this scenario, multiple consumers can process the queue, but the session data of each user is processed in a FIFO fashion.</p>  <ul>   <li> <p>You must associate a non-empty <code>MessageGroupId</code> with a message. If you don't provide a <code>MessageGroupId</code>, the action fails.</p> </li>   <li> <p> <code>ReceiveMessage</code> might return messages with multiple <code>MessageGroupId</code> values. For each <code>MessageGroupId</code>, the messages are sorted by time sent. The caller can't specify a <code>MessageGroupId</code>.</p> </li>  </ul>  <p>The length of <code>MessageGroupId</code> is 128 characters. Valid values: alphanumeric characters and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~)</code>.</p>  <p>For best practices of using <code>MessageGroupId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagegroupid-property.html">Using the MessageGroupId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p> <important>   <p> <code>MessageGroupId</code> is required for FIFO queues. You can't use it for Standard queues.</p>  </important>
                            /// - On success, responds with [`SendMessageOutput`](crate::output::SendMessageOutput) with field(s):
    ///   - [`md5_of_message_body(Option<String>)`](crate::output::SendMessageOutput::md5_of_message_body): <p>An MD5 digest of the non-URL-encoded message body string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
    ///   - [`md5_of_message_attributes(Option<String>)`](crate::output::SendMessageOutput::md5_of_message_attributes): <p>An MD5 digest of the non-URL-encoded message attribute string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
    ///   - [`md5_of_message_system_attributes(Option<String>)`](crate::output::SendMessageOutput::md5_of_message_system_attributes): <p>An MD5 digest of the non-URL-encoded message system attribute string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest.</p>
    ///   - [`message_id(Option<String>)`](crate::output::SendMessageOutput::message_id): <p>An attribute containing the <code>MessageId</code> of the message sent to the queue. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-queue-message-identifiers.html">Queue and Message Identifiers</a> in the <i>Amazon SQS Developer Guide</i>. </p>
    ///   - [`sequence_number(Option<String>)`](crate::output::SendMessageOutput::sequence_number): <p>This parameter applies only to FIFO (first-in-first-out) queues.</p>  <p>The large, non-consecutive number that Amazon SQS assigns to each message.</p>  <p>The length of <code>SequenceNumber</code> is 128 bits. <code>SequenceNumber</code> continues to increase for a particular <code>MessageGroupId</code>.</p>
                            /// - On failure, responds with [`SdkError<SendMessageError>`](crate::error::SendMessageError)
    pub fn send_message(&self) -> crate::client::fluent_builders::SendMessage {
                                crate::client::fluent_builders::SendMessage::new(self.handle.clone())
                            }
}

