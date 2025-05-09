use ratatui::layout::{Constraint, Direction, Layout};

// TODO: consider moving to some lib
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub fn vertical_layout(heights: Vec<u16>) -> Layout {
    let constraints = to_constraints(heights);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
}

pub fn horizontal_layout(widths: Vec<u16>) -> Layout {
    let constraints = to_constraints(widths);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
}

fn to_constraints(vals: Vec<u16>) -> Vec<Constraint> {
    vals.iter().map(|h| Constraint::Length(*h)).collect()
}

pub fn to_string(vec: Vec<&str>) -> Vec<String> {
    vec.iter().map(|s| s.to_string()).collect()
}
