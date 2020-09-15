use nannou::prelude::*;

const CELL_SIZE: f32 = 50.0;
const GAP_SIZE: f32 = 10.0;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Row {
    cells: [bool; 16],
}

impl Row {
    pub fn new() -> Self {
        Self {
            cells: [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        }
    }

    pub fn get_rects(&self, top_left: Point2<f32>) -> Vec<Rect> {
        let mut rects = Vec::with_capacity(self.cells.len());
        for i in 0..self.cells.len() {
            let rect = Rect::from_w_h(CELL_SIZE, CELL_SIZE)
                .shift_x(top_left.x + CELL_SIZE)
                .shift_y(top_left.y - CELL_SIZE)
                .shift_x(i as f32 * (CELL_SIZE + GAP_SIZE));
            rects.push(rect)
        }
        rects
    }
}

struct Model {
    rows: Vec<Row>,
}

fn model(_app: &App) -> Model {
    Model {
        rows: vec![
            Row::new(),
            Row::new(),
            Row::new(),
            Row::new(),
            Row::new(),
            Row::new(),
        ],
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();

    draw.background().color(BLACK);

    for (i, row) in model.rows.iter().enumerate() {
        for rect in row.get_rects(window.top_left()) {
            draw.rect()
                .y(rect.y() - i as f32 * (CELL_SIZE + GAP_SIZE))
                .x(rect.x())
                .wh(rect.wh())
                .color(WHITE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
