use nannou::prelude::*;

const CELL_SIZE: f32 = 50.0;
const GAP_SIZE: f32 = 10.0;
const INCREMENT_EVERY: usize = 15;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Row {
    y: f32,
    x: f32,
    cells: [bool; 16],
}

impl Row {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            y: y - CELL_SIZE,
            x: x + CELL_SIZE,
            cells: [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        }
    }

    pub fn get_rects(&self) -> Vec<(Rect, bool)> {
        let mut rects = Vec::with_capacity(self.cells.len());
        for (i, cell) in self.cells.iter().enumerate() {
            rects.push((self.cell_rect(i), *cell))
        }
        rects
    }

    pub fn click(&mut self, position: Point2<f32>) {
        // This could be done without iterating. Oh well.
        for i in 0..self.cells.len() {
            if self.cell_rect(i).contains(position) {
                self.cells[i] = !self.cells[i]
            }
        }
    }

    fn cell_rect(&self, i: usize) -> Rect {
        Rect::from_w_h(CELL_SIZE, CELL_SIZE)
            .shift_y(self.y)
            .shift_x(self.x)
            .shift_x(i as f32 * (CELL_SIZE + GAP_SIZE))
    }
}

struct Model {
    step: usize,
    frame: usize,
    rows: Vec<Row>,
}

impl Model {
    fn click(&mut self, position: Point2<f32>) {
        // This could be done without iterating. Oh well.
        for row in self.rows.iter_mut() {
            row.click(position)
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();
    let window = app.window_rect();
    Model {
        step: 0,
        frame: 0,
        rows: vec![
            Row::new(window.left(), window.top() - 0. * (CELL_SIZE + GAP_SIZE)),
            Row::new(window.left(), window.top() - 1. * (CELL_SIZE + GAP_SIZE)),
            Row::new(window.left(), window.top() - 2. * (CELL_SIZE + GAP_SIZE)),
            Row::new(window.left(), window.top() - 3. * (CELL_SIZE + GAP_SIZE)),
            Row::new(window.left(), window.top() - 4. * (CELL_SIZE + GAP_SIZE)),
            Row::new(window.left(), window.top() - 5. * (CELL_SIZE + GAP_SIZE)),
        ],
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.frame > INCREMENT_EVERY {
        model.frame = 0;
        model.step = (model.step + 1) % 16;
    } else {
        model.frame = model.frame + 1;
    }
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    model.click(app.mouse.position())
}

fn view(app: &App, model: &Model, frame: Frame) {
    let pink = rgb(255, 105, 180);
    let grey = rgb(230, 230, 230);
    let draw = app.draw();

    draw.background().color(BLACK);

    for row in model.rows.iter() {
        for (i, (rect, active)) in row.get_rects().into_iter().enumerate() {
            let color = if active {
                pink.clone()
            } else if model.step == i {
                grey.clone()
            } else if rect.contains(app.mouse.position()) {
                grey.clone()
            } else {
                WHITE
            };
            draw.rect().xy(rect.xy()).wh(rect.wh()).color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
