use serde::{Serialize, Serializer};
use std::fmt;

#[derive(PartialEq, Debug, Eq)]
pub enum Status {
    Success,
    Error,
}

impl Status {
    pub fn stringify(&self) -> String {
        match self {
            Self::Success => "Success".to_string(),
            Self::Error => "Error".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn from_string(input_string: &str) -> Self {
        match input_string {
            "Success" => Self::Success,
            "Error" => Self::Error,
            _ => panic!("Input '{input_string}' not supported as at valid response status"),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.stringify())
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.stringify().as_str())
    }
}

#[cfg(test)]
mod response_status_tests {
    use super::Status;

    #[test]
    fn stringify() {
        let success_status = Status::Success;
        let error_status = Status::Error;

        assert_eq!(success_status.stringify(), "Success".to_string());
        assert_eq!(error_status.stringify(), "Error".to_string());
    }

    #[test]
    fn from_string() {
        let success_string = "Success";
        let error_string = "Error";

        assert_eq!(Status::from_string(success_string), Status::Success);
        assert_eq!(Status::from_string(error_string), Status::Error);
    }

    #[test]
    #[should_panic(expected = "Input 'fail' not supported as at valid response status")]
    fn from_string_panic() {
        let not_existing_status_string = "fail";

        Status::from_string(not_existing_status_string);
    }

    #[test]
    fn display() {
        let success_status = Status::Success;
        let error_status = Status::Error;

        assert_eq!(format!("{}", success_status), "Success");
        assert_eq!(format!("{}", error_status), "Error");
    }

    #[test]
    fn serialize() {
        use serde_json;

        let success_status = Status::Success;
        let error_status = Status::Error;

        assert_eq!(
            serde_json::to_string(&success_status).unwrap(),
            "\"Success\""
        );
        assert_eq!(serde_json::to_string(&error_status).unwrap(), "\"Error\"");
    }
}
