class Connection {
    constructor(status, user_id) {
        this.status = status?.value || status
        this.user_id = user_id?.value || user_id
    }

    toPlainObject() {
        return {
            status: this.status,
            user_id: this.user_id,
        }
    }
}

export default Connection
