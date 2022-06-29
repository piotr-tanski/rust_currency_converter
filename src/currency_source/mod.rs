use curl::easy::{Easy2, Handler, WriteError};
use json::JsonValue;

use crate::converter::RateSource;
use crate::currency::Currency;

pub struct RateCollector(Option<f32>);

pub struct NBP {
    curl: Easy2<RateCollector>,
}

impl NBP {
    pub fn new() -> NBP {
        NBP {
            curl: Easy2::new(RateCollector(None)),
        }
    }

    fn perform_get(&mut self, url: String) {
        self.curl.get(true).unwrap();
        self.curl.url(&*url).unwrap();
        self.curl.perform().unwrap_or_else(|err| {
            println!("Query to {} failed: [{}] {}", url, err.code(), err.description());
        });
    }

    fn create_url(currency_code: &String) -> String {
        const URL_TEMPLATE: &str = "http://api.nbp.pl/api/exchangerates/rates/A";
        const FORMAT: &str = "format=json";
        format!("{URL_TEMPLATE}/{currency_code}?{FORMAT}")
    }
}

impl Handler for RateCollector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        let data_str = std::str::from_utf8(data).unwrap_or("");
        let parsed = json::parse(data_str).unwrap_or(JsonValue::Null);
        if parsed == JsonValue::Null {
            println!("Unable to parse a response.");
            return Ok(0);
        }

        self.0 = Some(parsed["rates"][0]["mid"].as_f32().unwrap());
        return Ok(data.len());
    }
}

impl RateSource for NBP {
    fn get_rate(&mut self, currency: &Currency) -> Option<f32> {
        self.perform_get(NBP::create_url(&currency.code));
        let result = self.curl.get_ref();
        return result.0
    }
}
