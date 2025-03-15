/* An [enum] whose [variants] each stores different amounts & types of values:
 *  Quit: has no data associated with it
 *  Move: has an anonymous [struct] with named fields
 *  Write: has a single [String]
 *  ChangeColor: has 3 [i32] values
 *
 */
// We have to explicitly include the functionality to print out debugging information.
#[derive(Debug)]
#[allow(dead_code)]
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
        println!("action:::{:?}:::score:::{}", self, score_of_action(self));
        println!("action:::{:#?}:::score:::{}", self, score_of_action(self));
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

// Matching with Option<T> that is defined in the standard library:
//  enum Option<T> {
//      None,
//      Some(T),
//  }
//   Rust doesn't have [nulls], but it has this enum to encode the
//   concept of a value being present/absent.
fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(val) => Some(val + 1),
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

    let none_plus_1 = plus_one(None);
    let one_plus_1 = plus_one(Some(1));
    println!("none+1->{:?}", one_plus_1);
    println!("one+1->{:?}", one_plus_1);

    // Matchings are exhaustive; we can use the [_] placeholder as a catch-all pattern:
    //  it matches any value & does not bind to that value (so Rust won't warn us about
    //  an unused variable)
    match none_plus_1 {
        None => println!("__None__"),
        _ => println!("__Some__"),
    }
    // A more concise control flow with [if-let-else] (of which we can think as syntax
    // sugar for a [match] that fits 1 value and ignores all others.)
    if let None = none_plus_1 {
        println!("__None__");
    } else {
        println!("__Some__");
    }
}
