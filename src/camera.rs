use cgmath::{
    Matrix4, Point3, Vector3, Rad, Quaternion,
    InnerSpace, Rotation, Rotation3,
};

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
}

impl Camera {
    pub fn new(eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) -> Self {
        Self { eye, target, up }
    }

    pub fn rotate_yaw(&mut self, angle_radians: f32) {
        let forward = (self.target - self.eye).normalize();
        let rotation = Quaternion::from_axis_angle(self.up, Rad(angle_radians));
        let rotated = rotation.rotate_vector(forward);
        self.target = self.eye + rotated;
    }

    pub fn rotate_pitch(&mut self, angle_radians: f32) {
        let forward = (self.target - self.eye).normalize();
        let right = forward.cross(self.up).normalize();
        let rotation = Quaternion::from_axis_angle(right, Rad(angle_radians));
        let rotated = rotation.rotate_vector(forward);
        self.target = self.eye + rotated;
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.eye, self.target, self.up)
    }

    pub fn move_forward(&mut self, amount: f32) {
        let forward = (self.target - self.eye).normalize() * amount;
        self.eye += forward;
        self.target += forward;
    }

    pub fn move_right(&mut self, amount: f32) {
        let forward = (self.target - self.eye).normalize();
        let right = forward.cross(self.up).normalize() * amount;
        self.eye += right;
        self.target += right;
    }

    pub fn move_up(&mut self, amount: f32) {
        let up = self.up.normalize() * amount;
        self.eye += up;
        self.target += up;
    }
}
