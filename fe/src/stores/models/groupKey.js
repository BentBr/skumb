class GroupKey {
    constructor(encrypted_key, iv, creation_date, for_user_id, from_user_id) {
        this.encrypted_key = encrypted_key?.value || encrypted_key
        this.iv = iv?.value || iv
        this.creation_date = creation_date?.value || creation_date
        this.for_user_id = for_user_id?.value || for_user_id
        this.from_user_id = from_user_id?.value || from_user_id
    }

    toPlainObject() {
        return {
            encrypted_key: this.encrypted_key,
            iv: this.iv,
            creation_date: this.creation_date,
            for_user_id: this.for_user_id,
            from_user_id: this.from_user_id,
        }
    }
}

export default GroupKey
