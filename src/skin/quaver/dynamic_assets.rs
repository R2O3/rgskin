use crate::{common::{alignment::Anchor, skin::{AnimationSpriteType, AssetAttribute}}, def_const_type_enum};

def_const_type_enum!(pub Column => StringPattern {
    LIGHTING => "{keys}k/Lighting/column-lighting" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomLeft)
    ],
});

def_const_type_enum!(pub LaneCover => StringPattern {
    TOP => "{keys}k/LaneCover/cover-top" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomLeft)
    ],
    BOTTOM => "{keys}k/LaneCover/cover-bottom" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
});

def_const_type_enum!(pub Lighting => StringPattern {
    HIT_LIGHTING => "{keys}k/Lighting/hitlighting" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    HOLD_LIGHTING => "{keys}k/Lighting/holdlighting" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
});

def_const_type_enum!(pub Notes => StringPattern {
    HIT_OBJECT => "{keys}k/HitObjects/note-hitobject-{note}" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    HOLD_HIT_OBJECT => "{keys}k/HitObjects/note-holdhitobject-{note}" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    HOLD_BODY => "{keys}k/HitObjects/note-holdbody-{note}" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    HOLD_END => "{keys}k/HitObjects/note-holdend-{note}" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    HIT_OBJECT_SHEET => "{keys}k/HitObjects/note-hitobject-sheet" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    HOLD_OBJECT_SHEET => "{keys}k/HitObjects/note-holdobject-sheet" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
});

def_const_type_enum!(pub Receptors => StringPattern {
    UP => "{keys}k/Receptors/receptor-up-{note}" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    DOWN => "{keys}k/Receptors/receptor-down-{note}" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
});

def_const_type_enum!(pub Stage => StringPattern {
    BG_MASK => "{keys}k/Stage/stage-bgmask" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    DISTANT_OVERLAY => "{keys}k/Stage/stage-distant-overlay" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopCentre)
    ],
    HIT_POSITION_OVERLAY => "{keys}k/Stage/stage-hitposition-overlay" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    LEFT_BORDER => "{keys}k/Stage/stage-left-border" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    RIGHT_BORDER => "{keys}k/Stage/stage-right-border" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopRight)
    ],
});

def_const_type_enum!(pub ComboAlerts => StringPattern {
    ALERT => "Combo/combo-alert-{number}" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
});

def_const_type_enum!(pub Background => StringPattern {
    BACKGROUND => "Backgrounds/{filename}" [AssetAttribute::Texture],
});