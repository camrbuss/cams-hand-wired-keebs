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
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const L5_S: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(5),
    tap: k(S),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const D_LA: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LAlt),
    tap: k(D),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const L2_F: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(2),
    tap: k(F),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const DT_R: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(RAlt),
    tap: k(Dot),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const X_LA: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LAlt),
    tap: k(X),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const SL_R: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(RCtrl),
    tap: k(Slash),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const Z_LC: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(LCtrl),
    tap: k(Z),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const L4_C: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(4),
    tap: k(C),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const SM_R: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: k(RShift),
    tap: k(SColon),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const L7_S: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(7),
    tap: k(Space),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});
const L4_O: Action<CustomActions> = Action::HoldTap(&HoldTapAction {
    timeout: 200,
    hold: l(4),
    tap: k(Comma),
    config: HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

const CT_T: Action<CustomActions> =
    Action::MultipleKeyCodes(&[KeyCode::LCtrl, KeyCode::Tab].as_slice());
const SC_T: Action<CustomActions> =
    Action::MultipleKeyCodes(&[KeyCode::LShift, KeyCode::LCtrl, KeyCode::Tab].as_slice());
const CA_D: Action<CustomActions> = Action::MultipleKeyCodes(&[LCtrl, LAlt, Delete].as_slice());

pub const CHORDS: [keyberon::chording::ChordDef; 3] = [
    ((3, 4), &[(0, 6), (1, 6)]),
    ((3, 9), &[(0, 7), (1, 7)]),
    ((3, 5), &[(0, 8), (1, 8)]),
];

pub static LAYERS: keyberon::layout::Layers<10, 4, 8, CustomActions> = keyberon::layout::layout! {
    { // 0
        [Q          W      E      R      T Y U      I      O      P       ]
        [{A_LS}     {L5_S} {D_LA} {L2_F} G H J      K      L      {SM_R}  ]
        [{Z_LC}     {X_LA} {L4_C} V      B N M      {L4_O} {DT_R} {SL_R}  ]
        [LGui {SC_T} BSpace {CT_T} Escape Enter {CA_D} {L7_S} LAlt Tab ]
    }
    { // 1
        [ t t t t t t t t t t  ]
        [ t t t t t t t t t t  ]
        [ t t t t t t t t t t  ]
        [ t t t t t t t t t t  ]
    }
    { // 2
        [ t t t   t t * 7 8 9 + ]
        [ t t t   t t / 4 5 6 - ]
        [ t t (6) t t t 1 2 3 . ]
        [ t t t   t t t t 0 t t ]
    }
    { // 3
        [  * 7 8 9 + t t t t t  ]
        [  / 4 5 6 - t t t t t  ]
        [  t 1 2 3 . t t t t t  ]
        [  t t 0 t t t t t t t  ]
    }
    { // 4
        [ !   @   #   $   % t ~   |    '`' +     ]
        [ '{' '}' '(' ')' t = '_' -    '"' Quote ]
        [ '[' ']' ^   &   * t /   '\\' t   t     ]
        [ t   t   t   t   t t t   t    t   t     ]
    }
    { // 5
        [  t t t t t t    t    PgUp t     t      ]
        [  t t t t t Left Down Up   Right Enter  ]
        [  t t t t t t    Home Down End   t      ]
        [  t t t t t t    t    t    t     t      ]
    }
    { // 6
        [ {RESET} {BOOTLOAD} t t t t F7 F8 F9 MediaSleep ]
        [ t       t          t t t t F4 F5 F6 t          ]
        [ t       t          t t t t F1 F2 F3 t          ]
        [ t       t          t t t t t  t  t  t          ]
    }
    { // 7
        [ t t t      t t MediaNextSong MediaPlayPause MediaVolDown MediaVolUp PScreen ]
        [ t t t      t t t             Escape         Tab          Enter      t       ]
        [ t t t      t t t             Home           PgDown       PgUp       End     ]
        [ t t Delete t t t             t              t            t          t       ]
    }
};
