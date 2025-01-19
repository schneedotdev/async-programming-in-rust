use crate::ffi;
use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd,
};

type Events = Vec<ffi::Event>;

/// represents the event queue itself
pub struct Poll {
    registry: Registry,
}

impl Poll {
    /// create a new event queue
    pub fn new() -> Result<Self> {
        // epoll_create expects some integer argument greater than 0 but will be ignored
        let res = unsafe { ffi::epoll_create(1) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            // holds epoll file descriptor
            registry: Registry { raw_fd: res },
        })
    }

    /// returns a reference to the registry that can be used to register interest to be notified about new events
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// blocks/parks the current thread until an event is ready or it times out, whichever occurs firs a handle that allows us to register interest in new events
    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let fd = self.registry.raw_fd;
        // if timeout is Some, we unwrap the value, else we pass -1 which tells the OS that we want to block until an event occurs even though that might never happen.
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;
        let res = unsafe { ffi::epoll_wait(fd, events.as_mut_ptr(), max_events, timeout) };

        if res < 0 {
            return Err(io::Error::last_os_error());
        };

        // This is safe because epoll_wait ensures that `res` events are assigned.
        unsafe { events.set_len(res as usize) };
        Ok(())
    }
}

/// handle that allows registering interest in new events
pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    /// register interest in new events based on interests passed as args that indicate which type of events the event queue should keep track of
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        let mut event = ffi::Event {
            events: interests as u32,
            epoll_data: token,
        };

        // defines operation - in this case: "ADD"
        let op = ffi::EPOLL_CTL_ADD;
        let res = unsafe { ffi::epoll_ctl(self.raw_fd, op, source.as_raw_fd(), &mut event) };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe { ffi::close(self.raw_fd) };

        if res < 0 {
            // mio logs error instead of panicking
            let err = io::Error::last_os_error();
            println!("ERROR: {err:?}");
        }
    }
}
