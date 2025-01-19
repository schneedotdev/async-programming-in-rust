pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1;
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
extern "C" {
    /// syscall that creates an epoll queue
    pub fn epoll_create(size: i32) -> i32;
    /// syscall that closes the file descriptor created by the epoll instance, in order to release resources properly
    pub fn close(fd: i32) -> i32;
    /// control interface used to perform operations on the epoll instance
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    /// syscall that blocks the current thread and waits until a notifcation that an event occurs or it times out.
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
#[repr(C)]
#[cfg_attr(target_arch = "x86_64", repr(packed))]
pub struct Event {
    pub(crate) events: u32,
    // token to identify event
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}
