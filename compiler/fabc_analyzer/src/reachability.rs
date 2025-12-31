pub struct Reachability {
    is_reachable: bool,
    terminates_normally: bool,
}

impl Default for Reachability {
    fn default() -> Self {
        Self {
            is_reachable: true,
            terminates_normally: true,
        }
    }
}

impl Reachability {
    pub fn is_reachable(&self) -> bool {
        self.is_reachable
    }
    pub fn terminates_normally(&self) -> bool {
        self.terminates_normally
    }
    pub fn set_is_reachable(&mut self, value: bool) -> &mut Self {
        self.is_reachable = value;
        self
    }
    pub fn set_terminates_normally(&mut self, value: bool) -> &mut Self {
        self.terminates_normally = value;
        self
    }
}
