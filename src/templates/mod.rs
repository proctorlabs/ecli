use crate::*;
use serde::Serialize;
use std::fmt;
use templar::{Context, SharedContext, Templar, Template, TemplateTree};

mod ser;

lazy_static! {
    pub static ref TEMPLAR: Templar = { Templar::default() };
    pub static ref CONTEXT: SharedContext = { SharedContext::default() };
}

#[derive(Debug, Clone)]
pub struct EcliExpression(Template, String);

impl fmt::Display for EcliExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl EcliExpression {
    pub fn exec(&self) -> Result<Document> {
        self.0
            .exec(&*CONTEXT)
            .map_err(|e| AppError::Info(e.to_string()))
    }

    pub fn render(&self) -> Result<String> {
        self.0
            .render(&*CONTEXT)
            .map_err(|e| AppError::Info(e.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct EcliTemplate(Template, String);

impl fmt::Display for EcliTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl EcliTemplate {
    pub fn exec(&self) -> Result<Document> {
        self.0
            .exec(&*CONTEXT)
            .map_err(|e| AppError::Info(e.to_string()))
    }

    pub fn render(&self) -> Result<String> {
        self.0
            .render(&*CONTEXT)
            .map_err(|e| AppError::Info(e.to_string()))
    }
}

pub fn context_set<T: Serialize + ?Sized>(key: templar::Document, val: &T) -> Result<()> {
    let v = templar::Document::new(val).map_err(|e| AppError::Fatal(format!("{}", e)))?;
    CONTEXT.set_path(&[&key], v)?;
    Ok(())
}

pub fn context_set_yaml(doc: &templar::Document) -> Result<()> {
    let t: TemplateTree = TEMPLAR.parse(doc)?;
    CONTEXT.set(t)?;
    Ok(())
}
