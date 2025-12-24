#[derive(Clone)]
pub struct Sounds {
    pub ui: UISounds,
    pub gameplay: GenericGameplaySounds,
    pub mania: ManiaGameplaySounds
}

#[derive(Clone)]
pub struct UISounds {
    pub menu_back_click: Option<String>,
    pub ui_click: Option<String>,
    pub ui_select: Option<String>,
    pub ui_hover: Option<String>
}

#[derive(Clone)]
pub struct ManiaGameplaySounds {
    pub hit: Option<String>,
}

#[derive(Clone)]
pub struct GenericGameplaySounds {
    pub miss: Option<String>,
    pub fail: Option<String>,
    pub restart: Option<String>
}