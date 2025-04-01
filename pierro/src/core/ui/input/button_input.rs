
/// The state of an input device that can either be down or not.
/// Has utilities for detecting the frame the button was first pressed, released, etc.
#[derive(Clone, Copy)]
pub struct ButtonInput {
    state: bool,
    prev_state: bool,
    clicked: bool,
    time_since_press: f32,
    time_since_release: f32,
    click_count: u32
}

impl ButtonInput {

    pub fn new() -> Self {
        Self {
            state: false,
            prev_state: false,
            clicked: false,
            time_since_press: 0.0,
            time_since_release: 0.0,
            click_count: 0
        }
    }

    fn timer_tick(&mut self, delta_time: f32) {
        self.time_since_press += delta_time;
        self.time_since_release += delta_time;
        if self.pressed() {
            self.time_since_press = 0.0;
        }
        if self.released() {
            self.clicked = self.time_since_press < 1.0;
            if self.time_since_release > 0.3 {
                self.click_count = 0;
            }
            if self.clicked {
                self.click_count += 1;
            }
            self.time_since_release = 0.0;
        } else {
            self.clicked = false;
        }
    }

    /// Update the button with a new state
    pub fn tick(&mut self, state: bool, delta_time: f32) {
        self.prev_state = self.state;
        self.state = state;
        self.timer_tick(delta_time);
    } 

    /// Set the button to be down 
    pub fn press(&mut self) {
        self.state = true;
    }

    /// Set the button to be up
    pub fn release(&mut self) {
        self.state = false;
    }

    pub fn press_with_edge(&mut self) {
        self.prev_state = false;
        self.state = true;
    }

    pub fn release_with_edge(&mut self) {
        self.prev_state = true;
        self.state = false;
    }

    /// Update the button without providing a new state
    pub fn tick_with_same_state(&mut self, delta_time: f32) {
        self.prev_state = self.state;
        self.timer_tick(delta_time);
    }

    /// Is the button down?
    pub fn down(&self) -> bool {
        self.state
    }

    /// Has the button just been pressed?
    pub fn pressed(&self) -> bool {
        self.state && !self.prev_state
    } 

    /// Has the button just been released?
    pub fn released(&self) -> bool {
        !self.state && self.prev_state
    }

    pub fn click_count(&self) -> u32 {
        if self.clicked {
            self.click_count
        } else {
            0
        }
    }

    /// Was the button clicked?
    pub fn clicked(&self) -> bool {
        self.clicked
    }

    /// Was the button clicked once?
    pub fn single_clicked(&self) -> bool {
        self.click_count() == 1
    }

    /// Was the button double clicked?
    pub fn double_clicked(&self) -> bool {
        self.click_count() == 2
    }

    /// Was the button triple clicked?
    pub fn triple_clicked(&self) -> bool {
        self.click_count() == 3
    }

}
