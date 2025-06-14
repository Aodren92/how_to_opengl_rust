use crate::sdl;
use crate::sdl::event;

pub struct Camera {
    pub camera_pos:     nalgebra_glm::Vec3,
    pub camera_front:   nalgebra_glm::Vec3,
    pub camera_up:      nalgebra_glm::Vec3,
}

impl Camera {

    pub fn init() -> Self {
        let camera_pos   = nalgebra_glm::vec3(0.0, 0.0,  3.0);
        let camera_front = nalgebra_glm::vec3(0.0, 0.0, -1.0);
        let camera_up    = nalgebra_glm::vec3(0.0, 1.0,  0.0);

        Camera {
            camera_pos,
            camera_front,
            camera_up,
        }

    }

    pub fn update(&mut self, key: Option<sdl::SDLKeycode>, delta_time: f32) {
        let camera_speed = 2.5 * delta_time;
        match key {
            Some(event::SDLK_UP)       => {
                self.camera_pos += (camera_speed) * self.camera_front;
            }
            Some(event::SDLK_DOWN)     => {
                self.camera_pos -= (camera_speed) * self.camera_front
            },
            Some(event::SDLK_LEFT)     => {
                let cross = self.camera_front.cross(&self.camera_up);
                self.camera_pos -= nalgebra_glm::normalize(&cross) * camera_speed;
            },
            Some(event::SDLK_RIGHT)    => {
                let cross = self.camera_front.cross(&self.camera_up);
                self.camera_pos += nalgebra_glm::normalize(&cross) * camera_speed;
            }
                _ => {},
        }

        //if Some(event::SDLK_DOWN) == key {

        //}
    }
}
