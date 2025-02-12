use anyhow::anyhow;
use gpui::{AssetSource, Result, SharedString};
use rust_embed::Embed;
use std::borrow::Cow;

#[derive(Embed)]
#[folder = "../../assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[include = "images/**/*"]
#[include = "themes/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        // Self::get(path)
        //     .map(|f| Some(f.data))
        //     .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))

        Self::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
            .map(Some)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        // Ok(Self::iter()
        //     .filter_map(|p| {
        //         if p.starts_with(path) {
        //             Some(p.into())
        //         } else {
        //             None
        //         }
        //     })
        //     .collect())

        Ok(Self::iter()
            .filter(|p| p.starts_with(path))
            .map(SharedString::from)
            .collect())
    }
}
