use keyberon::action::Action::Trans;
use keyberon::action::{k, l, m};
use keyberon::key_code::KeyCode::*;

// Shift + KeyCode
macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers = &[
    &[
        &[k(Grave), k(Kb1),k(Kb2),k(Kb3), k(Kb4), k(Kb5),  k(Kb6),  k(Kb7),k(Kb8), k(Kb9),  k(Kb0),   k(BSpace)],
        &[k(Tab),   k(Q),  k(W),  k(E),   k(R),   k(T),    k(Y),    k(U),  k(I),   k(O),    k(P),     k(Delete)],
        &[k(Escape),k(A),  k(S),  k(D),   k(F),   k(G),    k(H),    k(J),  k(K),   k(L),    k(SColon),k(Quote) ],
        &[k(LShift),k(Z),  k(X),  k(C),   k(V),   k(B),    k(N),    k(M),  k(Comma),k(Dot), k(Slash), k(Enter) ],
        &[k(LCtrl), k(LCtrl),k(LAlt),k(LGui),l(1),k(Space),k(Space),l(2),  k(Left),k(Down), k(Up),    k(Right) ],
    ], &[
        &[s!(Grave), s!(Kb1),s!(Kb2),s!(Kb3), s!(Kb4), s!(Kb5),  s!(Kb6),s!(Kb7),      s!(Kb8),         s!(Kb9),     s!(Kb0),     k(BSpace)        ],
        &[s!(Grave), s!(Kb1),s!(Kb2),s!(Kb3), s!(Kb4), s!(Kb5),  s!(Kb6),s!(Kb7),      s!(Kb8),         s!(Kb9),     s!(Kb0),     k(Delete)        ],
        &[k(Delete), k(F1),  k(F2),  k(F3),   k(F4),   k(F5),    k(F6),  s!(Minus),    s!(Equal),       s!(LBracket),s!(RBracket),s!(Bslash)       ],
        &[Trans,     k(F7),  k(F8),  k(F9),   k(F10),  k(F11),   k(F12), s!(NonUsHash),s!(NonUsBslash), Trans,       Trans,       Trans            ],
        &[Trans,     Trans,  Trans,  Trans,   Trans,   Trans,    Trans,  Trans,        k(MediaNextSong),k(VolDown),  k(VolUp),    k(MediaPlayPause)],
    ], &[
        &[k(Grave), k(Kb1),k(Kb2),k(Kb3), k(Kb4), k(Kb5),  k(Kb6),k(Kb7),      k(Kb8),          k(Kb9),     k(Kb0),     k(BSpace)        ],
        &[k(Grave), k(Kb1),k(Kb2),k(Kb3), k(Kb4), k(Kb5),  k(Kb6),k(Kb7),      k(Kb8),          k(Kb9),     k(Kb0),     k(Delete)        ],
        &[k(Delete),k(F1), k(F2), k(F3),  k(F4),  k(F5),   k(F6), k(Minus),    k(Equal),        k(LBracket),k(RBracket),k(Bslash)        ],
        &[Trans,    k(F7), k(F8), k(F9),  k(F10), k(F11),  k(F12),k(NonUsHash),k(NonUsBslash),  Trans,      Trans,      Trans            ],
        &[Trans,    Trans, Trans, Trans,  Trans,  Trans,   Trans, Trans,       k(MediaNextSong),k(VolDown), k(VolUp),   k(MediaPlayPause)],
    ]
];
