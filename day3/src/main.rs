use std::collections::HashMap;

fn main() {
    let input: usize = 289326;
    let (pos_x, pos_y) = pos_of_cell(input);
    println!("Distance for cell {} is: {}", input, distance(pos_x, pos_y));

    let first_value_larger = first_value_larger_than(input as u64);
    println!(
        "First value larger than the input is: {}",
        first_value_larger
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos_x: i64,
    pos_y: i64,

    right: i64,
    top: i64,
    left: i64,
    bottom: i64,

    next: fn(State) -> State,
}

fn distance(x: i64, y: i64) -> u64 {
    (x.abs() as u64) + (y.abs() as u64)
}

fn first_value_larger_than(input: u64) -> u64 {
    return if input == 0 {
        1
    } else {
        let mut state = State {
            pos_x: 1,
            pos_y: 0,
            right: 1,
            top: 0,
            left: 0,
            bottom: 0,
            next: move_up,
        };
        let mut values: HashMap<(i64, i64), u64> = HashMap::with_capacity(1024);
        values.insert((0, 0), 1);
        values.insert((1, 0), 1);
        let mut value: u64 = 1;
        while value < input {
            state = (state.next)(state);
            value = value_of(state.pos_x, state.pos_y, &values);
            values.insert((state.pos_x, state.pos_y), value);
        }
        value
    };
}

fn value_of(pos_x: i64, pos_y: i64, values: &HashMap<(i64, i64), u64>) -> u64 {
    return *values.get(&(pos_x - 1, pos_y - 1)).unwrap_or(&0) +
        *values.get(&(pos_x - 1, pos_y)).unwrap_or(&0) +
        *values.get(&(pos_x - 1, pos_y + 1)).unwrap_or(&0) +
        *values.get(&(pos_x, pos_y - 1)).unwrap_or(&0) +
        *values.get(&(pos_x, pos_y + 1)).unwrap_or(&0) +
        *values.get(&(pos_x + 1, pos_y - 1)).unwrap_or(&0) +
        *values.get(&(pos_x + 1, pos_y)).unwrap_or(&0) +
        *values.get(&(pos_x + 1, pos_y + 1)).unwrap_or(&0);
}

fn pos_of_cell(n: usize) -> (i64, i64) {
    return if n < 2 {
        (0, 0)
    } else {
        let mut state = State {
            pos_x: 1,
            pos_y: 0,
            right: 1,
            top: 0,
            left: 0,
            bottom: 0,
            next: move_up,
        };
        for _ in 2..n {
            state = (state.next)(state);
        }
        (state.pos_x, state.pos_y)
    };
}

fn move_up(mut state: State) -> State {
    state.pos_y = state.pos_y - 1;
    if state.top > state.pos_y {
        state.top = state.pos_y;
        state.next = move_left;
    }
    return state;
}

fn move_left(mut state: State) -> State {
    state.pos_x = state.pos_x - 1;
    if state.left > state.pos_x {
        state.left = state.pos_y;
        state.next = move_down;
    }
    return state;
}

fn move_down(mut state: State) -> State {
    state.pos_y = state.pos_y + 1;
    if state.bottom < state.pos_y {
        state.bottom = state.pos_y;
        state.next = move_right;
    }
    return state;
}

fn move_right(mut state: State) -> State {
    state.pos_x = state.pos_x + 1;
    if state.right < state.pos_x {
        state.right = state.pos_x;
        state.next = move_up;
    }
    return state;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_should_return_cbm_distance_to_origin() {
        // given
        let test_data: &[((i64, i64), u64)] = &[
            ((0, 0), 0),
            ((1, 0), 1),
            ((-1, 1), 2),
            ((0, -9), 9),
            ((3, -4), 7),
            ((1, 1), 2),
        ];

        // when/then
        for &(input, dis) in test_data {
            assert_eq!(distance(input.0, input.1), dis);
        }
    }

    #[test]
    fn pos_of_cell_should_caclulate_cell_position() {
        // given
        let test_data: &[(usize, (i64, i64))] = &[
            (0, (0, 0)),
            (1, (0, 0)),
            (2, (1, 0)),
            (11, (2, 0)),
            (16, (-1, -2)),
            (24, (1, 2)),
        ];

        // when/then
        for &(input, pos) in test_data {
            assert_eq!(pos_of_cell(input), pos);
        }
    }

    #[test]
    fn move_up_should_move_up() {
        // given
        let input = State {
            pos_x: 2,
            pos_y: 1,
            right: 2,
            top: -1,
            left: -1,
            bottom: 1,
            next: move_up,
        };

        // when
        let output = (input.next)(input);

        // then
        assert_eq!(
            output,
            State {
                pos_x: 2,
                pos_y: 0,
                right: 2,
                top: -1,
                left: -1,
                bottom: 1,
                next: move_up,
            }
        );
    }

    #[test]
    fn move_up_should_turn_left_at_the_end() {
        // given
        let input = State {
            pos_x: 2,
            pos_y: -1,
            right: 2,
            top: -1,
            left: -1,
            bottom: 1,
            next: move_up,
        };

        // when
        let output = (input.next)(input);

        // then
        assert_eq!(
            output,
            State {
                pos_x: 2,
                pos_y: -2,
                right: 2,
                top: -2,
                left: -1,
                bottom: 1,
                next: move_left,
            }
        );
    }

    #[test]
    fn move_left_should_move_left() {
        // given
        let input = State {
            pos_x: 1,
            pos_y: -2,
            right: 2,
            top: -2,
            left: -1,
            bottom: 1,
            next: move_left,
        };

        // when
        let output = (input.next)(input);

        // then
        assert_eq!(
            output,
            State {
                pos_x: 0,
                pos_y: -2,
                right: 2,
                top: -2,
                left: -1,
                bottom: 1,
                next: move_left,
            }
        );
    }

    #[test]
    fn move_left_should_turn_down_at_the_end() {
        // given
        let input = State {
            pos_x: -1,
            pos_y: -2,
            right: 2,
            top: -2,
            left: -1,
            bottom: 1,
            next: move_left,
        };

        // when
        let output = (input.next)(input);

        // then
        assert_eq!(
            output,
            State {
                pos_x: -2,
                pos_y: -2,
                right: 2,
                top: -2,
                left: -2,
                bottom: 1,
                next: move_down,
            }
        );
    }

    #[test]
    fn move_down_should_move_down() {
        // given
        let input = State {
            pos_x: -2,
            pos_y: -1,
            right: 2,
            top: -2,
            left: -2,
            bottom: 1,
            next: move_down,
        };

        // when
        let output = (input.next)(input);

        // then
        assert_eq!(
            output,
            State {
                pos_x: -2,
                pos_y: 0,
                right: 2,
                top: -2,
                left: -2,
                bottom: 1,
                next: move_down,
            }
        );
    }

    #[test]
    fn move_down_should_turn_right_at_the_end() {
        // given
        let input = State {
            pos_x: -2,
            pos_y: 1,
            right: 2,
            top: -2,
            left: -2,
            bottom: 1,
            next: move_down,
        };

        // when
        let output = (input.next)(input);

        // then
        assert_eq!(
            output,
            State {
                pos_x: -2,
                pos_y: 2,
                right: 2,
                top: -2,
                left: -2,
                bottom: 2,
                next: move_right,
            }
        );
    }

    #[test]
    fn move_right_should_move_right() {
        // given
        let input = State {
            pos_x: -1,
            pos_y: 2,
            right: 2,
            top: -2,
            left: -2,
            bottom: 2,
            next: move_right,
        };

        // when
        let output = (input.next)(input);

        // then
        assert_eq!(
            output,
            State {
                pos_x: 0,
                pos_y: 2,
                right: 2,
                top: -2,
                left: -2,
                bottom: 2,
                next: move_right,
            }
        );
    }

    #[test]
    fn move_right_should_turn_up_at_the_end() {
        // given
        let input = State {
            pos_x: 2,
            pos_y: 2,
            right: 2,
            top: -2,
            left: -2,
            bottom: 2,
            next: move_right,
        };

        // when
        let output = (input.next)(input);

        // then
        assert_eq!(
            output,
            State {
                pos_x: 3,
                pos_y: 2,
                right: 3,
                top: -2,
                left: -2,
                bottom: 2,
                next: move_up,
            }
        );
    }
}
