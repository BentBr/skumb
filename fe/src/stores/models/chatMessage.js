class ChatMessage {
    constructor(user_id, text, uuid = 'temp', message_sent_at = new Date().toISOString()) {
        this.user_id = user_id
        this.text = text
        this.uuid = uuid
        this.message_sent_at = message_sent_at
    }
}

export default ChatMessage
