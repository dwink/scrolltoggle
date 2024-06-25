
#[link(name = "PreferencePanesSupport", kind="framework")]
extern "C" {
    fn swipeScrollDirection() -> u8;
    fn setSwipeScrollDirection(natural: u8);
}

#[derive(Debug)]
enum ScrollDirection {
    Normal = 0,
    Natural = 1
}

fn swipe_scroll_direction() -> ScrollDirection {
    let direction = unsafe { swipeScrollDirection() };

    match direction {
        0 => ScrollDirection::Normal,
        1 => ScrollDirection::Natural,
        _ => unreachable!()
    }
}

fn set_swipe_scroll_direction(direction: ScrollDirection) {

    unsafe { setSwipeScrollDirection(direction as u8) }
}

fn main() {

    let new = match swipe_scroll_direction() {
        ScrollDirection::Normal => ScrollDirection::Natural,
        ScrollDirection::Natural => ScrollDirection::Normal,
    };

    set_swipe_scroll_direction(new)
    
}
