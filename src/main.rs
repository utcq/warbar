use wlroots_x11::XWaylandManager;

fn main() {
    unsafe {
        let mut event_loop = EventLoop::new();
        let mut xwm = XWaylandManager::new(&mut event_loop);
        event_loop.run(&mut xwm);
    }
}