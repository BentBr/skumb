class ChatMessage {
    constructor(user_id, cipher, iv, uuid = 'temp', message_sent_at = new Date().toISOString().split('.')[0]) {
        this.user_id = user_id
        this.cipher = cipher
        this.iv = iv
        this.uuid = uuid
        this.message_sent_at = message_sent_at
    }
}

export default ChatMessage
