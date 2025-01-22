#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::object::{Graphics, OamManaged, Object, Sprite, Tag, TagMap},
    fixnum::{FixedNum, Vector2D},
    include_aseprite,
    input::{Button, ButtonController},
};

type FixedNumberType = FixedNum<10>;

struct WorldVector(Vector2D<FixedNumberType>);
struct ScreenVector(Vector2D<i32>);

static MEOWDY_GRAPHICS: &Graphics = include_aseprite!("gfx/meowdy.aseprite");
static MEOWDY_TAG_MAP: &TagMap = MEOWDY_GRAPHICS.tags();

static MEOWDY_WALKING: &Tag = MEOWDY_TAG_MAP.get("Walking");
static MEOWDY_SHOOTING_LEFT: &Tag = MEOWDY_TAG_MAP.get("Shooting-Left");
static MEOWDY_SHOOTING_RIGHT: &Tag = MEOWDY_TAG_MAP.get("Shooting-Right");

static BULLET_GRAPHICS: &Graphics = include_aseprite!("gfx/bullet.aseprite");
static BULLET_TAG_MAP: &TagMap = BULLET_GRAPHICS.tags();
static BULLET_REGULAR: &Tag = BULLET_TAG_MAP.get("Regular");

/// Given a world position and the camera position in the world, returns the position on the screen.
///
/// Useful for rendering.
fn world_to_screen(
    position: Vector2D<FixedNumberType>,
    camera: Vector2D<FixedNumberType>,
) -> Vector2D<i32> {
    (position - camera).trunc()
        + Vector2D {
            x: agb::display::WIDTH / 2,
            y: agb::display::HEIGHT / 2,
        }
}

/// Holds a sprite and
struct Entity<'a> {
    sprite: Object<'a>,
    position: Vector2D<FixedNumberType>,
}

impl<'a> Entity<'a> {
    pub fn new(object: &'a OamManaged) -> Self {
        let mut bullet = Entity {
            sprite: object.object_sprite(BULLET_REGULAR.sprite(0)),
            position: Vector2D {
                x: 0.into(),
                y: 0.into(),
            },
        };
        bullet.sprite.show();
        bullet
    }

    pub fn render(&mut self, camera: Vector2D<FixedNumberType>) {
        self.sprite
            .set_position(world_to_screen(self.position, camera));
    }
}

pub fn main(mut gba: agb::Gba) -> ! {
    let object = gba.display.object.get_managed();
    let mut input = ButtonController::new();

    let mut meowdy = object.object_sprite(MEOWDY_WALKING.sprite(0));
    meowdy.show();

    let mut frame_count = 0;

    let mut camera: Vector2D<FixedNumberType> = Vector2D {
        x: 0.into(),
        y: 0.into(),
    };

    let mut bullet = Entity::new(&object);

    loop {
        meowdy.set_position(
            meowdy.position()
                + Vector2D {
                    x: input.x_tri() as i32,
                    y: input.y_tri() as i32,
                },
        );

        frame_count += 1;
        frame_count %= 64;
        meowdy.set_sprite(object.sprite(MEOWDY_WALKING.sprite(frame_count / 8)));
        agb::display::busy_wait_for_vblank();
        object.commit();
        input.update();
    }
}
