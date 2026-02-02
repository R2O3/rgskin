use crate::def_const_type_enum;

def_const_type_enum! (pub Other => &'static str {
    ICON => "icon",
});

def_const_type_enum! (pub Judgement => &'static str {
    MISS => "Judgement/miss",
    OKAY => "Judgement/okay",
    ALRIGHT => "Judgement/alright",
    GREAT => "Judgement/great",
    PERFECT => "Judgement/perfect",
    FLAWLESS => "Judgement/flawless",
});

def_const_type_enum! (pub Stage => &'static str {
    BACKGROUND => "Stage/background",
    BACKGROUND_TOP => "Stage/background-top",
    BACKGROUND_BOTTOM => "Stage/background-bottom",
    BORDER_LEFT => "Stage/border-left",
    BORDER_LEFT_TOP => "Stage/border-left-top",
    BORDER_LEFT_BOTTOM => "Stage/border-left-bottom",
    BORDER_RIGHT => "Stage/border-right",
    BORDER_RIGHT_TOP => "Stage/border-right-top",
    BORDER_RIGHT_BOTTOM => "Stage/border-right-bottom",
    LANE_COVER_TOP => "Stage/lane-cover-top",
    LANE_COVER_BOTTOM => "Stage/lane-cover-bottom",
    HITLINE => "Stage/hitline",
});

def_const_type_enum! (pub UserInterface => &'static str {
    BACKGROUND => "UserInterface/background",
});

def_const_type_enum! (pub Health => &'static str {
    BACKGROUND => "Health/background",
    FOREGROUND => "Health/foreground",
});

def_const_type_enum! (pub Lighting => &'static str {
    COLUMN_LIGHTING => "Lighting/column-lighting",
});

def_const_type_enum! (pub Gameplay => &'static str {
    FAIL_FLASH => "Gameplay/fail-flash",
});

def_const_type_enum! (pub Results => &'static str {
    RANK_X => "Results/rank-x",
    RANK_SS => "Results/rank-ss",
    RANK_S => "Results/rank-s",
    RANK_AA => "Results/rank-aa",
    RANK_A => "Results/rank-a",
    RANK_B => "Results/rank-b",
    RANK_C => "Results/rank-c",
    RANK_D => "Results/rank-d",
});

def_const_type_enum! (pub Samples => &'static str {
    UI_BACK => "Samples/UI/back",
    UI_SELECT => "Samples/UI/select",
    UI_HOVER => "Samples/UI/hover",
    UI_CLICK => "Samples/UI/click",
    UI_CLICK_DISABLED => "Samples/UI/click-disabled",
    UI_SKIN_SELECT_CLICK => "Samples/UI/skin-select-click",
    
    COURSE_CONFIRM => "Samples/Course/confirm",
    COURSE_COMPLETE => "Samples/Course/complete",
    COURSE_FAILED => "Samples/Course/failed",
    
    GAMEPLAY_HIT => "Samples/Gameplay/hit",
    GAMEPLAY_MISS => "Samples/Gameplay/miss",
    GAMEPLAY_FAIL => "Samples/Gameplay/fail",
    GAMEPLAY_RESTART => "Samples/Gameplay/restart",
    GAMEPLAY_FULL_COMBO => "Samples/Gameplay/full-combo",
    GAMEPLAY_ALL_FLAWLESS => "Samples/Gameplay/all-flawless"
});