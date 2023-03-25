/*========================================================
CONFIGURE RUSTC WARNINGS
========================================================*/
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

/*========================================================
MODULE INCLUSIONS
========================================================*/

/*========================================================
ENUM DEFINITIONS
========================================================*/
pub enum atBatResults {
    oddity,
    criticalHit,
    hit,
    walk,
    possibleError,
    productiveOut1,
    productiveOut2,
    out,
    megaOut,
}

/*========================================================
STRUCT DEFINITIONS
========================================================*/

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
pub fn atBat(batTarget: i32, onBaseTarget: i32, pitchResult: i32) -> atBatResults {
    let mut atBatResult = atBatResults::megaOut;
    
    if pitchResult == 1 {
        atBatResult = atBatResults::oddity;
    } else if pitchResult >= 2 && pitchResult <= 5 {
        atBatResult = atBatResults::criticalHit;
    } else if pitchResult >= 6 && pitchResult <= batTarget {
        atBatResult = atBatResults::hit;
    } else if pitchResult > batTarget && pitchResult <= onBaseTarget {
        atBatResult = atBatResults::walk;
    } else if pitchResult > onBaseTarget && pitchResult <= onBaseTarget + 5 {
        atBatResult = atBatResults::possibleError;
    } else if pitchResult >= onBaseTarget + 6 && pitchResult <= 49 {
        atBatResult = atBatResults::productiveOut1;
    } else if pitchResult >= 50 && pitchResult <= 69 {
        atBatResult = atBatResults::productiveOut2;
    } else if pitchResult >= 70 && pitchResult <= 98 {
        atBatResult = atBatResults::out;
    } else if pitchResult == 99 {
        atBatResult = atBatResults::oddity;
    } else if pitchResult >= 100 {
        atBatResult = atBatResults::megaOut;
    }

    atBatResult
}