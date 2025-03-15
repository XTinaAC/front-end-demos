/* An [enum] whose [variants] each stores different amounts & types of values:
 *  Quit: has no data associated with it
 *  Move: has an anonymous [struct] with named fields
 *  Write: has a single [String]
 *  ChangeColor: has 3 [i32] values
 *
 */
// We have to explicitly include the functionality to print out debugging information.
#[derive(Debug)]
enum Action {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Like [structs], we can define [methods] on [enums] using [impl]:
impl Action {
    fn take(&self) {
        // Putting the specifier [:?] inside the curly braces to
        // use an output format called Debug.
        println!("action:::{:?}:::score:::{}", *self, score_of_action(self));
        println!("action:::{:#?}:::score:::{}", *self, score_of_action(self));
    }
}

fn score_of_action(action: &Action) -> i32 {
    // A [match] expression that has [variants] of an [enum] as its patterns
    match action {
        Action::Quit => -1,
        Action::Move { x: _, y: _ } => 0,
        Action::Write(_str) => 1,
        Action::ChangeColor(_r, _g, _b) => 2,
    }
}

fn main() {
    let _quit = Action::Quit;
    let _move = Action::Move { x: 1, y: 2 };
    let _write = Action::Write(String::from("XTina"));
    let _change_color = Action::ChangeColor(1, 2, 3);
    _quit.take();
    _move.take();
    _write.take();
    _change_color.take();
}
