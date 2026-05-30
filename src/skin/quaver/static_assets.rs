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
