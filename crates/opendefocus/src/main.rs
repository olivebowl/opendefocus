use ndarray::Array3;
use opendefocus::{
    OpenDefocusRenderer,
    datamodel::render::RenderSpecs,
    datamodel::{IVector4, Settings, UVector2},
};

fn main() {
    let mut image: Array3<f32> = Array3::zeros((256, 256, 4));
    let mut settings = Settings::default();
    settings.render.resolution = UVector2 { x: 256, y: 256 }; // resolution of full image
    let full_region = IVector4 {
        x: 50,
        y: 50,
        z: 206,
        w: 206,
    }; // full stripe size
    let render_region = IVector4 {
        x: 56,
        y: 56,
        z: 200,
        w: 200,
    }; // region we want to render (padding of 6)
    let render_specs = RenderSpecs {
        full_region,
        render_region,
    };

    let renderer = OpenDefocusRenderer::new(true, &mut settings).await.unwrap();
    renderer
        .render_stripe(
            render_specs,
            settings,
            image.slice_mut(s!(50..250, 50..250, ..)).view_mut(),
            None,
            None,
        )
        .await
        .unwrap();
}
