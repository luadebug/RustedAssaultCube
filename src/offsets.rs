pub mod offsets {
    pub const LOCAL_PLAYER_OFFSET: usize = 0x18AC00;
    pub const HEAD_X_FROM_LOCAL_PLAYER: usize = 0x4;
    pub const HEAD_Y_FROM_LOCAL_PLAYER: usize = 0x8;
    pub const HEAD_Z_FROM_LOCAL_PLAYER: usize = 0xC;
    pub const POSITION_X_FROM_LOCAL_PLAYER: usize = 0x28;
    pub const POSITION_Y_FROM_LOCAL_PLAYER: usize = 0x2C;
    pub const POSITION_Z_FROM_LOCAL_PLAYER: usize = 0x30;
    pub const YAW_OFFSET: usize = 0x34;
    pub const PITCH_OFFSET: usize = 0x38;
    pub const HEALTH_OFFSET_FROM_LOCAL_PLAYER: usize = 0xEC;
    pub const ARMOR_OFFSET_FROM_LOCAL_PLAYER: usize = 0xF0;
    pub const GRENADES_COUNT: usize = 0x144;

    pub const AMMO_RIFLE: usize = 0x11C;
    pub const AMMO_IN_MAGAZINE_RIFLE: usize = 0x140;

    pub const AMMO_PISTOL: usize = 0x108;
    pub const AMMO_IN_MAGAZINE_PISTOL: usize = 0x12C;

    pub const AMMO_CARBINE: usize = 0x10C;
    pub const AMMO_IN_MAGAZINE_CARBINE: usize = 0x130;

    pub const AMMO_SHOTGUN: usize = 0x110;
    pub const AMMO_IN_MAGAZINE_SHOTGUN: usize = 0x134;

    pub const AMMO_SUBMACHINEGUN: usize = 0x114;
    pub const AMMO_IN_MAGAZINE_SUBMACHINEGUN: usize = 0x138;

    pub const AMMO_SNIPER: usize = 0x118;
    pub const AMMO_IN_MAGAZINE_SNIPER: usize = 0x13C;

    pub const KNIFE_COOLDOWN: usize = 0x14C;
    pub const PISTOL_COOLDOWN: usize = 0x150;
    pub const CARBINE_COOLDOWN: usize = 0x154;
    pub const SHOTGUN_COOLDOWN: usize = 0x158;
    pub const SUBMACHINEGUN_COOLDOWN: usize = 0x15C;
    pub const SNIPER_COOLDOWN: usize = 0x160;
    pub const RIFLE_COOLDOWN: usize = 0x164;

    pub const NAME_OFFSET_FROM_LOCAL_PLAYER: usize = 0x205;
    pub const NUMBER_OF_PLAYERS_IN_MATCH_OFFSET: usize = 0x18AC0C;
    pub const TEAM_OFFSET_FROM_LOCAL_PLAYER: usize = 0x30C;
    pub const VIEW_MATRIX_ADDR: usize = 0x057DFD0;
    pub const ENTITY_LIST_OFFSET: usize = 0x18AC04;

    pub const BRIGHTNESS: usize = 0x182D40;
    pub const SET_BRIGHTNESS: usize = 0xBA180;
}
