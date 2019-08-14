use nannou::app::Builder;
use nannou::Draw;
use nannou::prelude::*;

type Length = f32;

#[derive(Debug)]
enum LSystemSymbol {
    Variable((Variables, Vec<Action>)),
    Constant(Vec<Action>),
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Rotate(f32),
    DrawLine(Length, FillColor),
    DrawCircle(FillColor),
    Push,
    Pop,
}

#[derive(Copy, Clone, Debug)]
enum FillColor {
    Background,
    Inherit,
    Rgb(f32, f32, f32),
}

#[derive(Copy, Clone, Debug)]
enum Variables {
    A, B, C, D, E, F, G, H,
}

#[derive(Copy, Clone)]
struct Config {
    position: Point2<f32>,
    radians: f32,
    color: nannou::color::Rgb,
}

struct LSystem {
    state: Vec<LSystemSymbol>,
    dimensions: Vector2<f32>,
    // stack: Vec<Config>,
}

impl LSystem {
    fn axiom_config() -> Config {
        Config {
            position: Point2::new(0.0, 0.0),
            radians: nannou::prelude::PI * 0.5,
            color: rgb(0.0, 0.0, 0.0),
        }
    }

    fn axiom() -> LSystem {
        LSystem {
            state: vec![LSystemSymbol::Variable((Variables::A, vec![Action::DrawLine(10.0, FillColor::Inherit)]))],
            dimensions: Vector2::new(0.0, 10.0),
        }
    }

    fn draw(&self, draw: &Draw) {
        use nannou::geom::Vector2;
        draw.background().rgb(1.0, 1.0, 1.0);
        let mut curr_state = LSystem::axiom_config();
        curr_state.position.y -= self.dimensions.y * 0.5;
        let mut stack = Vec::new();

        for symbol in &self.state {
            match symbol {
                LSystemSymbol::Constant(actions) | LSystemSymbol::Variable((_, actions)) => {
                    process_actions(&draw, &mut curr_state, &mut stack, &actions)
                },
            }
        }
    }
}

fn process_actions(draw: &Draw, curr_state: &mut Config, stack: &mut Vec<Config>, actions: &[Action]) {
    for action in actions {
        match action {
            Action::DrawLine(length, color) => {
                let color = get_color(&curr_state, *color);
                let dir = Vector2::new(curr_state.radians.cos(), curr_state.radians.sin()) * *length;
                draw.line().start(curr_state.position).end(curr_state.position + dir).color(color).finish();
                curr_state.position += dir;
                curr_state.color = color;
            },
            Action::DrawCircle(color) => {
                let color = get_color(&curr_state, *color);
                curr_state.color = color;
                draw.ellipse().xy(curr_state.position).wh(Vector2::new(10.0, 10.0)).color(color);
            },
            Action::Rotate(rad) => {
                curr_state.radians += *rad;
            },
            Action::Push => {
                stack.push(*curr_state);
            },
            Action::Pop => {
                *curr_state = stack.pop().unwrap();
            }
        }
    }
}

fn get_color(curr_state: &Config, color: FillColor) -> nannou::color::Rgb<f32> {
    match color {
        FillColor::Inherit => curr_state.color,
        FillColor::Background => curr_state.color,
        FillColor::Rgb(r, g, b) => rgb(r, g, b)
    }
}

fn get_drawing_dimensions(lsys: &LSystem) -> Vector2<f32> {
    let mut curr_state = LSystem::axiom_config();
    let mut stack = Vec::new();
    let mut min = Vector2::new(0.0, 0.0);
    let mut max = Vector2::new(0.0, 0.0);

    for symbol in &lsys.state {
        match symbol {
            LSystemSymbol::Constant(actions) => {
                for action in actions {
                    match action {
                        Action::DrawLine(length, _) => {
                            let dir = Vector2::new(curr_state.radians.cos(), curr_state.radians.sin()) * *length;
                            curr_state.position += dir;
                            if curr_state.position.x < min.x {
                                min.x = curr_state.position.x;
                            }
                            if curr_state.position.x > max.x {
                                max.x = curr_state.position.x;
                            }
                            if curr_state.position.y < min.y {
                                min.y = curr_state.position.y;
                            }
                            if curr_state.position.y > max.y {
                                max.y = curr_state.position.y;
                            }
                        },
                        Action::Rotate(rad) => {
                            curr_state.radians += *rad;
                        },
                        Action::Push => {
                            stack.push(curr_state);
                        },
                        Action::Pop => {
                            curr_state = stack.pop().unwrap();
                        },
                        _ => (),
                    }
                }
            },
            LSystemSymbol::Variable((_, actions)) => {
                for action in actions {
                    match action {
                        Action::DrawLine(length, _) => {
                            let dir = Vector2::new(curr_state.radians.cos(), curr_state.radians.sin()) * *length;
                            curr_state.position += dir;
                            if curr_state.position.x < min.x {
                                min.x = curr_state.position.x;
                            }
                            if curr_state.position.x > max.x {
                                max.x = curr_state.position.x;
                            }
                            if curr_state.position.y < min.y {
                                min.y = curr_state.position.y;
                            }
                            if curr_state.position.y > max.y {
                                max.y = curr_state.position.y;
                            }
                        },
                        Action::Rotate(rad) => {
                            curr_state.radians += *rad;
                        },
                        Action::Push => {
                            stack.push(curr_state);
                        },
                        Action::Pop => {
                            curr_state = stack.pop().unwrap();
                        },
                        _ => (),
                    }
                }
            }
        }
    }
    Vector2::new(max.x - min.x, max.y - min.y)
}

fn proceed_system<F>(lsys: &mut LSystem, rules: F)
where F: Fn(Variables) -> Vec<LSystemSymbol>
 {
    // println!("old:\n{:?}", lsys.state);
    let mut new_state = Vec::with_capacity(lsys.state.len());
    for symbol in lsys.state.iter() {
        match symbol {
            LSystemSymbol::Constant(c) => new_state.push(LSystemSymbol::Constant(c.to_vec())),
            LSystemSymbol::Variable((v, actions)) => {
                new_state.append(&mut rules1(*v))
            }
        }
    }
    lsys.state = new_state;
    lsys.dimensions = get_drawing_dimensions(&lsys);
    // println!("new:\n{:?}", lsys.state);
}

fn rules1(var: Variables) -> Vec<LSystemSymbol> {
    use Variables::{A, B};
    use FillColor::*;
    match var {
        A => vec![
            LSystemSymbol::Variable((B, vec![Action::DrawLine(10.0, Rgb(0.6, 0.2, 0.8))])),
            LSystemSymbol::Constant(vec![Action::Push, Action::Rotate(f32::PI() * 0.25)]),
            LSystemSymbol::Variable((A, vec![Action::DrawLine(10.0, Rgb(1.0, 0.2, 0.5)), Action::DrawCircle(Inherit)])),
            LSystemSymbol::Constant(vec![Action::Pop, Action::Rotate(f32::PI() * -0.25)]),
            LSystemSymbol::Variable((A, vec![Action::DrawLine(10.0, Rgb(0.1, 0.7, 0.7)), Action::DrawCircle(Rgb(0.0, 0.8, 0.2))])),
        ],
        B => vec![
            LSystemSymbol::Variable((B, vec![Action::DrawLine(10.0, Rgb(0.3, 0.5, 0.8))])),
            LSystemSymbol::Variable((B, vec![Action::DrawLine(10.0, Inherit)])),
        ],
        _ => vec![],
    }
}

fn model(_app: &App) -> LSystem {
    let mut sys = LSystem::axiom();
    proceed_system(&mut sys, rules1);
    proceed_system(&mut sys, rules1);
    proceed_system(&mut sys, rules1);
    proceed_system(&mut sys, rules1);
    proceed_system(&mut sys, rules1);
    proceed_system(&mut sys, rules1);
    sys
}

fn update(_app: &App, _model: &mut LSystem, _update: Update) {

}

fn view(app: &App, model: &LSystem, frame: &Frame) {
    let draw = app.draw();
    // draw.background().rgb(0.4, 0.8, 0.7);
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
