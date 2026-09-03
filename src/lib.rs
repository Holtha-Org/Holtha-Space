pub mod audio {
    pub use holtha_audio_decode as decode;
    pub use holtha_audio_mix as mix;
    pub use holtha_audio_play as play;
}

pub mod core {
    pub use holtha_core_errors as errors;
    pub use holtha_core_types as types;
    pub use holtha_core_utils as utils;
}

pub mod gfx {
    pub use holtha_gfx_color as color;
    pub use holtha_gfx_render as render;
    pub use holtha_gfx_window as window;
}

pub mod input {
    pub use holtha_input_key as key;
    pub use holtha_input_mouse as mouse;
    pub use holtha_input_pad as pad;
}

pub mod math {
    pub use holtha_math_geo as geo;
    pub use holtha_math_mat as mat;
    pub use holtha_math_vec as vec;
}

pub mod net {
    pub use holtha_net_tcp as tcp;
    pub use holtha_net_udp as udp;
}

pub mod sys {
    pub use holtha_sys_fs as fs;
    pub use holtha_sys_info as info;
    pub use holtha_sys_mem as mem;
}