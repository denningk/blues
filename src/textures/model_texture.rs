pub struct ModelTexture {
    texture_id: u32,
}

impl ModelTexture {
    pub fn new(texture_id: u32) -> ModelTexture {
        ModelTexture { texture_id }
    }

    pub fn get_texture_id(&self) -> &u32 {
        &self.texture_id
    }
}
