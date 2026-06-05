use crate::{common::{alignment::Anchor, skin::{AnimationSpriteType, AssetAttribute}}, def_const_type_enum};

def_const_type_enum!(pub Grades => StringPattern {
    SMALL_A => "Grades/grade-small-a" [AssetAttribute::Texture],
    SMALL_B => "Grades/grade-small-b" [AssetAttribute::Texture],
    SMALL_C => "Grades/grade-small-c" [AssetAttribute::Texture],
    SMALL_D => "Grades/grade-small-d" [AssetAttribute::Texture],
    SMALL_F => "Grades/grade-small-f" [AssetAttribute::Texture],
    SMALL_S => "Grades/grade-small-s" [AssetAttribute::Texture],
    SMALL_SS => "Grades/grade-small-ss" [AssetAttribute::Texture],
    SMALL_X => "Grades/grade-small-x" [AssetAttribute::Texture],
});

def_const_type_enum!(pub HealthBar => StringPattern {
    BACKGROUND => "Health/health-background" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet)
    ],
    FOREGROUND => "Health/health-foreground" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet)
    ],
});

def_const_type_enum!(pub HitBubbles => StringPattern {
    BACKGROUND => "HitBubbles/bubbles-background" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet)
    ],
    BUBBLE => "HitBubbles/bubble" [AssetAttribute::Texture],
});

def_const_type_enum!(pub Judgements => StringPattern {
    OVERLAY => "Judgements/judgement-overlay" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    OVERLAY_BACKGROUND => "Judgements/judgement-overlay-background-{judgement}" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    MARV => "Judgements/judge-marv" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    PERF => "Judgements/judge-perf" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    GREAT => "Judgements/judge-great" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    GOOD => "Judgements/judge-good" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    OKAY => "Judgements/judge-okay" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    MISS => "Judgements/judge-miss" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
});

def_const_type_enum!(pub Numbers => StringPattern {
    COMBO => "Numbers/combo-{number}" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    SCORE => "Numbers/score-{number}" [AssetAttribute::Texture],
    SCORE_PERCENT => "Numbers/score-percent" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopRight)
    ],
    SCORE_DECIMAL => "Numbers/score-decimal" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopRight)
    ],
    SONG_TIME => "Numbers/song-time-{number}" [AssetAttribute::Texture],
    SONG_TIME_COLON => "Numbers/song-time-colon" [AssetAttribute::Texture],
    SONG_TIME_MINUS => "Numbers/song-time-minus" [AssetAttribute::Texture],
});

def_const_type_enum!(pub Scoreboard => StringPattern {
    USER => "Scoreboard/scoreboard" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    OTHER => "Scoreboard/scoreboard-other" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    RED_TEAM => "Scoreboard/scoreboard-red-team" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    RED_TEAM_OTHER => "Scoreboard/scoreboard-red-team-other" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    BLUE_TEAM => "Scoreboard/scoreboard-blue-team" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    BLUE_TEAM_OTHER => "Scoreboard/scoreboard-blue-team-other" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
});

def_const_type_enum!(pub SkipDisplay => StringPattern {
    SKIP => "Skip/skip" [
        AssetAttribute::Texture, 
        AssetAttribute::Animatable(AnimationSpriteType::SpriteSheet), 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
});

def_const_type_enum!(pub PauseScreen => StringPattern {
    CONTINUE => "Pause/pause-continue" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    RETRY => "Pause/pause-retry" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    BACK => "Pause/pause-back" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    BACKGROUND => "Pause/pause-background" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
});

def_const_type_enum!(pub BattleRoyale => StringPattern {
    ELIMINATED => "Multiplayer/eliminated" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    WARNING => "Multiplayer/warning" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
});

def_const_type_enum!(pub Cursor => StringPattern {
    MAIN_CURSOR => "Cursor/main-cursor" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
});

def_const_type_enum!(pub MainMenu => StringPattern {
    MENU_BACKGROUND => "MainMenu/menu-background" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    LOGO_BACKGROUND => "MainMenu/logo-background" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    NAVIGATION_BUTTON => "MainMenu/navigation-button" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    NAVIGATION_BUTTON_SELECTED => "MainMenu/navigation-button-selected" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    NAVIGATION_BUTTON_HOVERED => "MainMenu/navigation-button-hovered" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    TIP_PANEL => "MainMenu/tip-panel" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomRight)
    ],
    NEWS_PANEL => "MainMenu/news-panel" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomRight)
    ],
    JUKEBOX_OVERLAY => "MainMenu/jukebox-overlay" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    NOTE_VISUALIZER => "MainMenu/note-visualizer" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
});

def_const_type_enum!(pub MenuBorder => StringPattern {
    MENU_BORDER_BACKGROUND => "MenuBorder/menu-border-background" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopCentre)
    ],
    MENU_BORDER_BACKGROUND_FOOTER => "MenuBorder/menu-border-background-footer" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
});

def_const_type_enum!(pub SongSelect => StringPattern {
    MAPSET_DESELECTED => "SongSelect/mapset-deselected" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    MAPSET_SELECTED => "SongSelect/mapset-selected" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    MAPSET_HOVERED => "SongSelect/mapset-hovered" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    STATUS_RANKED => "SongSelect/status-ranked" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    STATUS_UNRANKED => "SongSelect/status-unranked" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    STATUS_NOTSUBMITTED => "SongSelect/status-notsubmitted" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    STATUS_OSU => "SongSelect/status-osu" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    STATUS_SM => "SongSelect/status-sm" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    STATUS_VARIOUS => "SongSelect/status-various" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    GAME_MODE_4K => "SongSelect/game-mode-4k" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    GAME_MODE_7K => "SongSelect/game-mode-7k" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    GAME_MODE_4K7K => "SongSelect/game-mode-4k7k" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreRight)
    ],
    LEADERBOARD_PANEL => "SongSelect/leaderboard-panel" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    PERSONALBEST_PANEL => "SongSelect/personalbest-panel" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::CentreLeft)
    ],
    SELECT_FILTER_PANEL_LEFT => "SongSelect/select-filter-panel-left" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    SELECT_FILTER_PANEL_RIGHT => "SongSelect/select-filter-panel-right" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
});

def_const_type_enum!(pub Results => StringPattern {
    AVATAR_BORDER => "Results/avatar-border" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    AVATAR_MASK => "Results/avatar-mask" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    TAB_SELECTOR_BACKGROUND => "Results/tab-selector-background" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopCentre)
    ],
    SCORE_CONTAINER_PANEL => "Results/score-container-panel" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopCentre)
    ],
    LABEL_ACCURACY => "Results/label-accuracy" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    LABEL_MAX_COMBO => "Results/label-max-combo" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    LABEL_PERFORMANCE_RATING => "Results/label-performance-rating" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    LABEL_RANKED_ACCURACY => "Results/label-ranked-accuracy" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    LABEL_TOTAL_SCORE => "Results/label-total-score" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    LABEL_BLUE_TEAM => "Results/label-blue-team" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    LABEL_RED_TEAM => "Results/label-red-team" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    LABEL_SCORE => "Results/label-score" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    BACKGROUND => "Results/background" [
        AssetAttribute::Texture,
    ],
    BACKGROUND_FILTER => "Results/background-filter" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopCentre)
    ],
    GRAPH_CONTAINER_PANEL => "Results/graph-container-panel" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::Centre)
    ],
    MULTIPLAYER_FFA_PANEL => "Results/multiplayer-ffa-panel" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
    MULTIPLAYER_TEAM_PANEL => "Results/multiplayer-team-panel" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::BottomCentre)
    ],
});

def_const_type_enum!(pub ResultsGrades => StringPattern {
    LARGE_X => "Grades/grade-large-x" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    LARGE_SS => "Grades/grade-large-ss" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    LARGE_S => "Grades/grade-large-s" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    LARGE_A => "Grades/grade-large-a" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    LARGE_B => "Grades/grade-large-b" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    LARGE_C => "Grades/grade-large-c" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    LARGE_D => "Grades/grade-large-d" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
    LARGE_F => "Grades/grade-large-f" [
        AssetAttribute::Texture, 
        AssetAttribute::Alignment(Anchor::TopLeft)
    ],
});

def_const_type_enum!(pub Sfx => StringPattern {
    CLICK => "SFX/sound-click" [AssetAttribute::Sample],
    BACK => "SFX/sound-back" [AssetAttribute::Sample],
    HOVER => "SFX/sound-hover" [AssetAttribute::Sample],
    APPLAUSE => "SFX/sound-applause" [AssetAttribute::Sample],
    MENU_KEYCLICK => "SFX/sound-menu-keyclick-{number}" [AssetAttribute::Sample],
    SELECT => "SFX/sound-select" [AssetAttribute::Sample],
    HIT => "SFX/sound-hit" [AssetAttribute::Sample],
    HIT_CLAP => "SFX/sound-hitclap" [AssetAttribute::Sample],
    HIT_WHISTLE => "SFX/sound-hitwhistle" [AssetAttribute::Sample],
    HIT_FINISH => "SFX/sound-hitfinish" [AssetAttribute::Sample],
    COMBO_BREAK => "SFX/sound-combobreak" [AssetAttribute::Sample],
    FAILURE => "SFX/sound-failure" [AssetAttribute::Sample],
    RETRY => "SFX/sound-retry" [AssetAttribute::Sample],
    COMBO_ALERT => "SFX/sound-combo-alert-{number}" [AssetAttribute::Sample], // normally this would be in dynamic_assets
});
