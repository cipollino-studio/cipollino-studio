
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Key {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    Escape,
    Tab,
    Backspace,
    Enter,
    Space,

    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    Colon,
    Comma,
    Backslash,
    Slash,
    Pipe,
    QuestionMark,
    OpenBracket,
    CloseBracket,
    Backtick,
    Minus,
    Period,
    Plus,
    Equals,
    Semicolon,
    Quote,

    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    A, 
    B,
    C, 
    D, 
    E, 
    F, 
    G, 
    H, 
    I, 
    J, 
    K, 
    L,
    M,
    N,
    O, 
    P, 
    Q,
    R, 
    S, 
    T, 
    U, 
    V, 
    W, 
    X, 
    Y,
    Z, 

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}


bitflags::bitflags! {

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct KeyModifiers: u8 {
        const CONTROL = 1 << 0;
        const SHIFT = 1 << 1;
        const OPTION = 1 << 2;
    } 

}
