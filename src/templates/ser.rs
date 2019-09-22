use super::{EcliExpression, EcliTemplate, TEMPLAR};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use templar::Template;

impl<'de> Deserialize<'de> for EcliExpression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExpressionVisitor;

        impl<'de> Visitor<'de> for ExpressionVisitor {
            type Value = EcliExpression;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<EcliExpression, E>
            where
                E: de::Error,
            {
                let template: Template = TEMPLAR.parse_expression(value).unwrap_or_default();
                Ok(EcliExpression(template, value.into()))
            }
        }

        deserializer.deserialize_str(ExpressionVisitor)
    }
}

impl Serialize for EcliExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.1)
    }
}

impl<'de> Deserialize<'de> for EcliTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExpressionVisitor;

        impl<'de> Visitor<'de> for ExpressionVisitor {
            type Value = EcliTemplate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<EcliTemplate, E>
            where
                E: de::Error,
            {
                let template: Template = TEMPLAR.parse(value).unwrap_or_default();
                Ok(EcliTemplate(template, value.into()))
            }
        }

        deserializer.deserialize_str(ExpressionVisitor)
    }
}

impl Serialize for EcliTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.1)
    }
}

impl From<&str> for EcliTemplate {
    fn from(s: &str) -> EcliTemplate {
        let template: Template = TEMPLAR.parse(s).unwrap_or_default();
        EcliTemplate(template, s.into())
    }
}
