use crate::common::traits::ManiaSkin;
use crate::common::vector::Vector2;
use crate::converting::fluxis::{from_generic_mania, to_generic_mania};
use crate::fluxis::skin_json::Keymode;
use crate::skin::generic::GenericManiaSkin;
use crate::skin::fluxis::{FluXisLayout, SkinJson};
use crate::io::texture::TextureStore;
use crate::utils::fluxis::FluXisDimensions;

pub struct FluXisSkin {
    pub resolution: Vector2<u32>,
    pub skin_json: SkinJson,
    pub textures: TextureStore
}

impl FluXisSkin {
    pub fn new(skin_json: SkinJson, textures: Option<TextureStore>) -> Self {
        Self { skin_json,
            textures: textures.unwrap_or(TextureStore::new()),
            resolution: Vector2::new(FluXisDimensions::X.as_u32(), FluXisDimensions::Y.as_u32())
        }
    }
}

impl ManiaSkin for FluXisSkin {
    type Keymode = Keymode;
    type ToParams = Option<FluXisLayout>;
    type FromReturn = (FluXisSkin, FluXisLayout);

    fn to_generic_mania(self, params: Self::ToParams) -> Result<GenericManiaSkin, Box<dyn std::error::Error>> {
        to_generic_mania(self, params)
    }

    fn from_generic_mania(skin: GenericManiaSkin) -> Result<Self::FromReturn, Box<dyn std::error::Error>> {
        from_generic_mania(skin)
    }

    fn get_keymode(&self, keymode: u8) -> Option<&Keymode> {
        for k in &self.skin_json.keymodes {
            if k.keymode == keymode { return Some(k); }
        }
        None
    }
}
