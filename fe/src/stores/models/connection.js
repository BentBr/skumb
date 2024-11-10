class Connection {
    constructor(status, user_id, user_name, public_key) {
        this.status = status?.value || status
        this.user_id = user_id?.value || user_id
        this.user_name = user_name?.value || user_name
        this.public_key = public_key?.value || public_key
    }

    toPlainObject() {
        return {
            status: this.status,
            user_id: this.user_id,
            user_name: this.user_name,
            public_key: this.public_key,
        }
    }
}

export default Connection
