/* THIS FILE IS GENERATED, DO NOT EDIT */

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    SYN,
    KEY,
    REL,
    ABS,
    MSC,
    SW,
    LED,
    SND,
    REP,
    FF,
    PWR,
    FF_STATUS,
    UNKNOWN(TypeRaw),
}

impl Type {
    pub fn from_raw(r#type: TypeRaw) -> Type {
        match r#type {
            0x0 => Type::SYN,
            0x1 => Type::KEY,
            0x2 => Type::REL,
            0x3 => Type::ABS,
            0x4 => Type::MSC,
            0x5 => Type::SW,
            0x11 => Type::LED,
            0x12 => Type::SND,
            0x14 => Type::REP,
            0x15 => Type::FF,
            0x16 => Type::PWR,
            0x17 => Type::FF_STATUS,
            _ => Type::UNKNOWN(r#type),
        }
    }
    pub fn as_raw(&self) -> TypeRaw {
        match *self {
            Type::SYN => 0x0,
            Type::KEY => 0x1,
            Type::REL => 0x2,
            Type::ABS => 0x3,
            Type::MSC => 0x4,
            Type::SW => 0x5,
            Type::LED => 0x11,
            Type::SND => 0x12,
            Type::REP => 0x14,
            Type::FF => 0x15,
            Type::PWR => 0x16,
            Type::FF_STATUS => 0x17,
            Type::UNKNOWN(r#type) => r#type,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Code {
    SYN(CodeValRaw),
    KEY(CodeValRaw),
    REL(CodeValRaw),
    ABS(CodeValRaw),
    MSC(CodeValRaw),
    SW(CodeValRaw),
    LED(CodeValRaw),
    SND(CodeValRaw),
    REP(CodeValRaw),
    FF(CodeValRaw),
    PWR(CodeValRaw),
    FF_STATUS(CodeValRaw),
    UNKNOWN(TypeRaw, CodeValRaw),
}

impl Code {
    pub fn from_raw(r#type: Type, codeval: CodeValRaw) -> Code {
        let r#type = r#type.as_raw();
        match r#type {
            0x0 => Code::SYN(codeval),
            0x1 => Code::KEY(codeval),
            0x2 => Code::REL(codeval),
            0x3 => Code::ABS(codeval),
            0x4 => Code::MSC(codeval),
            0x5 => Code::SW(codeval),
            0x11 => Code::LED(codeval),
            0x12 => Code::SND(codeval),
            0x14 => Code::REP(codeval),
            0x15 => Code::FF(codeval),
            0x16 => Code::PWR(codeval),
            0x17 => Code::FF_STATUS(codeval),
            _ => Code::UNKNOWN(r#type, codeval),
        }
    }
    pub fn get_type(&self) -> Type {
        match *self {
            Code::SYN(..) => Type::SYN,
            Code::KEY(..) => Type::KEY,
            Code::REL(..) => Type::REL,
            Code::ABS(..) => Type::ABS,
            Code::MSC(..) => Type::MSC,
            Code::SW(..) => Type::SW,
            Code::LED(..) => Type::LED,
            Code::SND(..) => Type::SND,
            Code::REP(..) => Type::REP,
            Code::FF(..) => Type::FF,
            Code::PWR(..) => Type::PWR,
            Code::FF_STATUS(..) => Type::FF_STATUS,
            Code::UNKNOWN(r#type, _) => Type::UNKNOWN(r#type),
        }
    }
    pub fn as_raw(&self) -> (TypeRaw, CodeValRaw) {
        match *self {
            Code::SYN(codeval) => (0x0, codeval),
            Code::KEY(codeval) => (0x1, codeval),
            Code::REL(codeval) => (0x2, codeval),
            Code::ABS(codeval) => (0x3, codeval),
            Code::MSC(codeval) => (0x4, codeval),
            Code::SW(codeval) => (0x5, codeval),
            Code::LED(codeval) => (0x11, codeval),
            Code::SND(codeval) => (0x12, codeval),
            Code::REP(codeval) => (0x14, codeval),
            Code::FF(codeval) => (0x15, codeval),
            Code::PWR(codeval) => (0x16, codeval),
            Code::FF_STATUS(codeval) => (0x17, codeval),
            Code::UNKNOWN(r#type, codeval) => (r#type, codeval),
        }
    }
}

impl AsCodeRaw for (Type, CodeValRaw) {
    fn as_code_raw(&self) -> CodeRaw {
        match self.0 {
            Type::SYN => (0x0, self.1),
            Type::KEY => (0x1, self.1),
            Type::REL => (0x2, self.1),
            Type::ABS => (0x3, self.1),
            Type::MSC => (0x4, self.1),
            Type::SW => (0x5, self.1),
            Type::LED => (0x11, self.1),
            Type::SND => (0x12, self.1),
            Type::REP => (0x14, self.1),
            Type::FF => (0x15, self.1),
            Type::PWR => (0x16, self.1),
            Type::FF_STATUS => (0x17, self.1),
            Type::UNKNOWN(r#type) => (r#type, self.1),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SYN {
    REPORT = 0x0,
    CONFIG = 0x1,
    MT_REPORT = 0x2,
    DROPPED = 0x3,
}

impl SYN {
    pub fn from_raw(codeval: CodeValRaw) -> Option<SYN> {
        match codeval {
            0x0 => Some(SYN::REPORT),
            0x1 => Some(SYN::CONFIG),
            0x2 => Some(SYN::MT_REPORT),
            0x3 => Some(SYN::DROPPED),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        15
    }
}

impl AsCodeRaw for SYN {
    fn as_code_raw(&self) -> CodeRaw {
        (0x0, *self as CodeValRaw)
    }
}

impl AsCode for SYN {
    fn as_code(&self) -> Code {
        Code::SYN(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum KEY {
    RESERVED = 0x0,
    ESC = 0x1,
    KEY_1 = 0x2,
    KEY_2 = 0x3,
    KEY_3 = 0x4,
    KEY_4 = 0x5,
    KEY_5 = 0x6,
    KEY_6 = 0x7,
    KEY_7 = 0x8,
    KEY_8 = 0x9,
    KEY_9 = 0xa,
    KEY_0 = 0xb,
    MINUS = 0xc,
    EQUAL = 0xd,
    BACKSPACE = 0xe,
    TAB = 0xf,
    Q = 0x10,
    W = 0x11,
    E = 0x12,
    R = 0x13,
    T = 0x14,
    Y = 0x15,
    U = 0x16,
    I = 0x17,
    O = 0x18,
    P = 0x19,
    LEFTBRACE = 0x1a,
    RIGHTBRACE = 0x1b,
    ENTER = 0x1c,
    LEFTCTRL = 0x1d,
    A = 0x1e,
    S = 0x1f,
    D = 0x20,
    F = 0x21,
    G = 0x22,
    H = 0x23,
    J = 0x24,
    K = 0x25,
    L = 0x26,
    SEMICOLON = 0x27,
    APOSTROPHE = 0x28,
    GRAVE = 0x29,
    LEFTSHIFT = 0x2a,
    BACKSLASH = 0x2b,
    Z = 0x2c,
    X = 0x2d,
    C = 0x2e,
    V = 0x2f,
    B = 0x30,
    N = 0x31,
    M = 0x32,
    COMMA = 0x33,
    DOT = 0x34,
    SLASH = 0x35,
    RIGHTSHIFT = 0x36,
    KPASTERISK = 0x37,
    LEFTALT = 0x38,
    SPACE = 0x39,
    CAPSLOCK = 0x3a,
    F1 = 0x3b,
    F2 = 0x3c,
    F3 = 0x3d,
    F4 = 0x3e,
    F5 = 0x3f,
    F6 = 0x40,
    F7 = 0x41,
    F8 = 0x42,
    F9 = 0x43,
    F10 = 0x44,
    NUMLOCK = 0x45,
    SCROLLLOCK = 0x46,
    KP7 = 0x47,
    KP8 = 0x48,
    KP9 = 0x49,
    KPMINUS = 0x4a,
    KP4 = 0x4b,
    KP5 = 0x4c,
    KP6 = 0x4d,
    KPPLUS = 0x4e,
    KP1 = 0x4f,
    KP2 = 0x50,
    KP3 = 0x51,
    KP0 = 0x52,
    KPDOT = 0x53,
    ZENKAKUHANKAKU = 0x55,
    KEY_102ND = 0x56,
    F11 = 0x57,
    F12 = 0x58,
    RO = 0x59,
    KATAKANA = 0x5a,
    HIRAGANA = 0x5b,
    HENKAN = 0x5c,
    KATAKANAHIRAGANA = 0x5d,
    MUHENKAN = 0x5e,
    KPJPCOMMA = 0x5f,
    KPENTER = 0x60,
    RIGHTCTRL = 0x61,
    KPSLASH = 0x62,
    SYSRQ = 0x63,
    RIGHTALT = 0x64,
    LINEFEED = 0x65,
    HOME = 0x66,
    UP = 0x67,
    PAGEUP = 0x68,
    LEFT = 0x69,
    RIGHT = 0x6a,
    END = 0x6b,
    DOWN = 0x6c,
    PAGEDOWN = 0x6d,
    INSERT = 0x6e,
    DELETE = 0x6f,
    MACRO = 0x70,
    MUTE = 0x71,
    VOLUMEDOWN = 0x72,
    VOLUMEUP = 0x73,
    POWER = 0x74,
    KPEQUAL = 0x75,
    KPPLUSMINUS = 0x76,
    PAUSE = 0x77,
    SCALE = 0x78,
    KPCOMMA = 0x79,
    HANGEUL = 0x7a,
    HANJA = 0x7b,
    YEN = 0x7c,
    LEFTMETA = 0x7d,
    RIGHTMETA = 0x7e,
    COMPOSE = 0x7f,
    STOP = 0x80,
    AGAIN = 0x81,
    PROPS = 0x82,
    UNDO = 0x83,
    FRONT = 0x84,
    COPY = 0x85,
    OPEN = 0x86,
    PASTE = 0x87,
    FIND = 0x88,
    CUT = 0x89,
    HELP = 0x8a,
    MENU = 0x8b,
    CALC = 0x8c,
    SETUP = 0x8d,
    SLEEP = 0x8e,
    WAKEUP = 0x8f,
    FILE = 0x90,
    SENDFILE = 0x91,
    DELETEFILE = 0x92,
    XFER = 0x93,
    PROG1 = 0x94,
    PROG2 = 0x95,
    WWW = 0x96,
    MSDOS = 0x97,
    COFFEE = 0x98,
    ROTATE_DISPLAY = 0x99,
    CYCLEWINDOWS = 0x9a,
    MAIL = 0x9b,
    BOOKMARKS = 0x9c,
    COMPUTER = 0x9d,
    BACK = 0x9e,
    FORWARD = 0x9f,
    CLOSECD = 0xa0,
    EJECTCD = 0xa1,
    EJECTCLOSECD = 0xa2,
    NEXTSONG = 0xa3,
    PLAYPAUSE = 0xa4,
    PREVIOUSSONG = 0xa5,
    STOPCD = 0xa6,
    RECORD = 0xa7,
    REWIND = 0xa8,
    PHONE = 0xa9,
    ISO = 0xaa,
    CONFIG = 0xab,
    HOMEPAGE = 0xac,
    REFRESH = 0xad,
    EXIT = 0xae,
    MOVE = 0xaf,
    EDIT = 0xb0,
    SCROLLUP = 0xb1,
    SCROLLDOWN = 0xb2,
    KPLEFTPAREN = 0xb3,
    KPRIGHTPAREN = 0xb4,
    NEW = 0xb5,
    REDO = 0xb6,
    F13 = 0xb7,
    F14 = 0xb8,
    F15 = 0xb9,
    F16 = 0xba,
    F17 = 0xbb,
    F18 = 0xbc,
    F19 = 0xbd,
    F20 = 0xbe,
    F21 = 0xbf,
    F22 = 0xc0,
    F23 = 0xc1,
    F24 = 0xc2,
    PLAYCD = 0xc8,
    PAUSECD = 0xc9,
    PROG3 = 0xca,
    PROG4 = 0xcb,
    DASHBOARD = 0xcc,
    SUSPEND = 0xcd,
    CLOSE = 0xce,
    PLAY = 0xcf,
    FASTFORWARD = 0xd0,
    BASSBOOST = 0xd1,
    PRINT = 0xd2,
    HP = 0xd3,
    CAMERA = 0xd4,
    SOUND = 0xd5,
    QUESTION = 0xd6,
    EMAIL = 0xd7,
    CHAT = 0xd8,
    SEARCH = 0xd9,
    CONNECT = 0xda,
    FINANCE = 0xdb,
    SPORT = 0xdc,
    SHOP = 0xdd,
    ALTERASE = 0xde,
    CANCEL = 0xdf,
    BRIGHTNESSDOWN = 0xe0,
    BRIGHTNESSUP = 0xe1,
    MEDIA = 0xe2,
    SWITCHVIDEOMODE = 0xe3,
    KBDILLUMTOGGLE = 0xe4,
    KBDILLUMDOWN = 0xe5,
    KBDILLUMUP = 0xe6,
    SEND = 0xe7,
    REPLY = 0xe8,
    FORWARDMAIL = 0xe9,
    SAVE = 0xea,
    DOCUMENTS = 0xeb,
    BATTERY = 0xec,
    BLUETOOTH = 0xed,
    WLAN = 0xee,
    UWB = 0xef,
    UNKNOWN = 0xf0,
    VIDEO_NEXT = 0xf1,
    VIDEO_PREV = 0xf2,
    BRIGHTNESS_CYCLE = 0xf3,
    BRIGHTNESS_AUTO = 0xf4,
    DISPLAY_OFF = 0xf5,
    WWAN = 0xf6,
    RFKILL = 0xf7,
    MICMUTE = 0xf8,
    OK = 0x160,
    SELECT = 0x161,
    GOTO = 0x162,
    CLEAR = 0x163,
    POWER2 = 0x164,
    OPTION = 0x165,
    INFO = 0x166,
    TIME = 0x167,
    VENDOR = 0x168,
    ARCHIVE = 0x169,
    PROGRAM = 0x16a,
    CHANNEL = 0x16b,
    FAVORITES = 0x16c,
    EPG = 0x16d,
    PVR = 0x16e,
    MHP = 0x16f,
    LANGUAGE = 0x170,
    TITLE = 0x171,
    SUBTITLE = 0x172,
    ANGLE = 0x173,
    ZOOM = 0x174,
    MODE = 0x175,
    KEYBOARD = 0x176,
    SCREEN = 0x177,
    PC = 0x178,
    TV = 0x179,
    TV2 = 0x17a,
    VCR = 0x17b,
    VCR2 = 0x17c,
    SAT = 0x17d,
    SAT2 = 0x17e,
    CD = 0x17f,
    TAPE = 0x180,
    RADIO = 0x181,
    TUNER = 0x182,
    PLAYER = 0x183,
    TEXT = 0x184,
    DVD = 0x185,
    AUX = 0x186,
    MP3 = 0x187,
    AUDIO = 0x188,
    VIDEO = 0x189,
    DIRECTORY = 0x18a,
    LIST = 0x18b,
    MEMO = 0x18c,
    CALENDAR = 0x18d,
    RED = 0x18e,
    GREEN = 0x18f,
    YELLOW = 0x190,
    BLUE = 0x191,
    CHANNELUP = 0x192,
    CHANNELDOWN = 0x193,
    FIRST = 0x194,
    LAST = 0x195,
    AB = 0x196,
    NEXT = 0x197,
    RESTART = 0x198,
    SLOW = 0x199,
    SHUFFLE = 0x19a,
    BREAK = 0x19b,
    PREVIOUS = 0x19c,
    DIGITS = 0x19d,
    TEEN = 0x19e,
    TWEN = 0x19f,
    VIDEOPHONE = 0x1a0,
    GAMES = 0x1a1,
    ZOOMIN = 0x1a2,
    ZOOMOUT = 0x1a3,
    ZOOMRESET = 0x1a4,
    WORDPROCESSOR = 0x1a5,
    EDITOR = 0x1a6,
    SPREADSHEET = 0x1a7,
    GRAPHICSEDITOR = 0x1a8,
    PRESENTATION = 0x1a9,
    DATABASE = 0x1aa,
    NEWS = 0x1ab,
    VOICEMAIL = 0x1ac,
    ADDRESSBOOK = 0x1ad,
    MESSENGER = 0x1ae,
    DISPLAYTOGGLE = 0x1af,
    SPELLCHECK = 0x1b0,
    LOGOFF = 0x1b1,
    DOLLAR = 0x1b2,
    EURO = 0x1b3,
    FRAMEBACK = 0x1b4,
    FRAMEFORWARD = 0x1b5,
    CONTEXT_MENU = 0x1b6,
    MEDIA_REPEAT = 0x1b7,
    KEY_10CHANNELSUP = 0x1b8,
    KEY_10CHANNELSDOWN = 0x1b9,
    IMAGES = 0x1ba,
    DEL_EOL = 0x1c0,
    DEL_EOS = 0x1c1,
    INS_LINE = 0x1c2,
    DEL_LINE = 0x1c3,
    FN = 0x1d0,
    FN_ESC = 0x1d1,
    FN_F1 = 0x1d2,
    FN_F2 = 0x1d3,
    FN_F3 = 0x1d4,
    FN_F4 = 0x1d5,
    FN_F5 = 0x1d6,
    FN_F6 = 0x1d7,
    FN_F7 = 0x1d8,
    FN_F8 = 0x1d9,
    FN_F9 = 0x1da,
    FN_F10 = 0x1db,
    FN_F11 = 0x1dc,
    FN_F12 = 0x1dd,
    FN_1 = 0x1de,
    FN_2 = 0x1df,
    FN_D = 0x1e0,
    FN_E = 0x1e1,
    FN_F = 0x1e2,
    FN_S = 0x1e3,
    FN_B = 0x1e4,
    BRL_DOT1 = 0x1f1,
    BRL_DOT2 = 0x1f2,
    BRL_DOT3 = 0x1f3,
    BRL_DOT4 = 0x1f4,
    BRL_DOT5 = 0x1f5,
    BRL_DOT6 = 0x1f6,
    BRL_DOT7 = 0x1f7,
    BRL_DOT8 = 0x1f8,
    BRL_DOT9 = 0x1f9,
    BRL_DOT10 = 0x1fa,
    NUMERIC_0 = 0x200,
    NUMERIC_1 = 0x201,
    NUMERIC_2 = 0x202,
    NUMERIC_3 = 0x203,
    NUMERIC_4 = 0x204,
    NUMERIC_5 = 0x205,
    NUMERIC_6 = 0x206,
    NUMERIC_7 = 0x207,
    NUMERIC_8 = 0x208,
    NUMERIC_9 = 0x209,
    NUMERIC_STAR = 0x20a,
    NUMERIC_POUND = 0x20b,
    NUMERIC_A = 0x20c,
    NUMERIC_B = 0x20d,
    NUMERIC_C = 0x20e,
    NUMERIC_D = 0x20f,
    CAMERA_FOCUS = 0x210,
    WPS_BUTTON = 0x211,
    TOUCHPAD_TOGGLE = 0x212,
    TOUCHPAD_ON = 0x213,
    TOUCHPAD_OFF = 0x214,
    CAMERA_ZOOMIN = 0x215,
    CAMERA_ZOOMOUT = 0x216,
    CAMERA_UP = 0x217,
    CAMERA_DOWN = 0x218,
    CAMERA_LEFT = 0x219,
    CAMERA_RIGHT = 0x21a,
    ATTENDANT_ON = 0x21b,
    ATTENDANT_OFF = 0x21c,
    ATTENDANT_TOGGLE = 0x21d,
    LIGHTS_TOGGLE = 0x21e,
    ALS_TOGGLE = 0x230,
    ROTATE_LOCK_TOGGLE = 0x231,
    BUTTONCONFIG = 0x240,
    TASKMANAGER = 0x241,
    JOURNAL = 0x242,
    CONTROLPANEL = 0x243,
    APPSELECT = 0x244,
    SCREENSAVER = 0x245,
    VOICECOMMAND = 0x246,
    ASSISTANT = 0x247,
    BRIGHTNESS_MIN = 0x250,
    BRIGHTNESS_MAX = 0x251,
    KBDINPUTASSIST_PREV = 0x260,
    KBDINPUTASSIST_NEXT = 0x261,
    KBDINPUTASSIST_PREVGROUP = 0x262,
    KBDINPUTASSIST_NEXTGROUP = 0x263,
    KBDINPUTASSIST_ACCEPT = 0x264,
    KBDINPUTASSIST_CANCEL = 0x265,
    RIGHT_UP = 0x266,
    RIGHT_DOWN = 0x267,
    LEFT_UP = 0x268,
    LEFT_DOWN = 0x269,
    ROOT_MENU = 0x26a,
    MEDIA_TOP_MENU = 0x26b,
    NUMERIC_11 = 0x26c,
    NUMERIC_12 = 0x26d,
    AUDIO_DESC = 0x26e,
    KEY_3D_MODE = 0x26f,
    NEXT_FAVORITE = 0x270,
    STOP_RECORD = 0x271,
    PAUSE_RECORD = 0x272,
    VOD = 0x273,
    UNMUTE = 0x274,
    FASTREVERSE = 0x275,
    SLOWREVERSE = 0x276,
    DATA = 0x277,
    ONSCREEN_KEYBOARD = 0x278,
}

impl KEY {
    pub fn from_raw(codeval: CodeValRaw) -> Option<KEY> {
        match codeval {
            0x0 => Some(KEY::RESERVED),
            0x1 => Some(KEY::ESC),
            0x2 => Some(KEY::KEY_1),
            0x3 => Some(KEY::KEY_2),
            0x4 => Some(KEY::KEY_3),
            0x5 => Some(KEY::KEY_4),
            0x6 => Some(KEY::KEY_5),
            0x7 => Some(KEY::KEY_6),
            0x8 => Some(KEY::KEY_7),
            0x9 => Some(KEY::KEY_8),
            0xa => Some(KEY::KEY_9),
            0xb => Some(KEY::KEY_0),
            0xc => Some(KEY::MINUS),
            0xd => Some(KEY::EQUAL),
            0xe => Some(KEY::BACKSPACE),
            0xf => Some(KEY::TAB),
            0x10 => Some(KEY::Q),
            0x11 => Some(KEY::W),
            0x12 => Some(KEY::E),
            0x13 => Some(KEY::R),
            0x14 => Some(KEY::T),
            0x15 => Some(KEY::Y),
            0x16 => Some(KEY::U),
            0x17 => Some(KEY::I),
            0x18 => Some(KEY::O),
            0x19 => Some(KEY::P),
            0x1a => Some(KEY::LEFTBRACE),
            0x1b => Some(KEY::RIGHTBRACE),
            0x1c => Some(KEY::ENTER),
            0x1d => Some(KEY::LEFTCTRL),
            0x1e => Some(KEY::A),
            0x1f => Some(KEY::S),
            0x20 => Some(KEY::D),
            0x21 => Some(KEY::F),
            0x22 => Some(KEY::G),
            0x23 => Some(KEY::H),
            0x24 => Some(KEY::J),
            0x25 => Some(KEY::K),
            0x26 => Some(KEY::L),
            0x27 => Some(KEY::SEMICOLON),
            0x28 => Some(KEY::APOSTROPHE),
            0x29 => Some(KEY::GRAVE),
            0x2a => Some(KEY::LEFTSHIFT),
            0x2b => Some(KEY::BACKSLASH),
            0x2c => Some(KEY::Z),
            0x2d => Some(KEY::X),
            0x2e => Some(KEY::C),
            0x2f => Some(KEY::V),
            0x30 => Some(KEY::B),
            0x31 => Some(KEY::N),
            0x32 => Some(KEY::M),
            0x33 => Some(KEY::COMMA),
            0x34 => Some(KEY::DOT),
            0x35 => Some(KEY::SLASH),
            0x36 => Some(KEY::RIGHTSHIFT),
            0x37 => Some(KEY::KPASTERISK),
            0x38 => Some(KEY::LEFTALT),
            0x39 => Some(KEY::SPACE),
            0x3a => Some(KEY::CAPSLOCK),
            0x3b => Some(KEY::F1),
            0x3c => Some(KEY::F2),
            0x3d => Some(KEY::F3),
            0x3e => Some(KEY::F4),
            0x3f => Some(KEY::F5),
            0x40 => Some(KEY::F6),
            0x41 => Some(KEY::F7),
            0x42 => Some(KEY::F8),
            0x43 => Some(KEY::F9),
            0x44 => Some(KEY::F10),
            0x45 => Some(KEY::NUMLOCK),
            0x46 => Some(KEY::SCROLLLOCK),
            0x47 => Some(KEY::KP7),
            0x48 => Some(KEY::KP8),
            0x49 => Some(KEY::KP9),
            0x4a => Some(KEY::KPMINUS),
            0x4b => Some(KEY::KP4),
            0x4c => Some(KEY::KP5),
            0x4d => Some(KEY::KP6),
            0x4e => Some(KEY::KPPLUS),
            0x4f => Some(KEY::KP1),
            0x50 => Some(KEY::KP2),
            0x51 => Some(KEY::KP3),
            0x52 => Some(KEY::KP0),
            0x53 => Some(KEY::KPDOT),
            0x55 => Some(KEY::ZENKAKUHANKAKU),
            0x56 => Some(KEY::KEY_102ND),
            0x57 => Some(KEY::F11),
            0x58 => Some(KEY::F12),
            0x59 => Some(KEY::RO),
            0x5a => Some(KEY::KATAKANA),
            0x5b => Some(KEY::HIRAGANA),
            0x5c => Some(KEY::HENKAN),
            0x5d => Some(KEY::KATAKANAHIRAGANA),
            0x5e => Some(KEY::MUHENKAN),
            0x5f => Some(KEY::KPJPCOMMA),
            0x60 => Some(KEY::KPENTER),
            0x61 => Some(KEY::RIGHTCTRL),
            0x62 => Some(KEY::KPSLASH),
            0x63 => Some(KEY::SYSRQ),
            0x64 => Some(KEY::RIGHTALT),
            0x65 => Some(KEY::LINEFEED),
            0x66 => Some(KEY::HOME),
            0x67 => Some(KEY::UP),
            0x68 => Some(KEY::PAGEUP),
            0x69 => Some(KEY::LEFT),
            0x6a => Some(KEY::RIGHT),
            0x6b => Some(KEY::END),
            0x6c => Some(KEY::DOWN),
            0x6d => Some(KEY::PAGEDOWN),
            0x6e => Some(KEY::INSERT),
            0x6f => Some(KEY::DELETE),
            0x70 => Some(KEY::MACRO),
            0x71 => Some(KEY::MUTE),
            0x72 => Some(KEY::VOLUMEDOWN),
            0x73 => Some(KEY::VOLUMEUP),
            0x74 => Some(KEY::POWER),
            0x75 => Some(KEY::KPEQUAL),
            0x76 => Some(KEY::KPPLUSMINUS),
            0x77 => Some(KEY::PAUSE),
            0x78 => Some(KEY::SCALE),
            0x79 => Some(KEY::KPCOMMA),
            0x7a => Some(KEY::HANGEUL),
            0x7b => Some(KEY::HANJA),
            0x7c => Some(KEY::YEN),
            0x7d => Some(KEY::LEFTMETA),
            0x7e => Some(KEY::RIGHTMETA),
            0x7f => Some(KEY::COMPOSE),
            0x80 => Some(KEY::STOP),
            0x81 => Some(KEY::AGAIN),
            0x82 => Some(KEY::PROPS),
            0x83 => Some(KEY::UNDO),
            0x84 => Some(KEY::FRONT),
            0x85 => Some(KEY::COPY),
            0x86 => Some(KEY::OPEN),
            0x87 => Some(KEY::PASTE),
            0x88 => Some(KEY::FIND),
            0x89 => Some(KEY::CUT),
            0x8a => Some(KEY::HELP),
            0x8b => Some(KEY::MENU),
            0x8c => Some(KEY::CALC),
            0x8d => Some(KEY::SETUP),
            0x8e => Some(KEY::SLEEP),
            0x8f => Some(KEY::WAKEUP),
            0x90 => Some(KEY::FILE),
            0x91 => Some(KEY::SENDFILE),
            0x92 => Some(KEY::DELETEFILE),
            0x93 => Some(KEY::XFER),
            0x94 => Some(KEY::PROG1),
            0x95 => Some(KEY::PROG2),
            0x96 => Some(KEY::WWW),
            0x97 => Some(KEY::MSDOS),
            0x98 => Some(KEY::COFFEE),
            0x99 => Some(KEY::ROTATE_DISPLAY),
            0x9a => Some(KEY::CYCLEWINDOWS),
            0x9b => Some(KEY::MAIL),
            0x9c => Some(KEY::BOOKMARKS),
            0x9d => Some(KEY::COMPUTER),
            0x9e => Some(KEY::BACK),
            0x9f => Some(KEY::FORWARD),
            0xa0 => Some(KEY::CLOSECD),
            0xa1 => Some(KEY::EJECTCD),
            0xa2 => Some(KEY::EJECTCLOSECD),
            0xa3 => Some(KEY::NEXTSONG),
            0xa4 => Some(KEY::PLAYPAUSE),
            0xa5 => Some(KEY::PREVIOUSSONG),
            0xa6 => Some(KEY::STOPCD),
            0xa7 => Some(KEY::RECORD),
            0xa8 => Some(KEY::REWIND),
            0xa9 => Some(KEY::PHONE),
            0xaa => Some(KEY::ISO),
            0xab => Some(KEY::CONFIG),
            0xac => Some(KEY::HOMEPAGE),
            0xad => Some(KEY::REFRESH),
            0xae => Some(KEY::EXIT),
            0xaf => Some(KEY::MOVE),
            0xb0 => Some(KEY::EDIT),
            0xb1 => Some(KEY::SCROLLUP),
            0xb2 => Some(KEY::SCROLLDOWN),
            0xb3 => Some(KEY::KPLEFTPAREN),
            0xb4 => Some(KEY::KPRIGHTPAREN),
            0xb5 => Some(KEY::NEW),
            0xb6 => Some(KEY::REDO),
            0xb7 => Some(KEY::F13),
            0xb8 => Some(KEY::F14),
            0xb9 => Some(KEY::F15),
            0xba => Some(KEY::F16),
            0xbb => Some(KEY::F17),
            0xbc => Some(KEY::F18),
            0xbd => Some(KEY::F19),
            0xbe => Some(KEY::F20),
            0xbf => Some(KEY::F21),
            0xc0 => Some(KEY::F22),
            0xc1 => Some(KEY::F23),
            0xc2 => Some(KEY::F24),
            0xc8 => Some(KEY::PLAYCD),
            0xc9 => Some(KEY::PAUSECD),
            0xca => Some(KEY::PROG3),
            0xcb => Some(KEY::PROG4),
            0xcc => Some(KEY::DASHBOARD),
            0xcd => Some(KEY::SUSPEND),
            0xce => Some(KEY::CLOSE),
            0xcf => Some(KEY::PLAY),
            0xd0 => Some(KEY::FASTFORWARD),
            0xd1 => Some(KEY::BASSBOOST),
            0xd2 => Some(KEY::PRINT),
            0xd3 => Some(KEY::HP),
            0xd4 => Some(KEY::CAMERA),
            0xd5 => Some(KEY::SOUND),
            0xd6 => Some(KEY::QUESTION),
            0xd7 => Some(KEY::EMAIL),
            0xd8 => Some(KEY::CHAT),
            0xd9 => Some(KEY::SEARCH),
            0xda => Some(KEY::CONNECT),
            0xdb => Some(KEY::FINANCE),
            0xdc => Some(KEY::SPORT),
            0xdd => Some(KEY::SHOP),
            0xde => Some(KEY::ALTERASE),
            0xdf => Some(KEY::CANCEL),
            0xe0 => Some(KEY::BRIGHTNESSDOWN),
            0xe1 => Some(KEY::BRIGHTNESSUP),
            0xe2 => Some(KEY::MEDIA),
            0xe3 => Some(KEY::SWITCHVIDEOMODE),
            0xe4 => Some(KEY::KBDILLUMTOGGLE),
            0xe5 => Some(KEY::KBDILLUMDOWN),
            0xe6 => Some(KEY::KBDILLUMUP),
            0xe7 => Some(KEY::SEND),
            0xe8 => Some(KEY::REPLY),
            0xe9 => Some(KEY::FORWARDMAIL),
            0xea => Some(KEY::SAVE),
            0xeb => Some(KEY::DOCUMENTS),
            0xec => Some(KEY::BATTERY),
            0xed => Some(KEY::BLUETOOTH),
            0xee => Some(KEY::WLAN),
            0xef => Some(KEY::UWB),
            0xf0 => Some(KEY::UNKNOWN),
            0xf1 => Some(KEY::VIDEO_NEXT),
            0xf2 => Some(KEY::VIDEO_PREV),
            0xf3 => Some(KEY::BRIGHTNESS_CYCLE),
            0xf4 => Some(KEY::BRIGHTNESS_AUTO),
            0xf5 => Some(KEY::DISPLAY_OFF),
            0xf6 => Some(KEY::WWAN),
            0xf7 => Some(KEY::RFKILL),
            0xf8 => Some(KEY::MICMUTE),
            0x160 => Some(KEY::OK),
            0x161 => Some(KEY::SELECT),
            0x162 => Some(KEY::GOTO),
            0x163 => Some(KEY::CLEAR),
            0x164 => Some(KEY::POWER2),
            0x165 => Some(KEY::OPTION),
            0x166 => Some(KEY::INFO),
            0x167 => Some(KEY::TIME),
            0x168 => Some(KEY::VENDOR),
            0x169 => Some(KEY::ARCHIVE),
            0x16a => Some(KEY::PROGRAM),
            0x16b => Some(KEY::CHANNEL),
            0x16c => Some(KEY::FAVORITES),
            0x16d => Some(KEY::EPG),
            0x16e => Some(KEY::PVR),
            0x16f => Some(KEY::MHP),
            0x170 => Some(KEY::LANGUAGE),
            0x171 => Some(KEY::TITLE),
            0x172 => Some(KEY::SUBTITLE),
            0x173 => Some(KEY::ANGLE),
            0x174 => Some(KEY::ZOOM),
            0x175 => Some(KEY::MODE),
            0x176 => Some(KEY::KEYBOARD),
            0x177 => Some(KEY::SCREEN),
            0x178 => Some(KEY::PC),
            0x179 => Some(KEY::TV),
            0x17a => Some(KEY::TV2),
            0x17b => Some(KEY::VCR),
            0x17c => Some(KEY::VCR2),
            0x17d => Some(KEY::SAT),
            0x17e => Some(KEY::SAT2),
            0x17f => Some(KEY::CD),
            0x180 => Some(KEY::TAPE),
            0x181 => Some(KEY::RADIO),
            0x182 => Some(KEY::TUNER),
            0x183 => Some(KEY::PLAYER),
            0x184 => Some(KEY::TEXT),
            0x185 => Some(KEY::DVD),
            0x186 => Some(KEY::AUX),
            0x187 => Some(KEY::MP3),
            0x188 => Some(KEY::AUDIO),
            0x189 => Some(KEY::VIDEO),
            0x18a => Some(KEY::DIRECTORY),
            0x18b => Some(KEY::LIST),
            0x18c => Some(KEY::MEMO),
            0x18d => Some(KEY::CALENDAR),
            0x18e => Some(KEY::RED),
            0x18f => Some(KEY::GREEN),
            0x190 => Some(KEY::YELLOW),
            0x191 => Some(KEY::BLUE),
            0x192 => Some(KEY::CHANNELUP),
            0x193 => Some(KEY::CHANNELDOWN),
            0x194 => Some(KEY::FIRST),
            0x195 => Some(KEY::LAST),
            0x196 => Some(KEY::AB),
            0x197 => Some(KEY::NEXT),
            0x198 => Some(KEY::RESTART),
            0x199 => Some(KEY::SLOW),
            0x19a => Some(KEY::SHUFFLE),
            0x19b => Some(KEY::BREAK),
            0x19c => Some(KEY::PREVIOUS),
            0x19d => Some(KEY::DIGITS),
            0x19e => Some(KEY::TEEN),
            0x19f => Some(KEY::TWEN),
            0x1a0 => Some(KEY::VIDEOPHONE),
            0x1a1 => Some(KEY::GAMES),
            0x1a2 => Some(KEY::ZOOMIN),
            0x1a3 => Some(KEY::ZOOMOUT),
            0x1a4 => Some(KEY::ZOOMRESET),
            0x1a5 => Some(KEY::WORDPROCESSOR),
            0x1a6 => Some(KEY::EDITOR),
            0x1a7 => Some(KEY::SPREADSHEET),
            0x1a8 => Some(KEY::GRAPHICSEDITOR),
            0x1a9 => Some(KEY::PRESENTATION),
            0x1aa => Some(KEY::DATABASE),
            0x1ab => Some(KEY::NEWS),
            0x1ac => Some(KEY::VOICEMAIL),
            0x1ad => Some(KEY::ADDRESSBOOK),
            0x1ae => Some(KEY::MESSENGER),
            0x1af => Some(KEY::DISPLAYTOGGLE),
            0x1b0 => Some(KEY::SPELLCHECK),
            0x1b1 => Some(KEY::LOGOFF),
            0x1b2 => Some(KEY::DOLLAR),
            0x1b3 => Some(KEY::EURO),
            0x1b4 => Some(KEY::FRAMEBACK),
            0x1b5 => Some(KEY::FRAMEFORWARD),
            0x1b6 => Some(KEY::CONTEXT_MENU),
            0x1b7 => Some(KEY::MEDIA_REPEAT),
            0x1b8 => Some(KEY::KEY_10CHANNELSUP),
            0x1b9 => Some(KEY::KEY_10CHANNELSDOWN),
            0x1ba => Some(KEY::IMAGES),
            0x1c0 => Some(KEY::DEL_EOL),
            0x1c1 => Some(KEY::DEL_EOS),
            0x1c2 => Some(KEY::INS_LINE),
            0x1c3 => Some(KEY::DEL_LINE),
            0x1d0 => Some(KEY::FN),
            0x1d1 => Some(KEY::FN_ESC),
            0x1d2 => Some(KEY::FN_F1),
            0x1d3 => Some(KEY::FN_F2),
            0x1d4 => Some(KEY::FN_F3),
            0x1d5 => Some(KEY::FN_F4),
            0x1d6 => Some(KEY::FN_F5),
            0x1d7 => Some(KEY::FN_F6),
            0x1d8 => Some(KEY::FN_F7),
            0x1d9 => Some(KEY::FN_F8),
            0x1da => Some(KEY::FN_F9),
            0x1db => Some(KEY::FN_F10),
            0x1dc => Some(KEY::FN_F11),
            0x1dd => Some(KEY::FN_F12),
            0x1de => Some(KEY::FN_1),
            0x1df => Some(KEY::FN_2),
            0x1e0 => Some(KEY::FN_D),
            0x1e1 => Some(KEY::FN_E),
            0x1e2 => Some(KEY::FN_F),
            0x1e3 => Some(KEY::FN_S),
            0x1e4 => Some(KEY::FN_B),
            0x1f1 => Some(KEY::BRL_DOT1),
            0x1f2 => Some(KEY::BRL_DOT2),
            0x1f3 => Some(KEY::BRL_DOT3),
            0x1f4 => Some(KEY::BRL_DOT4),
            0x1f5 => Some(KEY::BRL_DOT5),
            0x1f6 => Some(KEY::BRL_DOT6),
            0x1f7 => Some(KEY::BRL_DOT7),
            0x1f8 => Some(KEY::BRL_DOT8),
            0x1f9 => Some(KEY::BRL_DOT9),
            0x1fa => Some(KEY::BRL_DOT10),
            0x200 => Some(KEY::NUMERIC_0),
            0x201 => Some(KEY::NUMERIC_1),
            0x202 => Some(KEY::NUMERIC_2),
            0x203 => Some(KEY::NUMERIC_3),
            0x204 => Some(KEY::NUMERIC_4),
            0x205 => Some(KEY::NUMERIC_5),
            0x206 => Some(KEY::NUMERIC_6),
            0x207 => Some(KEY::NUMERIC_7),
            0x208 => Some(KEY::NUMERIC_8),
            0x209 => Some(KEY::NUMERIC_9),
            0x20a => Some(KEY::NUMERIC_STAR),
            0x20b => Some(KEY::NUMERIC_POUND),
            0x20c => Some(KEY::NUMERIC_A),
            0x20d => Some(KEY::NUMERIC_B),
            0x20e => Some(KEY::NUMERIC_C),
            0x20f => Some(KEY::NUMERIC_D),
            0x210 => Some(KEY::CAMERA_FOCUS),
            0x211 => Some(KEY::WPS_BUTTON),
            0x212 => Some(KEY::TOUCHPAD_TOGGLE),
            0x213 => Some(KEY::TOUCHPAD_ON),
            0x214 => Some(KEY::TOUCHPAD_OFF),
            0x215 => Some(KEY::CAMERA_ZOOMIN),
            0x216 => Some(KEY::CAMERA_ZOOMOUT),
            0x217 => Some(KEY::CAMERA_UP),
            0x218 => Some(KEY::CAMERA_DOWN),
            0x219 => Some(KEY::CAMERA_LEFT),
            0x21a => Some(KEY::CAMERA_RIGHT),
            0x21b => Some(KEY::ATTENDANT_ON),
            0x21c => Some(KEY::ATTENDANT_OFF),
            0x21d => Some(KEY::ATTENDANT_TOGGLE),
            0x21e => Some(KEY::LIGHTS_TOGGLE),
            0x230 => Some(KEY::ALS_TOGGLE),
            0x231 => Some(KEY::ROTATE_LOCK_TOGGLE),
            0x240 => Some(KEY::BUTTONCONFIG),
            0x241 => Some(KEY::TASKMANAGER),
            0x242 => Some(KEY::JOURNAL),
            0x243 => Some(KEY::CONTROLPANEL),
            0x244 => Some(KEY::APPSELECT),
            0x245 => Some(KEY::SCREENSAVER),
            0x246 => Some(KEY::VOICECOMMAND),
            0x247 => Some(KEY::ASSISTANT),
            0x250 => Some(KEY::BRIGHTNESS_MIN),
            0x251 => Some(KEY::BRIGHTNESS_MAX),
            0x260 => Some(KEY::KBDINPUTASSIST_PREV),
            0x261 => Some(KEY::KBDINPUTASSIST_NEXT),
            0x262 => Some(KEY::KBDINPUTASSIST_PREVGROUP),
            0x263 => Some(KEY::KBDINPUTASSIST_NEXTGROUP),
            0x264 => Some(KEY::KBDINPUTASSIST_ACCEPT),
            0x265 => Some(KEY::KBDINPUTASSIST_CANCEL),
            0x266 => Some(KEY::RIGHT_UP),
            0x267 => Some(KEY::RIGHT_DOWN),
            0x268 => Some(KEY::LEFT_UP),
            0x269 => Some(KEY::LEFT_DOWN),
            0x26a => Some(KEY::ROOT_MENU),
            0x26b => Some(KEY::MEDIA_TOP_MENU),
            0x26c => Some(KEY::NUMERIC_11),
            0x26d => Some(KEY::NUMERIC_12),
            0x26e => Some(KEY::AUDIO_DESC),
            0x26f => Some(KEY::KEY_3D_MODE),
            0x270 => Some(KEY::NEXT_FAVORITE),
            0x271 => Some(KEY::STOP_RECORD),
            0x272 => Some(KEY::PAUSE_RECORD),
            0x273 => Some(KEY::VOD),
            0x274 => Some(KEY::UNMUTE),
            0x275 => Some(KEY::FASTREVERSE),
            0x276 => Some(KEY::SLOWREVERSE),
            0x277 => Some(KEY::DATA),
            0x278 => Some(KEY::ONSCREEN_KEYBOARD),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        767
    }
}

impl AsCodeRaw for KEY {
    fn as_code_raw(&self) -> CodeRaw {
        (0x1, *self as CodeValRaw)
    }
}

impl AsCode for KEY {
    fn as_code(&self) -> Code {
        Code::KEY(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BTN {
    BTN_0 = 0x100,
    BTN_1 = 0x101,
    BTN_2 = 0x102,
    BTN_3 = 0x103,
    BTN_4 = 0x104,
    BTN_5 = 0x105,
    BTN_6 = 0x106,
    BTN_7 = 0x107,
    BTN_8 = 0x108,
    BTN_9 = 0x109,
    LEFT = 0x110,
    RIGHT = 0x111,
    MIDDLE = 0x112,
    SIDE = 0x113,
    EXTRA = 0x114,
    FORWARD = 0x115,
    BACK = 0x116,
    TASK = 0x117,
    TRIGGER = 0x120,
    THUMB = 0x121,
    THUMB2 = 0x122,
    TOP = 0x123,
    TOP2 = 0x124,
    PINKIE = 0x125,
    BASE = 0x126,
    BASE2 = 0x127,
    BASE3 = 0x128,
    BASE4 = 0x129,
    BASE5 = 0x12a,
    BASE6 = 0x12b,
    DEAD = 0x12f,
    SOUTH = 0x130,
    EAST = 0x131,
    C = 0x132,
    NORTH = 0x133,
    WEST = 0x134,
    Z = 0x135,
    TL = 0x136,
    TR = 0x137,
    TL2 = 0x138,
    TR2 = 0x139,
    SELECT = 0x13a,
    START = 0x13b,
    MODE = 0x13c,
    THUMBL = 0x13d,
    THUMBR = 0x13e,
    TOOL_PEN = 0x140,
    TOOL_RUBBER = 0x141,
    TOOL_BRUSH = 0x142,
    TOOL_PENCIL = 0x143,
    TOOL_AIRBRUSH = 0x144,
    TOOL_FINGER = 0x145,
    TOOL_MOUSE = 0x146,
    TOOL_LENS = 0x147,
    TOOL_QUINTTAP = 0x148,
    STYLUS3 = 0x149,
    TOUCH = 0x14a,
    STYLUS = 0x14b,
    STYLUS2 = 0x14c,
    TOOL_DOUBLETAP = 0x14d,
    TOOL_TRIPLETAP = 0x14e,
    TOOL_QUADTAP = 0x14f,
    GEAR_DOWN = 0x150,
    GEAR_UP = 0x151,
    DPAD_UP = 0x220,
    DPAD_DOWN = 0x221,
    DPAD_LEFT = 0x222,
    DPAD_RIGHT = 0x223,
    TRIGGER_HAPPY1 = 0x2c0,
    TRIGGER_HAPPY2 = 0x2c1,
    TRIGGER_HAPPY3 = 0x2c2,
    TRIGGER_HAPPY4 = 0x2c3,
    TRIGGER_HAPPY5 = 0x2c4,
    TRIGGER_HAPPY6 = 0x2c5,
    TRIGGER_HAPPY7 = 0x2c6,
    TRIGGER_HAPPY8 = 0x2c7,
    TRIGGER_HAPPY9 = 0x2c8,
    TRIGGER_HAPPY10 = 0x2c9,
    TRIGGER_HAPPY11 = 0x2ca,
    TRIGGER_HAPPY12 = 0x2cb,
    TRIGGER_HAPPY13 = 0x2cc,
    TRIGGER_HAPPY14 = 0x2cd,
    TRIGGER_HAPPY15 = 0x2ce,
    TRIGGER_HAPPY16 = 0x2cf,
    TRIGGER_HAPPY17 = 0x2d0,
    TRIGGER_HAPPY18 = 0x2d1,
    TRIGGER_HAPPY19 = 0x2d2,
    TRIGGER_HAPPY20 = 0x2d3,
    TRIGGER_HAPPY21 = 0x2d4,
    TRIGGER_HAPPY22 = 0x2d5,
    TRIGGER_HAPPY23 = 0x2d6,
    TRIGGER_HAPPY24 = 0x2d7,
    TRIGGER_HAPPY25 = 0x2d8,
    TRIGGER_HAPPY26 = 0x2d9,
    TRIGGER_HAPPY27 = 0x2da,
    TRIGGER_HAPPY28 = 0x2db,
    TRIGGER_HAPPY29 = 0x2dc,
    TRIGGER_HAPPY30 = 0x2dd,
    TRIGGER_HAPPY31 = 0x2de,
    TRIGGER_HAPPY32 = 0x2df,
    TRIGGER_HAPPY33 = 0x2e0,
    TRIGGER_HAPPY34 = 0x2e1,
    TRIGGER_HAPPY35 = 0x2e2,
    TRIGGER_HAPPY36 = 0x2e3,
    TRIGGER_HAPPY37 = 0x2e4,
    TRIGGER_HAPPY38 = 0x2e5,
    TRIGGER_HAPPY39 = 0x2e6,
    TRIGGER_HAPPY40 = 0x2e7,
}

impl BTN {
    pub fn from_raw(codeval: CodeValRaw) -> Option<BTN> {
        match codeval {
            0x100 => Some(BTN::BTN_0),
            0x101 => Some(BTN::BTN_1),
            0x102 => Some(BTN::BTN_2),
            0x103 => Some(BTN::BTN_3),
            0x104 => Some(BTN::BTN_4),
            0x105 => Some(BTN::BTN_5),
            0x106 => Some(BTN::BTN_6),
            0x107 => Some(BTN::BTN_7),
            0x108 => Some(BTN::BTN_8),
            0x109 => Some(BTN::BTN_9),
            0x110 => Some(BTN::LEFT),
            0x111 => Some(BTN::RIGHT),
            0x112 => Some(BTN::MIDDLE),
            0x113 => Some(BTN::SIDE),
            0x114 => Some(BTN::EXTRA),
            0x115 => Some(BTN::FORWARD),
            0x116 => Some(BTN::BACK),
            0x117 => Some(BTN::TASK),
            0x120 => Some(BTN::TRIGGER),
            0x121 => Some(BTN::THUMB),
            0x122 => Some(BTN::THUMB2),
            0x123 => Some(BTN::TOP),
            0x124 => Some(BTN::TOP2),
            0x125 => Some(BTN::PINKIE),
            0x126 => Some(BTN::BASE),
            0x127 => Some(BTN::BASE2),
            0x128 => Some(BTN::BASE3),
            0x129 => Some(BTN::BASE4),
            0x12a => Some(BTN::BASE5),
            0x12b => Some(BTN::BASE6),
            0x12f => Some(BTN::DEAD),
            0x130 => Some(BTN::SOUTH),
            0x131 => Some(BTN::EAST),
            0x132 => Some(BTN::C),
            0x133 => Some(BTN::NORTH),
            0x134 => Some(BTN::WEST),
            0x135 => Some(BTN::Z),
            0x136 => Some(BTN::TL),
            0x137 => Some(BTN::TR),
            0x138 => Some(BTN::TL2),
            0x139 => Some(BTN::TR2),
            0x13a => Some(BTN::SELECT),
            0x13b => Some(BTN::START),
            0x13c => Some(BTN::MODE),
            0x13d => Some(BTN::THUMBL),
            0x13e => Some(BTN::THUMBR),
            0x140 => Some(BTN::TOOL_PEN),
            0x141 => Some(BTN::TOOL_RUBBER),
            0x142 => Some(BTN::TOOL_BRUSH),
            0x143 => Some(BTN::TOOL_PENCIL),
            0x144 => Some(BTN::TOOL_AIRBRUSH),
            0x145 => Some(BTN::TOOL_FINGER),
            0x146 => Some(BTN::TOOL_MOUSE),
            0x147 => Some(BTN::TOOL_LENS),
            0x148 => Some(BTN::TOOL_QUINTTAP),
            0x149 => Some(BTN::STYLUS3),
            0x14a => Some(BTN::TOUCH),
            0x14b => Some(BTN::STYLUS),
            0x14c => Some(BTN::STYLUS2),
            0x14d => Some(BTN::TOOL_DOUBLETAP),
            0x14e => Some(BTN::TOOL_TRIPLETAP),
            0x14f => Some(BTN::TOOL_QUADTAP),
            0x150 => Some(BTN::GEAR_DOWN),
            0x151 => Some(BTN::GEAR_UP),
            0x220 => Some(BTN::DPAD_UP),
            0x221 => Some(BTN::DPAD_DOWN),
            0x222 => Some(BTN::DPAD_LEFT),
            0x223 => Some(BTN::DPAD_RIGHT),
            0x2c0 => Some(BTN::TRIGGER_HAPPY1),
            0x2c1 => Some(BTN::TRIGGER_HAPPY2),
            0x2c2 => Some(BTN::TRIGGER_HAPPY3),
            0x2c3 => Some(BTN::TRIGGER_HAPPY4),
            0x2c4 => Some(BTN::TRIGGER_HAPPY5),
            0x2c5 => Some(BTN::TRIGGER_HAPPY6),
            0x2c6 => Some(BTN::TRIGGER_HAPPY7),
            0x2c7 => Some(BTN::TRIGGER_HAPPY8),
            0x2c8 => Some(BTN::TRIGGER_HAPPY9),
            0x2c9 => Some(BTN::TRIGGER_HAPPY10),
            0x2ca => Some(BTN::TRIGGER_HAPPY11),
            0x2cb => Some(BTN::TRIGGER_HAPPY12),
            0x2cc => Some(BTN::TRIGGER_HAPPY13),
            0x2cd => Some(BTN::TRIGGER_HAPPY14),
            0x2ce => Some(BTN::TRIGGER_HAPPY15),
            0x2cf => Some(BTN::TRIGGER_HAPPY16),
            0x2d0 => Some(BTN::TRIGGER_HAPPY17),
            0x2d1 => Some(BTN::TRIGGER_HAPPY18),
            0x2d2 => Some(BTN::TRIGGER_HAPPY19),
            0x2d3 => Some(BTN::TRIGGER_HAPPY20),
            0x2d4 => Some(BTN::TRIGGER_HAPPY21),
            0x2d5 => Some(BTN::TRIGGER_HAPPY22),
            0x2d6 => Some(BTN::TRIGGER_HAPPY23),
            0x2d7 => Some(BTN::TRIGGER_HAPPY24),
            0x2d8 => Some(BTN::TRIGGER_HAPPY25),
            0x2d9 => Some(BTN::TRIGGER_HAPPY26),
            0x2da => Some(BTN::TRIGGER_HAPPY27),
            0x2db => Some(BTN::TRIGGER_HAPPY28),
            0x2dc => Some(BTN::TRIGGER_HAPPY29),
            0x2dd => Some(BTN::TRIGGER_HAPPY30),
            0x2de => Some(BTN::TRIGGER_HAPPY31),
            0x2df => Some(BTN::TRIGGER_HAPPY32),
            0x2e0 => Some(BTN::TRIGGER_HAPPY33),
            0x2e1 => Some(BTN::TRIGGER_HAPPY34),
            0x2e2 => Some(BTN::TRIGGER_HAPPY35),
            0x2e3 => Some(BTN::TRIGGER_HAPPY36),
            0x2e4 => Some(BTN::TRIGGER_HAPPY37),
            0x2e5 => Some(BTN::TRIGGER_HAPPY38),
            0x2e6 => Some(BTN::TRIGGER_HAPPY39),
            0x2e7 => Some(BTN::TRIGGER_HAPPY40),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
}

impl AsCodeRaw for BTN {
    fn as_code_raw(&self) -> CodeRaw {
        (0x1, *self as CodeValRaw)
    }
}

impl AsCode for BTN {
    fn as_code(&self) -> Code {
        Code::KEY(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum REL {
    X = 0x0,
    Y = 0x1,
    Z = 0x2,
    RX = 0x3,
    RY = 0x4,
    RZ = 0x5,
    HWHEEL = 0x6,
    DIAL = 0x7,
    WHEEL = 0x8,
    MISC = 0x9,
    RESERVED = 0xa,
}

impl REL {
    pub fn from_raw(codeval: CodeValRaw) -> Option<REL> {
        match codeval {
            0x0 => Some(REL::X),
            0x1 => Some(REL::Y),
            0x2 => Some(REL::Z),
            0x3 => Some(REL::RX),
            0x4 => Some(REL::RY),
            0x5 => Some(REL::RZ),
            0x6 => Some(REL::HWHEEL),
            0x7 => Some(REL::DIAL),
            0x8 => Some(REL::WHEEL),
            0x9 => Some(REL::MISC),
            0xa => Some(REL::RESERVED),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        15
    }
}

impl AsCodeRaw for REL {
    fn as_code_raw(&self) -> CodeRaw {
        (0x2, *self as CodeValRaw)
    }
}

impl AsCode for REL {
    fn as_code(&self) -> Code {
        Code::REL(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ABS {
    X = 0x0,
    Y = 0x1,
    Z = 0x2,
    RX = 0x3,
    RY = 0x4,
    RZ = 0x5,
    THROTTLE = 0x6,
    RUDDER = 0x7,
    WHEEL = 0x8,
    GAS = 0x9,
    BRAKE = 0xa,
    HAT0X = 0x10,
    HAT0Y = 0x11,
    HAT1X = 0x12,
    HAT1Y = 0x13,
    HAT2X = 0x14,
    HAT2Y = 0x15,
    HAT3X = 0x16,
    HAT3Y = 0x17,
    PRESSURE = 0x18,
    DISTANCE = 0x19,
    TILT_X = 0x1a,
    TILT_Y = 0x1b,
    TOOL_WIDTH = 0x1c,
    VOLUME = 0x20,
    MISC = 0x28,
    RESERVED = 0x2e,
    MT_SLOT = 0x2f,
    MT_TOUCH_MAJOR = 0x30,
    MT_TOUCH_MINOR = 0x31,
    MT_WIDTH_MAJOR = 0x32,
    MT_WIDTH_MINOR = 0x33,
    MT_ORIENTATION = 0x34,
    MT_POSITION_X = 0x35,
    MT_POSITION_Y = 0x36,
    MT_TOOL_TYPE = 0x37,
    MT_BLOB_ID = 0x38,
    MT_TRACKING_ID = 0x39,
    MT_PRESSURE = 0x3a,
    MT_DISTANCE = 0x3b,
    MT_TOOL_X = 0x3c,
    MT_TOOL_Y = 0x3d,
}

impl ABS {
    pub fn from_raw(codeval: CodeValRaw) -> Option<ABS> {
        match codeval {
            0x0 => Some(ABS::X),
            0x1 => Some(ABS::Y),
            0x2 => Some(ABS::Z),
            0x3 => Some(ABS::RX),
            0x4 => Some(ABS::RY),
            0x5 => Some(ABS::RZ),
            0x6 => Some(ABS::THROTTLE),
            0x7 => Some(ABS::RUDDER),
            0x8 => Some(ABS::WHEEL),
            0x9 => Some(ABS::GAS),
            0xa => Some(ABS::BRAKE),
            0x10 => Some(ABS::HAT0X),
            0x11 => Some(ABS::HAT0Y),
            0x12 => Some(ABS::HAT1X),
            0x13 => Some(ABS::HAT1Y),
            0x14 => Some(ABS::HAT2X),
            0x15 => Some(ABS::HAT2Y),
            0x16 => Some(ABS::HAT3X),
            0x17 => Some(ABS::HAT3Y),
            0x18 => Some(ABS::PRESSURE),
            0x19 => Some(ABS::DISTANCE),
            0x1a => Some(ABS::TILT_X),
            0x1b => Some(ABS::TILT_Y),
            0x1c => Some(ABS::TOOL_WIDTH),
            0x20 => Some(ABS::VOLUME),
            0x28 => Some(ABS::MISC),
            0x2e => Some(ABS::RESERVED),
            0x2f => Some(ABS::MT_SLOT),
            0x30 => Some(ABS::MT_TOUCH_MAJOR),
            0x31 => Some(ABS::MT_TOUCH_MINOR),
            0x32 => Some(ABS::MT_WIDTH_MAJOR),
            0x33 => Some(ABS::MT_WIDTH_MINOR),
            0x34 => Some(ABS::MT_ORIENTATION),
            0x35 => Some(ABS::MT_POSITION_X),
            0x36 => Some(ABS::MT_POSITION_Y),
            0x37 => Some(ABS::MT_TOOL_TYPE),
            0x38 => Some(ABS::MT_BLOB_ID),
            0x39 => Some(ABS::MT_TRACKING_ID),
            0x3a => Some(ABS::MT_PRESSURE),
            0x3b => Some(ABS::MT_DISTANCE),
            0x3c => Some(ABS::MT_TOOL_X),
            0x3d => Some(ABS::MT_TOOL_Y),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        63
    }
}

impl AsCodeRaw for ABS {
    fn as_code_raw(&self) -> CodeRaw {
        (0x3, *self as CodeValRaw)
    }
}

impl AsCode for ABS {
    fn as_code(&self) -> Code {
        Code::ABS(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MSC {
    SERIAL = 0x0,
    PULSELED = 0x1,
    GESTURE = 0x2,
    RAW = 0x3,
    SCAN = 0x4,
    TIMESTAMP = 0x5,
}

impl MSC {
    pub fn from_raw(codeval: CodeValRaw) -> Option<MSC> {
        match codeval {
            0x0 => Some(MSC::SERIAL),
            0x1 => Some(MSC::PULSELED),
            0x2 => Some(MSC::GESTURE),
            0x3 => Some(MSC::RAW),
            0x4 => Some(MSC::SCAN),
            0x5 => Some(MSC::TIMESTAMP),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        7
    }
}

impl AsCodeRaw for MSC {
    fn as_code_raw(&self) -> CodeRaw {
        (0x4, *self as CodeValRaw)
    }
}

impl AsCode for MSC {
    fn as_code(&self) -> Code {
        Code::MSC(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SW {
    LID = 0x0,
    TABLET_MODE = 0x1,
    HEADPHONE_INSERT = 0x2,
    RFKILL_ALL = 0x3,
    MICROPHONE_INSERT = 0x4,
    DOCK = 0x5,
    LINEOUT_INSERT = 0x6,
    JACK_PHYSICAL_INSERT = 0x7,
    VIDEOOUT_INSERT = 0x8,
    CAMERA_LENS_COVER = 0x9,
    KEYPAD_SLIDE = 0xa,
    FRONT_PROXIMITY = 0xb,
    ROTATE_LOCK = 0xc,
    LINEIN_INSERT = 0xd,
    MUTE_DEVICE = 0xe,
    PEN_INSERTED = 0xf,
}

impl SW {
    pub fn from_raw(codeval: CodeValRaw) -> Option<SW> {
        match codeval {
            0x0 => Some(SW::LID),
            0x1 => Some(SW::TABLET_MODE),
            0x2 => Some(SW::HEADPHONE_INSERT),
            0x3 => Some(SW::RFKILL_ALL),
            0x4 => Some(SW::MICROPHONE_INSERT),
            0x5 => Some(SW::DOCK),
            0x6 => Some(SW::LINEOUT_INSERT),
            0x7 => Some(SW::JACK_PHYSICAL_INSERT),
            0x8 => Some(SW::VIDEOOUT_INSERT),
            0x9 => Some(SW::CAMERA_LENS_COVER),
            0xa => Some(SW::KEYPAD_SLIDE),
            0xb => Some(SW::FRONT_PROXIMITY),
            0xc => Some(SW::ROTATE_LOCK),
            0xd => Some(SW::LINEIN_INSERT),
            0xe => Some(SW::MUTE_DEVICE),
            0xf => Some(SW::PEN_INSERTED),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
}

impl AsCodeRaw for SW {
    fn as_code_raw(&self) -> CodeRaw {
        (0x5, *self as CodeValRaw)
    }
}

impl AsCode for SW {
    fn as_code(&self) -> Code {
        Code::SW(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LED {
    NUML = 0x0,
    CAPSL = 0x1,
    SCROLLL = 0x2,
    COMPOSE = 0x3,
    KANA = 0x4,
    SLEEP = 0x5,
    SUSPEND = 0x6,
    MUTE = 0x7,
    MISC = 0x8,
    MAIL = 0x9,
    CHARGING = 0xa,
}

impl LED {
    pub fn from_raw(codeval: CodeValRaw) -> Option<LED> {
        match codeval {
            0x0 => Some(LED::NUML),
            0x1 => Some(LED::CAPSL),
            0x2 => Some(LED::SCROLLL),
            0x3 => Some(LED::COMPOSE),
            0x4 => Some(LED::KANA),
            0x5 => Some(LED::SLEEP),
            0x6 => Some(LED::SUSPEND),
            0x7 => Some(LED::MUTE),
            0x8 => Some(LED::MISC),
            0x9 => Some(LED::MAIL),
            0xa => Some(LED::CHARGING),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        15
    }
}

impl AsCodeRaw for LED {
    fn as_code_raw(&self) -> CodeRaw {
        (0x11, *self as CodeValRaw)
    }
}

impl AsCode for LED {
    fn as_code(&self) -> Code {
        Code::LED(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SND {
    CLICK = 0x0,
    BELL = 0x1,
    TONE = 0x2,
}

impl SND {
    pub fn from_raw(codeval: CodeValRaw) -> Option<SND> {
        match codeval {
            0x0 => Some(SND::CLICK),
            0x1 => Some(SND::BELL),
            0x2 => Some(SND::TONE),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        7
    }
}

impl AsCodeRaw for SND {
    fn as_code_raw(&self) -> CodeRaw {
        (0x12, *self as CodeValRaw)
    }
}

impl AsCode for SND {
    fn as_code(&self) -> Code {
        Code::SND(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum REP {
    DELAY = 0x0,
    PERIOD = 0x1,
}

impl REP {
    pub fn from_raw(codeval: CodeValRaw) -> Option<REP> {
        match codeval {
            0x0 => Some(REP::DELAY),
            0x1 => Some(REP::PERIOD),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
}

impl AsCodeRaw for REP {
    fn as_code_raw(&self) -> CodeRaw {
        (0x14, *self as CodeValRaw)
    }
}

impl AsCode for REP {
    fn as_code(&self) -> Code {
        Code::REP(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FF {
    RUMBLE = 0x50,
    PERIODIC = 0x51,
    CONSTANT = 0x52,
    SPRING = 0x53,
    FRICTION = 0x54,
    DAMPER = 0x55,
    INERTIA = 0x56,
    RAMP = 0x57,
    SQUARE = 0x58,
    TRIANGLE = 0x59,
    SINE = 0x5a,
    SAW_UP = 0x5b,
    SAW_DOWN = 0x5c,
    CUSTOM = 0x5d,
    GAIN = 0x60,
    AUTOCENTER = 0x61,
}

impl FF {
    pub fn from_raw(codeval: CodeValRaw) -> Option<FF> {
        match codeval {
            0x50 => Some(FF::RUMBLE),
            0x51 => Some(FF::PERIODIC),
            0x52 => Some(FF::CONSTANT),
            0x53 => Some(FF::SPRING),
            0x54 => Some(FF::FRICTION),
            0x55 => Some(FF::DAMPER),
            0x56 => Some(FF::INERTIA),
            0x57 => Some(FF::RAMP),
            0x58 => Some(FF::SQUARE),
            0x59 => Some(FF::TRIANGLE),
            0x5a => Some(FF::SINE),
            0x5b => Some(FF::SAW_UP),
            0x5c => Some(FF::SAW_DOWN),
            0x5d => Some(FF::CUSTOM),
            0x60 => Some(FF::GAIN),
            0x61 => Some(FF::AUTOCENTER),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        127
    }
}

impl AsCodeRaw for FF {
    fn as_code_raw(&self) -> CodeRaw {
        (0x15, *self as CodeValRaw)
    }
}

impl AsCode for FF {
    fn as_code(&self) -> Code {
        Code::FF(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FF_STATUS {
    STOPPED = 0x0,
    PLAYING = 0x1,
}

impl FF_STATUS {
    pub fn from_raw(codeval: CodeValRaw) -> Option<FF_STATUS> {
        match codeval {
            0x0 => Some(FF_STATUS::STOPPED),
            0x1 => Some(FF_STATUS::PLAYING),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        1
    }
}

impl AsCodeRaw for FF_STATUS {
    fn as_code_raw(&self) -> CodeRaw {
        (0x17, *self as CodeValRaw)
    }
}

impl AsCode for FF_STATUS {
    fn as_code(&self) -> Code {
        Code::FF_STATUS(*self as CodeValRaw)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum INPUT_PROP {
    POINTER = 0x0,
    DIRECT = 0x1,
    BUTTONPAD = 0x2,
    SEMI_MT = 0x3,
    TOPBUTTONPAD = 0x4,
    POINTING_STICK = 0x5,
    ACCELEROMETER = 0x6,
}

impl INPUT_PROP {
    pub fn from_raw(codeval: CodeValRaw) -> Option<INPUT_PROP> {
        match codeval {
            0x0 => Some(INPUT_PROP::POINTER),
            0x1 => Some(INPUT_PROP::DIRECT),
            0x2 => Some(INPUT_PROP::BUTTONPAD),
            0x3 => Some(INPUT_PROP::SEMI_MT),
            0x4 => Some(INPUT_PROP::TOPBUTTONPAD),
            0x5 => Some(INPUT_PROP::POINTING_STICK),
            0x6 => Some(INPUT_PROP::ACCELEROMETER),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
    pub fn max() -> CodeValRaw {
        31
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BUS {
    PCI = 0x1,
    ISAPNP = 0x2,
    USB = 0x3,
    HIL = 0x4,
    BLUETOOTH = 0x5,
    VIRTUAL = 0x6,
    ISA = 0x10,
    I8042 = 0x11,
    XTKBD = 0x12,
    RS232 = 0x13,
    GAMEPORT = 0x14,
    PARPORT = 0x15,
    AMIGA = 0x16,
    ADB = 0x17,
    I2C = 0x18,
    HOST = 0x19,
    GSC = 0x1a,
    ATARI = 0x1b,
    SPI = 0x1c,
    RMI = 0x1d,
    CEC = 0x1e,
    INTEL_ISHTP = 0x1f,
}

impl BUS {
    pub fn from_raw(codeval: CodeValRaw) -> Option<BUS> {
        match codeval {
            0x1 => Some(BUS::PCI),
            0x2 => Some(BUS::ISAPNP),
            0x3 => Some(BUS::USB),
            0x4 => Some(BUS::HIL),
            0x5 => Some(BUS::BLUETOOTH),
            0x6 => Some(BUS::VIRTUAL),
            0x10 => Some(BUS::ISA),
            0x11 => Some(BUS::I8042),
            0x12 => Some(BUS::XTKBD),
            0x13 => Some(BUS::RS232),
            0x14 => Some(BUS::GAMEPORT),
            0x15 => Some(BUS::PARPORT),
            0x16 => Some(BUS::AMIGA),
            0x17 => Some(BUS::ADB),
            0x18 => Some(BUS::I2C),
            0x19 => Some(BUS::HOST),
            0x1a => Some(BUS::GSC),
            0x1b => Some(BUS::ATARI),
            0x1c => Some(BUS::SPI),
            0x1d => Some(BUS::RMI),
            0x1e => Some(BUS::CEC),
            0x1f => Some(BUS::INTEL_ISHTP),
            _ => None,
        }
    }
    pub fn as_raw(&self) -> CodeValRaw {
        *self as CodeValRaw
    }
}

