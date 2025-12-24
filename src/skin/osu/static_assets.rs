use crate::def_const_type_enum;

def_const_type_enum! (pub Mania => &'static str {
    HIT0 => "mania-hit0",
    HIT50 => "mania-hit50",
    HIT100 => "mania-hit100",
    HIT200 => "mania-hit200",
    HIT300 => "mania-hit300",
    HIT300G => "mania-hit300g",
    COMBOBURST => "comboburst-mania",
    KEY1 => "mania-key1",
    KEY1D => "mania-key1D",
    KEY2 => "mania-key2",
    KEY2D => "mania-key2D",
    KEYS => "mania-keyS",
    KEYSD => "mania-keySD",
    NOTE1 => "mania-note1",
    NOTE2 => "mania-note2",
    NOTES => "mania-noteS",
    NOTE1H => "mania-note1H",
    NOTE2H => "mania-note2H",
    NOTESH => "mania-noteSH",
    NOTE1L => "mania-note1L",
    NOTE2L => "mania-note2L",
    NOTESL => "mania-noteSL",
    NOTE1T => "mania-note1T",
    NOTE2T => "mania-note2T",
    NOTEST => "mania-noteST",
    STAGE_LEFT => "mania-stage-left",
    STAGE_RIGHT => "mania-stage-right",
    STAGE_BOTTOM => "mania-stage-bottom",
    STAGE_LIGHT => "mania-stage-light",
    STAGE_HINT => "mania-stage-hint",
    WARNINGARROW => "mania-warningarrow",
    LIGHTINGL => "lightingL",
    LIGHTINGN => "lightingN",
});

def_const_type_enum! (pub Interface => &'static str {
    STAR => "star",
    STAR2 => "star2",
    SCOREBAR_BG => "scorebar-bg",
    SCOREBAR_COLOUR => "scorebar-colour",
    SCOREBAR_MARKER => "scorebar-marker"
});

def_const_type_enum! (pub Samples => &'static str {
    // Main menu
    HEARTBEAT => "heartbeat",
    SEEYA => "seeya",
    WELCOME => "welcome",
    
    // Keys
    KEY_CONFIRM => "key-confirm",
    KEY_DELETE => "key-delete",
    KEY_MOVEMENT => "key-movement",
    KEY_PRESS_1 => "key-press-1",
    KEY_PRESS_2 => "key-press-2",
    KEY_PRESS_3 => "key-press-3",
    KEY_PRESS_4 => "key-press-4",
    
    // Clicks
    BACK_BUTTON_CLICK => "back-button-click",
    CHECK_ON => "check-on",
    CHECK_OFF => "check-off",
    CLICK_CLOSE => "click-close",
    CLICK_SHORT_CONFIRM => "click-short-confirm",
    MENUBACK => "menuback",
    MENUHIT => "menuhit",
    MENU_BACK_CLICK => "menu-back-click",
    MENU_DIRECT_CLICK => "menu-direct-click",
    MENU_EDIT_CLICK => "menu-edit-click",
    MENU_EXIT_CLICK => "menu-exit-click",
    MENU_FREEPLAY_CLICK => "menu-freeplay-click",
    MENU_MULTIPLAYER_CLICK => "menu-multiplayer-click",
    MENU_OPTIONS_CLICK => "menu-options-click",
    MENU_PLAY_CLICK => "menu-play-click",
    PAUSE_BACK_CLICK => "pause-back-click",
    PAUSE_CONTINUE_CLICK => "pause-continue-click",
    PAUSE_RETRY_CLICK => "pause-retry-click",
    SELECT_EXPAND => "select-expand",
    SELECT_DIFFICULTY => "select-difficulty",
    SHUTTER => "shutter",
    
    // Hover
    BACK_BUTTON_HOVER => "back-button-hover",
    CLICK_SHORT => "click-short",
    MENUCLICK => "menuclick",
    MENU_BACK_HOVER => "menu-back-hover",
    MENU_DIRECT_HOVER => "menu-direct-hover",
    MENU_EDIT_HOVER => "menu-edit-hover",
    MENU_EXIT_HOVER => "menu-exit-hover",
    MENU_FREEPLAY_HOVER => "menu-freeplay-hover",
    MENU_MULTIPLAYER_HOVER => "menu-multiplayer-hover",
    MENU_OPTIONS_HOVER => "menu-options-hover",
    MENU_PLAY_HOVER => "menu-play-hover",
    PAUSE_HOVER => "pause-hover",
    PAUSE_BACK_HOVER => "pause-back-hover",
    PAUSE_CONTINUE_HOVER => "pause-continue-hover",
    PAUSE_RETRY_HOVER => "pause-retry-hover",
    
    // Drag
    SLIDERBAR => "sliderbar",
    WHOOSH => "whoosh",
    
    // Multi
    MATCH_CONFIRM => "match-confirm",
    MATCH_JOIN => "match-join",
    MATCH_LEAVE => "match-leave",
    MATCH_NOTREADY => "match-notready",
    MATCH_READY => "match-ready",
    MATCH_START => "match-start",
    
    // Metronome
    METRONOME_LOW => "metronomelow",
    
    // Countdown
    COUNT => "count",
    COUNT1S => "count1s",
    COUNT2S => "count2s",
    COUNT3S => "count3s",
    GOS => "gos",
    READYS => "readys",
    
    // Playfield
    COMBOBURST => "comboburst",
    COMBOBREAK => "combobreak",
    FAILSOUND => "failsound",
    SECTIONPASS => "sectionpass",
    SECTIONFAIL => "sectionfail",
    
    // Game screens
    APPLAUSE => "applause",
    PAUSE_LOOP => "pause-loop",
    
    // Drum set
    DRUM_HITNORMAL => "drum-hitnormal",
    DRUM_HITCLAP => "drum-hitclap",
    DRUM_HITFINISH => "drum-hitfinish",
    DRUM_HITWHISTLE => "drum-hitwhistle",
    DRUM_SLIDERTICK => "drum-slidertick",
    DRUM_SLIDERSLIDE => "drum-sliderslide",
    DRUM_SLIDERWHISTLE => "drum-sliderwhistle",
    
    // Normal set
    NORMAL_HITNORMAL => "normal-hitnormal",
    NORMAL_HITCLAP => "normal-hitclap",
    NORMAL_HITFINISH => "normal-hitfinish",
    NORMAL_HITWHISTLE => "normal-hitwhistle",
    NORMAL_SLIDERTICK => "normal-slidertick",
    NORMAL_SLIDERSLIDE => "normal-sliderslide",
    NORMAL_SLIDERWHISTLE => "normal-sliderwhistle",
    
    // Soft set
    SOFT_HITNORMAL => "soft-hitnormal",
    SOFT_HITCLAP => "soft-hitclap",
    SOFT_HITFINISH => "soft-hitfinish",
    SOFT_HITWHISTLE => "soft-hitwhistle",
    SOFT_SLIDERTICK => "soft-slidertick",
    SOFT_SLIDERSLIDE => "soft-sliderslide",
    SOFT_SLIDERWHISTLE => "soft-sliderwhistle",
    
    // Spinner set
    SPINNERSPIN => "spinnerspin",
    SPINNERBONUS => "spinnerbonus",
    SPINNERBONUS_MAX => "spinnerbonus-max",
    
    // Nightcore mod
    NIGHTCORE_KICK => "nightcore-kick",
    NIGHTCORE_CLAP => "nightcore-clap",
    NIGHTCORE_HAT => "nightcore-hat",
    NIGHTCORE_FINISH => "nightcore-finish",
    
    // taiko Normal set
    TAIKO_NORMAL_HITNORMAL => "taiko-normal-hitnormal",
    TAIKO_NORMAL_HITCLAP => "taiko-normal-hitclap",
    TAIKO_NORMAL_HITFINISH => "taiko-normal-hitfinish",
    TAIKO_NORMAL_HITWHISTLE => "taiko-normal-hitwhistle",
    
    // taiko Soft set
    TAIKO_SOFT_HITNORMAL => "taiko-soft-hitnormal",
    TAIKO_SOFT_HITCLAP => "taiko-soft-hitclap",
    TAIKO_SOFT_HITFINISH => "taiko-soft-hitfinish",
    TAIKO_SOFT_HITWHISTLE => "taiko-soft-hitwhistle",
    
    // taiko Drum set
    TAIKO_DRUM_HITNORMAL => "taiko-drum-hitnormal",
    TAIKO_DRUM_HITCLAP => "taiko-drum-hitclap",
    TAIKO_DRUM_HITFINISH => "taiko-drum-hitfinish",
    TAIKO_DRUM_HITWHISTLE => "taiko-drum-hitwhistle"
});