use crate::currency::{convert_to, Currency};
use crate::currency::PLN;

pub trait RateSource {
    fn get_rate(&mut self, currency: &Currency) -> Option<f32>;
}

#[derive(Debug)]
pub enum ConversionError {
    RateUnavailable,
}

pub fn convert(pln: &PLN, rate_source: &mut impl RateSource, currency: Currency) -> Result<f32, ConversionError> {
    let rate = rate_source.get_rate(&currency);
    match rate {
        Some(r) => Ok(convert_to(pln, r)),
        None => {
            println!("Unable to convert PLN to {}: rate is not available.", currency.code);
            Err(ConversionError::RateUnavailable)
        },
    }
}