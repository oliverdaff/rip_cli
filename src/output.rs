


pub mod json {
    use ripioc::IOCS;
    use serde_json::to_string;

    pub fn output_json(input : &IOCS) -> String {
        to_string(input).unwrap()
    }

}