/*========================================================
CONFIGURE RUSTC WARNINGS
========================================================*/
//#[allow(non_camel_case_types)]
//#[allow(non_snake_case)]

/*========================================================
MODULE INCLUSIONS
========================================================*/

/*========================================================
ENUM DEFINITIONS
========================================================*/
pub enum AtBatResults {
    Oddity,
    CriticalHit,
    Hit,
    Walk,
    PossibleError,
    ProductiveOut1,
    ProductiveOut2,
    Out,
    MegaOut,
}

/*========================================================
STRUCT DEFINITIONS
========================================================*/

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
pub fn at_bat(bat_target: i32, on_base_target: i32, pitch_result: i32) -> AtBatResults {
    let mut at_bat_result = AtBatResults::MegaOut;

    if pitch_result == 1 {
        at_bat_result = AtBatResults::Oddity;
    } else if pitch_result >= 2 && pitch_result <= 5 {
        at_bat_result = AtBatResults::CriticalHit;
    } else if pitch_result >= 6 && pitch_result <= bat_target {
        at_bat_result = AtBatResults::Hit;
    } else if pitch_result > bat_target && pitch_result <= on_base_target {
        at_bat_result = AtBatResults::Walk;
    } else if pitch_result > on_base_target && pitch_result <= on_base_target + 5 {
        at_bat_result = AtBatResults::PossibleError;
    } else if pitch_result >= on_base_target + 6 && pitch_result <= 49 {
        at_bat_result = AtBatResults::ProductiveOut1;
    } else if pitch_result >= 50 && pitch_result <= 69 {
        at_bat_result = AtBatResults::ProductiveOut2;
    } else if pitch_result >= 70 && pitch_result <= 98 {
        at_bat_result = AtBatResults::Out;
    } else if pitch_result == 99 {
        at_bat_result = AtBatResults::Oddity;
    } else if pitch_result >= 100 {
        at_bat_result = AtBatResults::MegaOut;
    }

    at_bat_result
}

