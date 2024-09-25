// use std::borrow::Cow;

// use poem::{http::HeaderValue, web::Field};
// use rust_decimal::Decimal;
// use serde_json::Value;

// use crate::{
//     registry::{MetaSchema, MetaSchemaRef},
//     types::{
//         ParseError, ParseFromJSON, ParseFromMultipartField, ParseFromParameter, ParseResult,
//         ToHeader, ToJSON, Type,
//     },
// };

// impl Type for Decimal {
//     const IS_REQUIRED: bool = true;

//     type RawValueType = Self;

//     type RawElementValueType = Self;

//     fn name() -> Cow<'static, str> {
//         "string(decimal)".into()
//     }

//     fn schema_ref() -> MetaSchemaRef {
//         MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("string", "decimal")))
//     }

//     fn as_raw_value(&self) -> Option<&Self::RawValueType> {
//         Some(self)
//     }

//     fn raw_element_iter<'a>(
//         &'a self,
//     ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
//         Box::new(self.as_raw_value().into_iter())
//     }
// }

// impl ParseFromJSON for Decimal {
//     fn parse_from_json(value: Option<Value>) -> ParseResult<Self> {
//         let value = value.unwrap_or_default();
//         match value {
//             Value::String(value) => Ok(value.parse()?),
//             Value::Number(num) if num.is_i64() => Ok(Decimal::from(
//                 num.as_i64()
//                     .ok_or_else(|| ParseError::custom("Expected a number"))?,
//             )),
//             Value::Number(num) if num.is_u64() => Ok(Decimal::from(
//                 num.as_u64()
//                     .ok_or_else(|| ParseError::custom("Expected a number"))?,
//             )),
//             Value::Number(num) if num.is_f64() => Ok(Decimal::try_from(
//                 num.as_f64()
//                     .ok_or_else(|| ParseError::custom("Expected a float"))?,
//             )
//             .map_err(|_| ParseError::custom("Float out of range"))?),
//             _ => Err(ParseError::expected_type(value)),
//         }
//     }
// }

// impl ParseFromParameter for Decimal {
//     fn parse_from_parameter(value: &str) -> ParseResult<Self> {
//         value.parse().map_err(ParseError::custom)
//     }
// }

// impl ParseFromMultipartField for Decimal {
//     async fn parse_from_multipart(field: Option<Field>) -> ParseResult<Self> {
//         match field {
//             Some(field) => Ok(field.text().await?.parse()?),
//             None => Err(ParseError::expected_input()),
//         }
//     }
// }

// impl ToJSON for Decimal {
//     fn to_json(&self) -> Option<Value> {
//         Some(Value::String(self.normalize().to_string()))
//     }
// }

// impl ToHeader for Decimal {
//     fn to_header(&self) -> Option<HeaderValue> {
//         HeaderValue::from_str(&self.normalize().to_string()).ok()
//     }
// }

use std::borrow::Cow;

use poem::{http::HeaderValue, web::Field};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde_json::Value;

use crate::{
    registry::{MetaSchema, MetaSchemaRef},
    types::{
        ParseError, ParseFromJSON, ParseFromMultipartField, ParseFromParameter, ParseResult,
        ToHeader, ToJSON, Type,
    },
};

impl Type for Decimal {
    const IS_REQUIRED: bool = true;

    type RawValueType = Self;

    type RawElementValueType = Self;

    fn name() -> Cow<'static, str> {
        // Updated to reflect a numeric type
        "number(decimal)".into()
    }

    fn schema_ref() -> MetaSchemaRef {
        // Changed the type from "string" to "number"
        MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("number", "decimal")))
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}

impl ParseFromJSON for Decimal {
    fn parse_from_json(value: Option<Value>) -> ParseResult<Self> {
        let value = value.unwrap_or_default();
        match value {
            Value::Number(num) => {
                // Attempt to parse from i64, u64, or f64
                if let Some(i) = num.as_i64() {
                    Ok(Decimal::from(i))
                } else if let Some(u) = num.as_u64() {
                    Ok(Decimal::from(u))
                } else if let Some(f) = num.as_f64() {
                    Decimal::try_from(f).map_err(|_| ParseError::custom("Float out of range"))
                } else {
                    Err(ParseError::custom("Invalid number"))
                }
            }
            Value::String(value) => {
                // Optionally support parsing from strings
                Ok(value.parse()?)
            }
            _ => Err(ParseError::expected_type(value)),
        }
    }
}

impl ParseFromParameter for Decimal {
    fn parse_from_parameter(value: &str) -> ParseResult<Self> {
        value.parse().map_err(ParseError::custom)
    }
}

impl ParseFromMultipartField for Decimal {
    async fn parse_from_multipart(field: Option<Field>) -> ParseResult<Self> {
        match field {
            Some(field) => Ok(field.text().await?.parse()?),
            None => Err(ParseError::expected_input()),
        }
    }
}

impl ToJSON for Decimal {
    fn to_json(&self) -> Option<Value> {
        // Convert Decimal to f64, then to serde_json::Number
        self.to_f64()
            .and_then(serde_json::Number::from_f64)
            .map(Value::Number)
    }
}

impl ToHeader for Decimal {
    fn to_header(&self) -> Option<HeaderValue> {
        // Headers typically require string representations
        HeaderValue::from_str(&self.normalize().to_string()).ok()
    }
}
