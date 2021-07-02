pub mod player {
    use crate::plugins::{animation::components::*, player::components::*};

    pub const BASE_SPEED: f32 = 250.0;

    pub const PLAYER_ANIMATIONS: &[EntList<AnimationType, PlayerType>] = &[EntList {
        animation_states: &[
            // MASK DUDE
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerType::MaskDude,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerType::MaskDude,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerType::MaskDude,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerType::MaskDude,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::MaskDude,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::MaskDude,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerType::MaskDude,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerType::MaskDude,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Mask Dude/Wall Jump (32x32).png",
            },
            // NINJA FROG
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Wall Jump (32x32).png",
            },
            // NINJA FROG
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerType::NinjaFrog,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Ninja Frog/Wall Jump (32x32).png",
            },
            // PINK MAN
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerType::PinkMan,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerType::PinkMan,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerType::PinkMan,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerType::PinkMan,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerType::PinkMan,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Wall Jump (32x32).png",
            },
            // Pink Man
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerType::PinkMan,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerType::PinkMan,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerType::PinkMan,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::PinkMan,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerType::PinkMan,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerType::PinkMan,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Pink Man/Wall Jump (32x32).png",
            },
            // VIRTUAL GUY
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Wall Jump (32x32).png",
            },
            // Virtual Guy
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Idle,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 11,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Idle (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::DoubleJump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 6,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Double Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Fall,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Fall (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Hit,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 7,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Hit (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Jump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 1,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Jump (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::Run,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 12,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Run (32x32).png",
            },
            AnimationState {
                kv: EntTypeKey {
                    anim_ty: AnimationType::WallJump,
                    ty: PlayerType::VirtualGuy,
                },
                frames: 5,
                tile_size: (32.0, 32.0),
                path: "Virtual Guy/Wall Jump (32x32).png",
            },
        ],
        root_path: "pixels/Players/",
    }];
}
