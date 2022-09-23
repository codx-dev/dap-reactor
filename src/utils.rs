use core::iter;
use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::error::*;

pub fn get_map_to_string_optional(
    map: &Map<String, Value>,
    attribute: &'static str,
) -> Result<Option<HashMap<String, String>>, Error> {
    map.get(attribute)
        .map(|x| {
            x.as_object()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeObject))
        })
        .transpose()?
        .map(|m| {
            m.iter()
                .map(|(k, v)| {
                    v.as_str()
                        .ok_or_else(|| Error::new(attribute, Cause::MustBeString))
                        .map(|s| (k.clone(), s.to_string()))
                })
                .collect()
        })
        .transpose()
}

pub fn get_map_to_string_or_null_optional(
    map: &Map<String, Value>,
    attribute: &'static str,
) -> Result<Option<HashMap<String, Option<String>>>, Error> {
    map.get(attribute)
        .map(|x| {
            x.as_object()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeObject))
        })
        .transpose()?
        .map(|m| {
            m.iter()
                .map(|(k, v)| {
                    let v = match v {
                        Value::Null => None,
                        Value::String(s) => Some(s.to_string()),
                        _ => return Err(Error::new(attribute, Cause::MustMapToStringOrNull)),
                    };

                    Ok((k.clone(), v))
                })
                .collect()
        })
        .transpose()
}

pub fn get_optional(map: &Map<String, Value>, attribute: &'static str) -> Option<Value> {
    map.get(attribute).filter(|v| !v.is_null()).cloned()
}

pub fn get_object<'a, T>(map: &'a Map<String, Value>, attribute: &'static str) -> Result<T, Error>
where
    T: TryFrom<&'a Map<String, Value>, Error = Error>,
{
    map.get(attribute)
        .ok_or_else(|| Error::new(attribute, Cause::IsMandatory))?
        .as_object()
        .ok_or_else(|| Error::new(attribute, Cause::MustBeObject))
        .and_then(T::try_from)
}

pub fn get_object_optional<'a, T>(
    map: &'a Map<String, Value>,
    attribute: &'static str,
) -> Result<Option<T>, Error>
where
    T: TryFrom<&'a Map<String, Value>, Error = Error>,
{
    map.get(attribute)
        .map(|x| {
            x.as_object()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeObject))
                .and_then(T::try_from)
        })
        .transpose()
}

pub fn get_array_optional<'a, T>(
    map: &'a Map<String, Value>,
    attribute: &'static str,
) -> Result<Vec<T>, Error>
where
    T: TryFrom<&'a Map<String, Value>, Error = Error>,
{
    let array = map
        .get(attribute)
        .map(|x| {
            x.as_array()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeArray))
        })
        .transpose()?
        .map(|a| {
            a.iter()
                .map(|x| {
                    x.as_object()
                        .ok_or_else(|| Error::new(attribute, Cause::MustBeObject))
                        .and_then(T::try_from)
                })
                .collect()
        })
        .transpose()?
        .unwrap_or_default();

    Ok(array)
}

pub fn get_array_of_string(
    map: &Map<String, Value>,
    attribute: &'static str,
) -> Result<Vec<String>, Error> {
    map.get(attribute)
        .ok_or_else(|| Error::new(attribute, Cause::IsMandatory))?
        .as_array()
        .ok_or_else(|| Error::new(attribute, Cause::MustBeArray))
        .and_then(|a| {
            a.iter()
                .map(|x| {
                    x.as_str()
                        .ok_or_else(|| Error::new(attribute, Cause::MustBeString))
                        .map(|s| s.to_string())
                })
                .collect()
        })
}

pub fn get_array_of_string_optional(
    map: &Map<String, Value>,
    attribute: &'static str,
) -> Result<Vec<String>, Error> {
    Ok(map
        .get(attribute)
        .map(|x| {
            x.as_array()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeArray))
                .and_then(|a| {
                    a.iter()
                        .map(|x| {
                            x.as_str()
                                .ok_or_else(|| Error::new(attribute, Cause::MustBeString))
                                .map(|s| s.to_string())
                        })
                        .collect()
                })
        })
        .transpose()?
        .unwrap_or_default())
}

pub fn get_array_of_string_enum_optional<'a, T>(
    map: &'a Map<String, Value>,
    attribute: &'static str,
) -> Result<Vec<T>, Error>
where
    T: TryFrom<&'a str, Error = Error>,
{
    Ok(map
        .get(attribute)
        .map(|x| {
            x.as_array()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeArray))
                .and_then(|a| {
                    a.iter()
                        .map(|x| {
                            x.as_str()
                                .ok_or_else(|| Error::new(attribute, Cause::MustBeString))
                                .and_then(T::try_from)
                        })
                        .collect()
                })
        })
        .transpose()?
        .unwrap_or_default())
}

pub fn get_array_usize_optional(
    map: &Map<String, Value>,
    attribute: &'static str,
) -> Result<Vec<usize>, Error> {
    map.get(attribute)
        .map(|x| {
            x.as_array()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeArray))
        })
        .transpose()?
        .map(|a| {
            a.iter()
                .map(|x| {
                    x.as_u64()
                        .map(|n| n as usize)
                        .ok_or_else(|| Error::new(attribute, Cause::MustBeUnsignedInteger))
                })
                .collect()
        })
        .transpose()
        .map(|v| v.unwrap_or_default())
}

pub fn get_str_optional<'a>(
    map: &'a Map<String, Value>,
    attribute: &'static str,
) -> Result<Option<&'a str>, Error> {
    map.get(attribute)
        .map(|x| {
            x.as_str()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeString))
        })
        .transpose()
}

pub fn get_str<'a>(map: &'a Map<String, Value>, attribute: &'static str) -> Result<&'a str, Error> {
    get_str_optional(map, attribute)?.ok_or_else(|| Error::new(attribute, Cause::IsMandatory))
}

pub fn get_string_optional(
    map: &Map<String, Value>,
    attribute: &'static str,
) -> Result<Option<String>, Error> {
    get_str_optional(map, attribute).map(|x| x.map(|s| s.to_string()))
}

pub fn get_string(map: &Map<String, Value>, attribute: &'static str) -> Result<String, Error> {
    get_string_optional(map, attribute)?.ok_or_else(|| Error::new(attribute, Cause::IsMandatory))
}

pub fn get_bool_optional(map: &Map<String, Value>, attribute: &'static str) -> Result<bool, Error> {
    map.get(attribute)
        .map(|x| {
            x.as_bool()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeBoolean))
        })
        .transpose()
        .map(|x| x.unwrap_or(false))
}

pub fn get_bool(map: &Map<String, Value>, attribute: &'static str) -> Result<bool, Error> {
    map.get(attribute)
        .ok_or_else(|| Error::new(attribute, Cause::IsMandatory))
        .and_then(|x| {
            x.as_bool()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeBoolean))
        })
}

pub fn get_u64_optional(
    map: &Map<String, Value>,
    attribute: &'static str,
) -> Result<Option<u64>, Error> {
    map.get(attribute)
        .map(|x| {
            x.as_u64()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeUnsignedInteger))
        })
        .transpose()
}

pub fn get_i64_optional(
    map: &Map<String, Value>,
    attribute: &'static str,
) -> Result<Option<i64>, Error> {
    map.get(attribute)
        .map(|x| {
            x.as_i64()
                .ok_or_else(|| Error::new(attribute, Cause::MustBeUnsignedInteger))
        })
        .transpose()
}

pub fn get_u64(map: &Map<String, Value>, attribute: &'static str) -> Result<u64, Error> {
    get_u64_optional(map, attribute)?.ok_or_else(|| Error::new(attribute, Cause::IsMandatory))
}

pub fn get_u32_optional(
    map: &Map<String, Value>,
    attribute: &'static str,
) -> Result<Option<u32>, Error> {
    get_u64_optional(map, attribute).map(|x| x.map(|n| n as u32))
}

pub fn attribute<T>(attribute: &'static str, v: T) -> impl Iterator<Item = Option<(String, Value)>>
where
    T: Into<Value>,
{
    iter::once(Some((attribute.to_string(), v.into())))
}

pub fn attribute_optional<T>(
    attribute: &'static str,
    v: Option<T>,
) -> impl Iterator<Item = Option<(String, Value)>>
where
    T: Into<Value>,
{
    iter::once(
        v.map(|v| v.into())
            .filter(|v| !v.is_null())
            .map(|v| (attribute.to_string(), v)),
    )
}

pub fn attribute_map_optional<M, T>(
    attribute: &'static str,
    map: Option<M>,
) -> impl Iterator<Item = Option<(String, Value)>>
where
    M: IntoIterator<Item = (String, T)>,
    T: Into<Value>,
{
    iter::once(
        map.map(|m| {
            m.into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect::<Map<String, Value>>()
        })
        .filter(|m| !m.is_empty())
        .map(|m| (attribute.to_string(), Value::Object(m))),
    )
}

pub fn attribute_string_optional<T>(
    attribute: &'static str,
    s: Option<T>,
) -> impl Iterator<Item = Option<(String, Value)>>
where
    T: Into<String>,
{
    iter::once(s.map(|s| (attribute.to_string(), Value::String(s.into()))))
}

pub fn attribute_string<T>(
    attribute: &'static str,
    s: T,
) -> impl Iterator<Item = Option<(String, Value)>>
where
    T: Into<String>,
{
    iter::once(Some((attribute.to_string(), Value::String(s.into()))))
}

pub fn attribute_u32_optional(
    attribute: &'static str,
    n: Option<u32>,
) -> impl Iterator<Item = Option<(String, Value)>> {
    iter::once(n.map(|n| (attribute.to_string(), Value::Number(n.into()))))
}

pub fn attribute_u64_optional(
    attribute: &'static str,
    n: Option<u64>,
) -> impl Iterator<Item = Option<(String, Value)>> {
    iter::once(n.map(|n| (attribute.to_string(), Value::Number(n.into()))))
}

pub fn attribute_u64(
    attribute: &'static str,
    n: u64,
) -> impl Iterator<Item = Option<(String, Value)>> {
    iter::once(Some((attribute.to_string(), Value::Number(n.into()))))
}

pub fn attribute_i64_optional(
    attribute: &'static str,
    n: Option<i64>,
) -> impl Iterator<Item = Option<(String, Value)>> {
    iter::once(n.map(|n| (attribute.to_string(), Value::Number(n.into()))))
}

pub fn attribute_array_optional<I, T>(
    attribute: &'static str,
    a: I,
) -> impl Iterator<Item = Option<(String, Value)>>
where
    I: IntoIterator<Item = T>,
    T: Into<Value>,
{
    let a = a.into_iter().map(|t| t.into()).collect::<Vec<_>>();

    iter::once((!a.is_empty()).then_some((attribute.to_string(), Value::Array(a))))
}

pub fn attribute_array_of_string_optional<I, T>(
    attribute: &'static str,
    a: Option<I>,
) -> impl Iterator<Item = Option<(String, Value)>>
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    iter::once(
        a.map(|m| {
            m.into_iter()
                .map(|v| Value::from(v.into()))
                .collect::<Vec<Value>>()
        })
        .filter(|m| !m.is_empty())
        .map(|m| (attribute.to_string(), Value::Array(m))),
    )
}

pub fn attribute_array<I, T>(
    attribute: &'static str,
    a: I,
) -> impl Iterator<Item = Option<(String, Value)>>
where
    I: IntoIterator<Item = T>,
    T: Into<Value>,
{
    let a = a.into_iter().map(|t| t.into()).collect::<Vec<_>>();

    iter::once(Some((attribute.to_string(), Value::Array(a))))
}

pub fn attribute_bool(
    attribute: &'static str,
    b: bool,
) -> impl Iterator<Item = Option<(String, Value)>> {
    iter::once(Some((attribute.to_string(), Value::Bool(b))))
}

pub fn attribute_bool_optional(
    attribute: &'static str,
    b: bool,
) -> impl Iterator<Item = Option<(String, Value)>> {
    iter::once(b.then_some((attribute.to_string(), Value::Bool(true))))
}

pub fn finalize_object<I>(iter: I) -> Value
where
    I: Iterator<Item = Option<(String, Value)>>,
{
    Value::Object(iter.flatten().collect())
}
