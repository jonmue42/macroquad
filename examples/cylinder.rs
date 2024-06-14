use macroquad::prelude::*;

#[macroquad::main("Cylinder")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(30, 1., BLACK, GRAY);

        draw_cylinder(vec3(0., 1., -10.), 2., 2., 5., None, DARKBLUE);
        draw_cylinder_wires(vec3(0., 1., -5.), 2., 2., 5., None, DARKBLUE);
        draw_cylinder(vec3(0., 1., 0.), 1., 2., 5., None, DARKBLUE);
        draw_cylinder(vec3(0., 1., 5.), 0., 2., 5., None, DARKBLUE);
        draw_cylinder(vec3(0., 1., 10.), 2., 0., 5., None, DARKBLUE);

        next_frame().await
    }
}
