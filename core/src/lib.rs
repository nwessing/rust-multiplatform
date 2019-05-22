pub mod pong {
    use std::cmp::{min, max};

    pub enum VerticalOrigin {
        Top,
        Bottom
    }

    pub struct ImageBuffer {
        pub data: Vec<u8>,
        pub width: i32,
        pub height: i32,
        pub vertical_origin: VerticalOrigin,
        pub red_offset: usize,
        pub blue_offset: usize,
        pub green_offset: usize
    }

    impl ImageBuffer {
        pub fn new(width: i32, height: i32, vertical_origin: VerticalOrigin, red_offset: usize, blue_offset: usize, green_offset: usize) -> ImageBuffer {
            ImageBuffer {
                data: vec![0; (width * height * 4) as usize],
                width,
                height,
                vertical_origin,
                red_offset,
                green_offset,
                blue_offset
            }
        }

        pub fn draw_rect(&mut self, left: i32, top: i32, width: i32, height: i32, r: u8, g: u8, b: u8) {
            let right = left + width;
            let bottom = top + height;
            for x_dest in min(left, right)..max(left, right) {
                for y_dest in min(top, bottom)..max(top, bottom) {
                    let y_dest = match self.vertical_origin {
                        VerticalOrigin::Bottom => y_dest,
                        VerticalOrigin::Top => self.height - (y_dest + 1),
                    };
                    if x_dest < 0 || x_dest >= self.width || y_dest < 0 || y_dest >= self.height {
                        continue;
                    }

                    let pixel_base = ((x_dest + (self.width * y_dest)) * 4)as usize;
                    let i_blue = pixel_base + self.blue_offset;
                    let i_green = pixel_base + self.green_offset;
                    let i_red = pixel_base + self.red_offset;

                    self.data[i_red] = r;
                    self.data[i_green] = g;
                    self.data[i_blue] = b;
                }
            }
        }

        pub fn draw_rect_world(&mut self, left: f32, top: f32, width: f32, height: f32, r: u8, g: u8, b: u8) {
            let image_width = self.width as f32;
            let image_height = self.height as f32;
            self.draw_rect(
                (left * image_width) as i32,
                (top * image_height) as i32,
                (width * image_width) as i32,
                (height * image_height) as i32,
                r, g, b
            );
        }
    }

    pub enum PongInput {
        Up,
        Down
    }

    pub struct Point {
        x: f32,
        y: f32
    }

    pub struct Rectangle {
        pub origin: Point,
        pub width: f32,
        pub height: f32
    }

    impl Rectangle {
        pub fn left(&self) -> f32 {
            self.origin.x
        }
        pub fn right(&self) -> f32 {
            self.origin.x + self.width
        }
        pub fn top(&self) -> f32 {
            self.origin.y
        }
        pub fn bottom(&self) -> f32 {
            self.origin.y + self.height
        }
        pub fn center(&self) -> Point {
            Point {
                x: (self.right() + self.left()) / 2.0,
                y: (self.bottom() + self.top()) / 2.0
            }
        }
    }

    pub struct Pong {
        pub player: Rectangle,
        pub player_speed: f32,
        pub opponent: Rectangle,
        pub opponent_speed: f32,
        pub opponent_update_frequency: f32,
        pub opponent_time_since_last_update: f32,
        pub opponent_input: Option<PongInput>,
        pub ball: Rectangle,
        pub ball_velocity: (f32, f32)
    }

    impl Pong {
        pub fn new() -> Pong {
            Pong {
                player: Rectangle {
                    origin: Point{ x: 0.05, y: 0.5 },
                    width: 0.02,
                    height: 0.25
                },
                player_speed: 1.0,
                opponent: Rectangle {
                    origin: Point{ x: 0.93, y: 0.5 },
                    width: 0.02,
                    height: 0.25
                },
                opponent_speed: 0.75,
                opponent_update_frequency: 0.25,
                opponent_time_since_last_update: 0.0,
                opponent_input: None,
                ball: Rectangle {
                    origin: Point{ x: 0.5, y: 0.5 },
                    width: 0.02,
                    height: 0.02
                },
                ball_velocity: (-0.5, -0.25),
            }
        }

        pub fn update(self: &mut Pong, input: Option<PongInput>, elapsed: f32, image: &mut ImageBuffer) {
            let width = image.width;
            let height = image.height;

            update_position(&mut self.player, &input, self.player_speed, elapsed);

            if self.opponent_time_since_last_update >= self.opponent_update_frequency {
                let opponent_y = self.opponent.center().y;
                let ball_y = self.ball.center().y;
                self.opponent_input = if f32::abs(opponent_y - ball_y) < self.opponent.height / 4.0 {
                    None
                } else if opponent_y <= ball_y {
                    Some(PongInput::Up)
                } else {
                    Some(PongInput::Down)
                };
                self.opponent_time_since_last_update = 0.0;
            } else {
                self.opponent_time_since_last_update += elapsed;
            }

            update_position(&mut self.opponent, &self.opponent_input, self.opponent_speed, elapsed);

            self.ball.origin.x += self.ball_velocity.0 * elapsed;
            self.ball.origin.y += self.ball_velocity.1 * elapsed;

            if is_colliding(&self.player, &self.ball) {
                self.ball_velocity.0 = f32::abs(self.ball_velocity.0);
            }

            if is_colliding(&self.opponent, &self.ball) {
                self.ball_velocity.0 = -f32::abs(self.ball_velocity.0);
            }

            if self.ball.origin.x <= 0.0  || self.ball.origin.x + self.ball.width >= 1.0 {
                self.ball.origin.x = 0.5;
                self.ball.origin.y = 0.5;
            }

            if self.ball.origin.y <= 0.0 {
                self.ball_velocity.1 = self.ball_velocity.1.abs();
            }
            if self.ball.origin.y + self.ball.height >= 1.0 {
                self.ball_velocity.1 = -1.0 * self.ball_velocity.1.abs();
            }

            image.draw_rect(0, 0, width, height, 0, 0, 0);
            image.draw_rect_world(self.player.origin.x, self.player.origin.y, self.player.width, self.player.height, 255, 0, 0);
            image.draw_rect_world(self.opponent.origin.x, self.opponent.origin.y, self.opponent.width, self.opponent.height, 255, 0, 0);
            image.draw_rect_world(self.ball.origin.x, self.ball.origin.y, self.ball.width, self.ball.height, 255, 255, 255);
        }
    }

    fn is_colliding(a: &Rectangle, b: &Rectangle) -> bool {
        a.left() < b.right() && a.right() > b.left() && a.top() < b.bottom() && a.bottom() > b.top()
    }

    fn update_position(entity: &mut Rectangle, input: &Option<PongInput>, speed: f32, elapsed: f32) {
        entity.origin.y += speed * elapsed * match input {
            Some(PongInput::Up) => 1.0,
            Some(PongInput::Down) => -1.0,
            None => 0.0
        };

        entity.origin.y = f32::max(entity.origin.y, 0.0);
        entity.origin.y = f32::min(entity.origin.y, 1.0 - entity.height);
    }
}
