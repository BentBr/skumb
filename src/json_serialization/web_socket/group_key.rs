use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/**
 * `GroupKey` struct - Contains a freshly generated key for a group, encrypted with the public key for one specific user.
 */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupKey {
    pub encrypted_key: String,
    pub iv: String,
    pub creation_date: NaiveDateTime,
    pub for_user_id: String,
    pub from_user_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use serde_json;

    #[test]
    fn test_group_key_serialize() {
        let creation_date = NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap();
        let group_key = GroupKey {
            encrypted_key: "some_key".to_string(),
            iv: "iv".to_string(),
            creation_date,
            for_user_id: "user123".to_string(),
            from_user_id: "user=6789".to_string(),
        };
        let json = serde_json::to_string(&group_key).unwrap();
        assert_eq!(
            json,
            r#"{"encrypted_key":"some_key","iv":"iv","creation_date":"2023-10-01T12:34:56","for_user_id":"user123","from_user_id":"user=6789"}"#
        );
    }

    #[test]
    fn test_group_key_deserialize() {
        let json = r#"{"encrypted_key":"some_key","iv":"iv","creation_date":"2023-10-01T12:34:56","for_user_id":"user123","from_user_id":"user=6789"}"#;
        let group_key: GroupKey = serde_json::from_str(json).unwrap();
        let creation_date = NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap();
        assert_eq!(group_key.encrypted_key, "some_key".to_string());
        assert_eq!(group_key.iv, "iv".to_string());
        assert_eq!(group_key.creation_date, creation_date);
        assert_eq!(group_key.for_user_id, "user123".to_string());
        assert_eq!(group_key.from_user_id, "user=6789".to_string());
    }

    #[test]
    fn test_group_key_new() {
        let creation_date = NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap();
        let group_key = GroupKey {
            encrypted_key: "some_key".to_string(),
            iv: "iv".to_string(),
            creation_date,
            for_user_id: "user123".to_string(),
            from_user_id: "user=6789".to_string(),
        };
        assert_eq!(group_key.encrypted_key, "some_key".to_string());
        assert_eq!(
            group_key.creation_date,
            NaiveDateTime::parse_from_str("2023-10-01T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap()
        );
        assert_eq!(group_key.for_user_id, "user123".to_string());
        assert_eq!(group_key.from_user_id, "user=6789".to_string());
    }
}
