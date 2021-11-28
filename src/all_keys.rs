use evdev::{AttributeSet, Key};

pub fn all_keys() -> AttributeSet<Key> {
    let mut keys = AttributeSet::new();
    for key in KEYS {
        keys.insert(*key);
    }
    keys
}

const KEYS: &[Key] = &[
    Key::KEY_RESERVED,
    Key::KEY_ESC,
    Key::KEY_1,
    Key::KEY_2,
    Key::KEY_3,
    Key::KEY_4,
    Key::KEY_5,
    Key::KEY_6,
    Key::KEY_7,
    Key::KEY_8,
    Key::KEY_9,
    Key::KEY_0,
    Key::KEY_MINUS,
    Key::KEY_EQUAL,
    Key::KEY_BACKSPACE,
    Key::KEY_TAB,
    Key::KEY_Q,
    Key::KEY_W,
    Key::KEY_E,
    Key::KEY_R,
    Key::KEY_T,
    Key::KEY_Y,
    Key::KEY_U,
    Key::KEY_I,
    Key::KEY_O,
    Key::KEY_P,
    Key::KEY_LEFTBRACE,
    Key::KEY_RIGHTBRACE,
    Key::KEY_ENTER,
    Key::KEY_LEFTCTRL,
    Key::KEY_A,
    Key::KEY_S,
    Key::KEY_D,
    Key::KEY_F,
    Key::KEY_G,
    Key::KEY_H,
    Key::KEY_J,
    Key::KEY_K,
    Key::KEY_L,
    Key::KEY_SEMICOLON,
    Key::KEY_APOSTROPHE,
    Key::KEY_GRAVE,
    Key::KEY_LEFTSHIFT,
    Key::KEY_BACKSLASH,
    Key::KEY_Z,
    Key::KEY_X,
    Key::KEY_C,
    Key::KEY_V,
    Key::KEY_B,
    Key::KEY_N,
    Key::KEY_M,
    Key::KEY_COMMA,
    Key::KEY_DOT,
    Key::KEY_SLASH,
    Key::KEY_RIGHTSHIFT,
    Key::KEY_KPASTERISK,
    Key::KEY_LEFTALT,
    Key::KEY_SPACE,
    Key::KEY_CAPSLOCK,
    Key::KEY_F1,
    Key::KEY_F2,
    Key::KEY_F3,
    Key::KEY_F4,
    Key::KEY_F5,
    Key::KEY_F6,
    Key::KEY_F7,
    Key::KEY_F8,
    Key::KEY_F9,
    Key::KEY_F10,
    Key::KEY_NUMLOCK,
    Key::KEY_SCROLLLOCK,
    Key::KEY_KP7,
    Key::KEY_KP8,
    Key::KEY_KP9,
    Key::KEY_KPMINUS,
    Key::KEY_KP4,
    Key::KEY_KP5,
    Key::KEY_KP6,
    Key::KEY_KPPLUS,
    Key::KEY_KP1,
    Key::KEY_KP2,
    Key::KEY_KP3,
    Key::KEY_KP0,
    Key::KEY_KPDOT,
    Key::KEY_ZENKAKUHANKAKU,
    Key::KEY_102ND,
    Key::KEY_F11,
    Key::KEY_F12,
    Key::KEY_RO,
    Key::KEY_KATAKANA,
    Key::KEY_HIRAGANA,
    Key::KEY_HENKAN,
    Key::KEY_KATAKANAHIRAGANA,
    Key::KEY_MUHENKAN,
    Key::KEY_KPJPCOMMA,
    Key::KEY_KPENTER,
    Key::KEY_RIGHTCTRL,
    Key::KEY_KPSLASH,
    Key::KEY_SYSRQ,
    Key::KEY_RIGHTALT,
    Key::KEY_LINEFEED,
    Key::KEY_HOME,
    Key::KEY_UP,
    Key::KEY_PAGEUP,
    Key::KEY_LEFT,
    Key::KEY_RIGHT,
    Key::KEY_END,
    Key::KEY_DOWN,
    Key::KEY_PAGEDOWN,
    Key::KEY_INSERT,
    Key::KEY_DELETE,
    Key::KEY_MACRO,
    Key::KEY_MUTE,
    Key::KEY_VOLUMEDOWN,
    Key::KEY_VOLUMEUP,
    Key::KEY_POWER,
    Key::KEY_KPEQUAL,
    Key::KEY_KPPLUSMINUS,
    Key::KEY_PAUSE,
    Key::KEY_SCALE,
    Key::KEY_KPCOMMA,
    Key::KEY_HANGEUL,
    Key::KEY_HANJA,
    Key::KEY_YEN,
    Key::KEY_LEFTMETA,
    Key::KEY_RIGHTMETA,
    Key::KEY_COMPOSE,
    Key::KEY_STOP,
    Key::KEY_AGAIN,
    Key::KEY_PROPS,
    Key::KEY_UNDO,
    Key::KEY_FRONT,
    Key::KEY_COPY,
    Key::KEY_OPEN,
    Key::KEY_PASTE,
    Key::KEY_FIND,
    Key::KEY_CUT,
    Key::KEY_HELP,
    Key::KEY_MENU,
    Key::KEY_CALC,
    Key::KEY_SETUP,
    Key::KEY_SLEEP,
    Key::KEY_WAKEUP,
    Key::KEY_FILE,
    Key::KEY_SENDFILE,
    Key::KEY_DELETEFILE,
    Key::KEY_XFER,
    Key::KEY_PROG1,
    Key::KEY_PROG2,
    Key::KEY_WWW,
    Key::KEY_MSDOS,
    Key::KEY_COFFEE,
    Key::KEY_DIRECTION,
    Key::KEY_ROTATE_DISPLAY,
    Key::KEY_CYCLEWINDOWS,
    Key::KEY_MAIL,
    Key::KEY_BOOKMARKS,
    Key::KEY_COMPUTER,
    Key::KEY_BACK,
    Key::KEY_FORWARD,
    Key::KEY_CLOSECD,
    Key::KEY_EJECTCD,
    Key::KEY_EJECTCLOSECD,
    Key::KEY_NEXTSONG,
    Key::KEY_PLAYPAUSE,
    Key::KEY_PREVIOUSSONG,
    Key::KEY_STOPCD,
    Key::KEY_RECORD,
    Key::KEY_REWIND,
    Key::KEY_PHONE,
    Key::KEY_ISO,
    Key::KEY_CONFIG,
    Key::KEY_HOMEPAGE,
    Key::KEY_REFRESH,
    Key::KEY_EXIT,
    Key::KEY_MOVE,
    Key::KEY_EDIT,
    Key::KEY_SCROLLUP,
    Key::KEY_SCROLLDOWN,
    Key::KEY_KPLEFTPAREN,
    Key::KEY_KPRIGHTPAREN,
    Key::KEY_NEW,
    Key::KEY_REDO,
    Key::KEY_F13,
    Key::KEY_F14,
    Key::KEY_F15,
    Key::KEY_F16,
    Key::KEY_F17,
    Key::KEY_F18,
    Key::KEY_F19,
    Key::KEY_F20,
    Key::KEY_F21,
    Key::KEY_F22,
    Key::KEY_F23,
    Key::KEY_F24,
    Key::KEY_PLAYCD,
    Key::KEY_PAUSECD,
    Key::KEY_PROG3,
    Key::KEY_PROG4,
    Key::KEY_DASHBOARD,
    Key::KEY_SUSPEND,
    Key::KEY_CLOSE,
    Key::KEY_PLAY,
    Key::KEY_FASTFORWARD,
    Key::KEY_BASSBOOST,
    Key::KEY_PRINT,
    Key::KEY_HP,
    Key::KEY_CAMERA,
    Key::KEY_SOUND,
    Key::KEY_QUESTION,
    Key::KEY_EMAIL,
    Key::KEY_CHAT,
    Key::KEY_SEARCH,
    Key::KEY_CONNECT,
    Key::KEY_FINANCE,
    Key::KEY_SPORT,
    Key::KEY_SHOP,
    Key::KEY_ALTERASE,
    Key::KEY_CANCEL,
    Key::KEY_BRIGHTNESSDOWN,
    Key::KEY_BRIGHTNESSUP,
    Key::KEY_MEDIA,
    Key::KEY_SWITCHVIDEOMODE,
    Key::KEY_KBDILLUMTOGGLE,
    Key::KEY_KBDILLUMDOWN,
    Key::KEY_KBDILLUMUP,
    Key::KEY_SEND,
    Key::KEY_REPLY,
    Key::KEY_FORWARDMAIL,
    Key::KEY_SAVE,
    Key::KEY_DOCUMENTS,
    Key::KEY_BATTERY,
    Key::KEY_BLUETOOTH,
    Key::KEY_WLAN,
    Key::KEY_UWB,
    Key::KEY_UNKNOWN,
    Key::KEY_VIDEO_NEXT,
    Key::KEY_VIDEO_PREV,
    Key::KEY_BRIGHTNESS_CYCLE,
    Key::KEY_BRIGHTNESS_AUTO,
    Key::KEY_DISPLAY_OFF,
    Key::KEY_WWAN,
    Key::KEY_RFKILL,
    Key::KEY_MICMUTE,
    Key::KEY_OK,
    Key::KEY_SELECT,
    Key::KEY_GOTO,
    Key::KEY_CLEAR,
    Key::KEY_POWER2,
    Key::KEY_OPTION,
    Key::KEY_INFO,
    Key::KEY_TIME,
    Key::KEY_VENDOR,
    Key::KEY_ARCHIVE,
    Key::KEY_PROGRAM,
    Key::KEY_CHANNEL,
    Key::KEY_FAVORITES,
    Key::KEY_EPG,
    Key::KEY_PVR,
    Key::KEY_MHP,
    Key::KEY_LANGUAGE,
    Key::KEY_TITLE,
    Key::KEY_SUBTITLE,
    Key::KEY_ANGLE,
    Key::KEY_ZOOM,
    Key::KEY_FULL_SCREEN,
    Key::KEY_MODE,
    Key::KEY_KEYBOARD,
    Key::KEY_SCREEN,
    Key::KEY_PC,
    Key::KEY_TV,
    Key::KEY_TV2,
    Key::KEY_VCR,
    Key::KEY_VCR2,
    Key::KEY_SAT,
    Key::KEY_SAT2,
    Key::KEY_CD,
    Key::KEY_TAPE,
    Key::KEY_RADIO,
    Key::KEY_TUNER,
    Key::KEY_PLAYER,
    Key::KEY_TEXT,
    Key::KEY_DVD,
    Key::KEY_AUX,
    Key::KEY_MP3,
    Key::KEY_AUDIO,
    Key::KEY_VIDEO,
    Key::KEY_DIRECTORY,
    Key::KEY_LIST,
    Key::KEY_MEMO,
    Key::KEY_CALENDAR,
    Key::KEY_RED,
    Key::KEY_GREEN,
    Key::KEY_YELLOW,
    Key::KEY_BLUE,
    Key::KEY_CHANNELUP,
    Key::KEY_CHANNELDOWN,
    Key::KEY_FIRST,
    Key::KEY_LAST,
    Key::KEY_AB,
    Key::KEY_NEXT,
    Key::KEY_RESTART,
    Key::KEY_SLOW,
    Key::KEY_SHUFFLE,
    Key::KEY_BREAK,
    Key::KEY_PREVIOUS,
    Key::KEY_DIGITS,
    Key::KEY_TEEN,
    Key::KEY_TWEN,
    Key::KEY_VIDEOPHONE,
    Key::KEY_GAMES,
    Key::KEY_ZOOMIN,
    Key::KEY_ZOOMOUT,
    Key::KEY_ZOOMRESET,
    Key::KEY_WORDPROCESSOR,
    Key::KEY_EDITOR,
    Key::KEY_SPREADSHEET,
    Key::KEY_GRAPHICSEDITOR,
    Key::KEY_PRESENTATION,
    Key::KEY_DATABASE,
    Key::KEY_NEWS,
    Key::KEY_VOICEMAIL,
    Key::KEY_ADDRESSBOOK,
    Key::KEY_MESSENGER,
    Key::KEY_DISPLAYTOGGLE,
    Key::KEY_SPELLCHECK,
    Key::KEY_LOGOFF,
    Key::KEY_DOLLAR,
    Key::KEY_EURO,
    Key::KEY_FRAMEBACK,
    Key::KEY_FRAMEFORWARD,
    Key::KEY_CONTEXT_MENU,
    Key::KEY_MEDIA_REPEAT,
    Key::KEY_10CHANNELSUP,
    Key::KEY_10CHANNELSDOWN,
    Key::KEY_IMAGES,
    Key::KEY_DEL_EOL,
    Key::KEY_DEL_EOS,
    Key::KEY_INS_LINE,
    Key::KEY_DEL_LINE,
    Key::KEY_FN,
    Key::KEY_FN_ESC,
    Key::KEY_FN_F1,
    Key::KEY_FN_F2,
    Key::KEY_FN_F3,
    Key::KEY_FN_F4,
    Key::KEY_FN_F5,
    Key::KEY_FN_F6,
    Key::KEY_FN_F7,
    Key::KEY_FN_F8,
    Key::KEY_FN_F9,
    Key::KEY_FN_F10,
    Key::KEY_FN_F11,
    Key::KEY_FN_F12,
    Key::KEY_FN_1,
    Key::KEY_FN_2,
    Key::KEY_FN_D,
    Key::KEY_FN_E,
    Key::KEY_FN_F,
    Key::KEY_FN_S,
    Key::KEY_FN_B,
    Key::KEY_BRL_DOT1,
    Key::KEY_BRL_DOT2,
    Key::KEY_BRL_DOT3,
    Key::KEY_BRL_DOT4,
    Key::KEY_BRL_DOT5,
    Key::KEY_BRL_DOT6,
    Key::KEY_BRL_DOT7,
    Key::KEY_BRL_DOT8,
    Key::KEY_BRL_DOT9,
    Key::KEY_BRL_DOT10,
    Key::KEY_NUMERIC_0,
    Key::KEY_NUMERIC_1,
    Key::KEY_NUMERIC_2,
    Key::KEY_NUMERIC_3,
    Key::KEY_NUMERIC_4,
    Key::KEY_NUMERIC_5,
    Key::KEY_NUMERIC_6,
    Key::KEY_NUMERIC_7,
    Key::KEY_NUMERIC_8,
    Key::KEY_NUMERIC_9,
    Key::KEY_NUMERIC_STAR,
    Key::KEY_NUMERIC_POUND,
    Key::KEY_NUMERIC_A,
    Key::KEY_NUMERIC_B,
    Key::KEY_NUMERIC_C,
    Key::KEY_NUMERIC_D,
    Key::KEY_CAMERA_FOCUS,
    Key::KEY_WPS_BUTTON,
    Key::KEY_TOUCHPAD_TOGGLE,
    Key::KEY_TOUCHPAD_ON,
    Key::KEY_TOUCHPAD_OFF,
    Key::KEY_CAMERA_ZOOMIN,
    Key::KEY_CAMERA_ZOOMOUT,
    Key::KEY_CAMERA_UP,
    Key::KEY_CAMERA_DOWN,
    Key::KEY_CAMERA_LEFT,
    Key::KEY_CAMERA_RIGHT,
    Key::KEY_ATTENDANT_ON,
    Key::KEY_ATTENDANT_OFF,
    Key::KEY_ATTENDANT_TOGGLE,
    Key::KEY_LIGHTS_TOGGLE,
    Key::KEY_ALS_TOGGLE,
    Key::KEY_BUTTONCONFIG,
    Key::KEY_TASKMANAGER,
    Key::KEY_JOURNAL,
    Key::KEY_CONTROLPANEL,
    Key::KEY_APPSELECT,
    Key::KEY_SCREENSAVER,
    Key::KEY_VOICECOMMAND,
    Key::KEY_ASSISTANT,
    Key::KEY_KBD_LAYOUT_NEXT,
    Key::KEY_BRIGHTNESS_MIN,
    Key::KEY_BRIGHTNESS_MAX,
    Key::KEY_KBDINPUTASSIST_PREV,
    Key::KEY_KBDINPUTASSIST_NEXT,
    Key::KEY_KBDINPUTASSIST_PREVGROUP,
    Key::KEY_KBDINPUTASSIST_NEXTGROUP,
    Key::KEY_KBDINPUTASSIST_ACCEPT,
    Key::KEY_KBDINPUTASSIST_CANCEL,
    Key::KEY_RIGHT_UP,
    Key::KEY_RIGHT_DOWN,
    Key::KEY_LEFT_UP,
    Key::KEY_LEFT_DOWN,
    Key::KEY_ROOT_MENU,
    Key::KEY_MEDIA_TOP_MENU,
    Key::KEY_NUMERIC_11,
    Key::KEY_NUMERIC_12,
    Key::KEY_AUDIO_DESC,
    Key::KEY_3D_MODE,
    Key::KEY_NEXT_FAVORITE,
    Key::KEY_STOP_RECORD,
    Key::KEY_PAUSE_RECORD,
    Key::KEY_VOD,
    Key::KEY_UNMUTE,
    Key::KEY_FASTREVERSE,
    Key::KEY_SLOWREVERSE,
    Key::KEY_DATA,
    Key::KEY_ONSCREEN_KEYBOARD,
    Key::KEY_PRIVACY_SCREEN_TOGGLE,
    Key::KEY_SELECTIVE_SCREENSHOT,
];
