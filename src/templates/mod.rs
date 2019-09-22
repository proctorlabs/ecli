use crate::*;
pub use ser::{EcliExpression, EcliTemplate};
use serde::Serialize;
use templar::{Context, SharedContext, Templar, TemplateTree};

mod ser;

lazy_static! {
    pub static ref TEMPLAR: Templar = { Templar::default() };
    pub static ref CONTEXT: SharedContext = {
        let context = SharedContext::default();
        let mut init = Document::default();
        init["ecli"]["version"] = crate_version!().into();
        context.set(init).unwrap_or_default();
        context
    };
}

pub fn context_set<T: Serialize + ?Sized>(key: templar::Document, val: &T) -> Result<()> {
    let v = templar::Document::new(val).map_err(|e| AppError::Fatal(format!("{}", e)))?;
    CONTEXT.set_path(&[&key], v)?;
    Ok(())
}

pub fn context_set_value(doc: &templar::Document) -> Result<()> {
    let t: TemplateTree = TEMPLAR.parse(doc)?;
    CONTEXT.set(t)?;
    Ok(())
}
