use keyberon::action::{k, l, Action, HoldTapAction, HoldTapConfig};
use keyberon::key_code::{KeyCode, KeyCode::*};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CustomActions {
    Bootload,
    Reset,
}
const BOOTLOAD: Action<CustomActions> = Action::Custom(CustomActions::Bootload);
const RESET: Action<CustomActions> = Action::Custom(CustomActions::Reset);

const A_LS: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LShift),
    tap: k(A),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const L5_S: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(5),
    tap: k(S),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const D_LA: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LAlt),
    tap: k(D),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const L2_F: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(2),
    tap: k(F),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const DT_R: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(RAlt),
    tap: k(Dot),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const X_LA: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LAlt),
    tap: k(X),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const SL_R: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(RCtrl),
    tap: k(Slash),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const Z_LC: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LCtrl),
    tap: k(Z),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const L4_C: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(4),
    tap: k(C),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const SM_R: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(RShift),
    tap: k(SColon),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const L7_S: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(7),
    tap: k(Space),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});
const L4_O: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(4),
    tap: k(Comma),
    config: HoldTapConfig::Default,
    tap_hold_interval: 0,
});

const CT_T: Action<CustomActions> =
    Action::MultipleKeyCodes(&[KeyCode::LCtrl, KeyCode::Tab].as_slice());
const SC_T: Action<CustomActions> =
    Action::MultipleKeyCodes(&[KeyCode::LShift, KeyCode::LCtrl, KeyCode::Tab].as_slice());

// TODO: fix chords to map to tab, escape, and enter
pub const CHORDS: [keyberon::chording::ChordDef; 3] = [
    ((0, 0), &[(0, 8), (1, 8)]),
    ((0, 1), &[(0, 9), (1, 9)]),
    ((0, 2), &[(0, 10), (1, 10)]),
];

pub static LAYERS: keyberon::layout::Layers<12, 3, 8, CustomActions> = keyberon::layout::layout! {
    { // 0
        [Q      W      E      R      T {SC_T} {BOOTLOAD}      Y U I      O      P]
        [{A_LS} {L5_S} {D_LA} {L2_F} G BSpace {L7_S} H J K      L      {SM_R}]
        [{Z_LC} {X_LA} {L4_C} V      B {CT_T} t      N M {L4_O} {DT_R} {SL_R}]
    }
    { // 1
        [t t t t t t t t t t t t]
        [t t t t t t t t t t t t]
        [t t t t t t t t t t t t]
    }
    { // 2
        [t t t t {BOOTLOAD} t t * 7 8 9 +]
        [t t t t t          t 0 / 4 5 6 -]
        [t t t t MediaSleep t t . 1 2 3 .]
    }
    { // 3
        [t 7 8 9 t t t t t t t t]
        [t 4 5 6 t t t t t t t t]
        [0 1 2 3 t t t t t t t t]
    }
    { // 4
        [!   @   #   $   % t t t ~   |    '`' +]
        ['{' '}' '(' ')' t t t = '_' -    '"' Quote]
        ['[' ']' ^   &   * t t t /   '\\' t   t]
    }
    { // 5
        [t t t      t t t t t    t    PgUp   t     t]
        [t t Delete t t t t Left Down Up     Right Enter]
        [t t t      t t t t t    Home PgDown End   t]
    }
    { // 6
        [{RESET} {BOOTLOAD} t t t T T F10 F7 F8 F9 MediaSleep]
        [t       t          t t t t t F11 F4 F5 F6 t]
        [t       t          t t t t t F12 F1 F2 F3 t]
    }
    { // 7
        [t t     t   t      t t      t MediaNextSong MediaPlayPause MediaVolDown MediaVolUp PScreen]
        [t Enter Tab Escape t Delete t t             Escape         Tab          Enter      Enter]
        [t t     t   t      t t      t t             t              t            t          Delete]
    }
};
