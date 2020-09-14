use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    count: usize,
}

fn model(_app: &App) -> Model {
    Model { count: 0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.count = (model.count + 1) % 120;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if model.count > 60 {
        draw.background().color(PURPLE);
    } else {
        draw.background().color(BLUE);
    }

    draw.ellipse().color(WHITE).xy(app.mouse.position());

    draw.to_frame(app, &frame).unwrap();
}
