use std::fmt;

use super::modifier::Modifier;

#[derive(Default)]
pub struct Modifiers {
    #[allow(dead_code)]
    inner: std::collections::HashMap<String, String>,
}

impl fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Modifiers {:?}", self.inner)
    }
}

impl Modifiers {
    pub fn try_from_slice<M>(modifiers: M) -> Result<Self, Modifier>
    where
        M: AsRef<[Modifier]>,
    {
        let mut inner = std::collections::HashMap::new();

        for modifier in modifiers.as_ref() {
            if inner
                .insert(modifier.name.to_string(), modifier.value.to_string())
                .is_some()
            {
                return Err(modifier.clone());
            }
        }

        Ok(Self { inner })
    }
}
