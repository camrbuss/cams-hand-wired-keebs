use keyberon::action::{k, l, Action, HoldTapAction, HoldTapConfig};
use keyberon::key_code::{KeyCode, KeyCode::*};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CustomActions {
    Bootload,
    Reset,
}
#[allow(dead_code)]
const BOOTLOAD: Action<CustomActions> = Action::Custom(CustomActions::Bootload);
#[allow(dead_code)]
const RESET: Action<CustomActions> = Action::Custom(CustomActions::Reset);

#[allow(dead_code)]
const A_LS: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LShift),
    tap: k(A),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const L5_S: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(5),
    tap: k(S),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const D_LA: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LAlt),
    tap: k(D),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const L2_F: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(2),
    tap: k(F),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const DT_R: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(RAlt),
    tap: k(Dot),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const X_LA: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LAlt),
    tap: k(X),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const SL_R: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(RCtrl),
    tap: k(Slash),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const Z_LC: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LCtrl),
    tap: k(Z),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const L4_C: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(4),
    tap: k(C),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const SM_R: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(RShift),
    tap: k(SColon),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const L7_S: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(7),
    tap: k(Space),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const L4_O: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(4),
    tap: k(Comma),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const L3_D: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(3),
    tap: k(D),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
#[allow(dead_code)]
const L4_S: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(4),
    tap: k(S),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

#[allow(dead_code)]
const CT_T: Action<CustomActions> =
    Action::MultipleKeyCodes(&[KeyCode::LCtrl, KeyCode::Tab].as_slice());
#[allow(dead_code)]
const SC_T: Action<CustomActions> =
    Action::MultipleKeyCodes(&[KeyCode::LShift, KeyCode::LCtrl, KeyCode::Tab].as_slice());
#[allow(dead_code)]
const CA_D: Action<CustomActions> = Action::MultipleKeyCodes(&[LCtrl, LAlt, Delete].as_slice());

pub static LAYERS: keyberon::layout::Layers<12, 5, 5, CustomActions> = keyberon::layout::layout! {
    { // 0
        [ '`'    1    2    3      4      5      6      7 8    9    0  -     ]
        [ Tab    Q    W    E      R      T      Y      U I    O    P  '\\'  ]
        [ Escape A    S    {L3_D} {L2_F} G      H      J K    L    ;  Quote ]
        [ LShift Z    X    C      V      B      N      M ,    .    /  Enter ]
        [ LCtrl  LGui LAlt (2)    (1)    BSpace {L4_S} A Left Down Up Right ]
    }
    { // 1
        [ t F1 F2 F3 F4     F5 F6 F7 F8 F9 F10 t ]
        [ t t  t  t  t      t  t  t  t  t  t   t ]
        [ t t  t  t  {CA_D} t  t  t  t  t  t   t ]
        [ t t  t  t  t      t  t  t  t  t  t   t ]
        [ t t  t  t  t      t  t  t  t  t  t   t ]
    }
    { // 2
        [ {BOOTLOAD}  t t t t t t t t t t t ]
        [ t           t t t t t t 7 8 9 t t ]
        [ t           t t t t t t 4 5 6 t t ]
        [ t           t t t t t t 1 2 3 . t ]
        [ t           t t t t t 0 t t t t t ]
    }
    { // 3
        [ t t   t   t t t t   t   t    t   t     t ]
        [ t '(' ')' t t t t   '_' |    =   +     t ]
        [ t '{' '}' t t t '`' ~   /    '"' Quote t ]
        [ t '[' ']' ^ & * t   -   '\\' t   t     t ]
        [ t t   t   t t t t   t   t    t   t     t ]
    }
    { // 4
        [ t t t t      t t t    t    t      t     t       t ]
        [ t t t t      t t t    t    PgUp   t     PScreen t ]
        [ t t t t      t t Left Down Up     Right t       t ]
        [ t t t t      t t t    Home PgDown End   t       t ]
        [ t t t Delete t t t    t    t      t     t       t ]
    }
};
