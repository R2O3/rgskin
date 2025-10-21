use crate::converting::fluxis::{from_generic_mania, to_generic_mania};
use crate::skin::generic::GenericManiaSkin;
use crate::skin::fluxis::{FluXisLayout, SkinJson};
use crate::io::TextureStore;

pub struct FluXisSkin {
    pub skin_json: SkinJson,
    pub textures: TextureStore
}

impl FluXisSkin {
    pub fn new(skin_json: SkinJson, textures: Option<TextureStore>) -> Self {
        Self { skin_json, textures: textures.unwrap_or(TextureStore::new()) }
    }

    pub fn to_generic_mania(self, layout: Option<FluXisLayout>) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
        to_generic_mania(self, layout)
    }

    pub fn from_generic_mania(skin: GenericManiaSkin) -> Result<(FluXisSkin, FluXisLayout), Box<dyn std::error::Error>> {
        from_generic_mania(skin)
    }
}
