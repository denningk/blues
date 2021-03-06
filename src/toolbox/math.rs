use cgmath::{Vector3, Matrix4, vec3, Deg};

use crate::entities::camera::Camera;

pub fn create_transformation_matrix(translation: &Vector3<f32>, rx: f32, ry: f32,
    rz: f32, scale: f32) -> Matrix4<f32> {
        let mut matrix = Matrix4::<f32>::from_translation(*translation);
        matrix = matrix * Matrix4::<f32>::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(rx));
        matrix = matrix * Matrix4::<f32>::from_axis_angle(vec3(0.0, 1.0, 0.0), Deg(ry));
        matrix = matrix * Matrix4::<f32>::from_axis_angle(vec3(1.0, 0.0, 1.0), Deg(rz));
        matrix = matrix * Matrix4::<f32>::from_scale(scale);

        return matrix;
}

pub fn create_view_matrix(camera: &Camera) -> Matrix4<f32> {
        let mut view_matrix = Matrix4::<f32>::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(*camera.get_pitch()));
        view_matrix = view_matrix * Matrix4::<f32>::from_axis_angle(vec3(0.0, 1.0, 0.0), Deg(*camera.get_yaw()));
        
        let camera_pos = camera.get_position();
        let negative_camera_pos = vec3(-camera_pos.x, -camera_pos.y, -camera_pos.z);
        view_matrix = view_matrix * Matrix4::<f32>::from_translation(negative_camera_pos);

        return view_matrix;
}
