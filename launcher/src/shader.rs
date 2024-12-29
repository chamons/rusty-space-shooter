use anyhow::Result;
use macroquad::prelude::*;

pub struct Shader {
    render_target: RenderTarget,
    material: Material,
}

// This is rather hard coded in all but code for the starfield
// shader (single param - direction_modifier)
impl Shader {
    pub fn new(fragment: &str, vertex: &str) -> Result<Self> {
        let render_target = render_target(320, 150);
        render_target.texture.set_filter(FilterMode::Nearest);
        let material = load_material(
            ShaderSource::Glsl { vertex, fragment },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("iResolution", UniformType::Float2),
                    UniformDesc::new("direction_modifier", UniformType::Float1),
                ],
                ..Default::default()
            },
        )?;
        Ok(Shader {
            render_target,
            material,
        })
    }

    pub fn render(&self, direction_modifier: f32) {
        self.material
            .set_uniform("iResolution", (screen_width(), screen_height()));
        self.material
            .set_uniform("direction_modifier", direction_modifier);

        gl_use_material(&self.material);
        draw_texture_ex(
            &self.render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}
