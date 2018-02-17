//! Code for the `qagame` Quake 3 module
//!
//! The `qagame` module contains the server-side logic of Quake 3, e.g.:
//!
//! * player physics
//! * game types
//! * weapons
//! * bot A.I.
//!
//! Unlike the other modules, it does not have to be present on the game client,
//! i.e. players do not have to download it.

/// System traps provided by the engine
///
/// See `gameImport_t` in [ioquake3's `game/g_public.h`](https://github.com/ioquake/ioq3/blob/master/code/game/g_public.h).
#[repr(C)]
// TODO: Should these be shortened and renamed, e.g. `Print` and `Error` instead of `G_PRINT` and `G_ERROR`?
#[allow(non_camel_case_types)]
#[derive(Primitive)]
pub enum Imports {
    G_PRINT = 0,
    G_ERROR = 1,
}

/// Functions exported by the module
///
/// See `gameExport_t` in [ioquake3's `game/g_public.h`](https://github.com/ioquake/ioq3/blob/master/code/game/g_public.h).
#[repr(C)]
// TODO: Should these be shortened and renamed, e.g. `Init` and `Shutdown` instead of `GAME_INIT` and `GAME_SHUTDOWN`?
#[allow(non_camel_case_types)]
#[derive(Primitive)]
pub enum Exports {
    GAME_INIT = 0,
    GAME_SHUTDOWN = 1,
}
