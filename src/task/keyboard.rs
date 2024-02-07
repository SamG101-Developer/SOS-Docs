use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use core::{pin::Pin, task::{Context, Poll}};
use futures_util::{stream::{Stream, StreamExt}, task::AtomicWaker};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use crate::{print, println};


static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub fn add_scancode(scancode: u8) {
    match SCANCODE_QUEUE.try_get() {
        Ok(queue) if let Err(_) = queue.push(scancode) => { println!("Scancode queue full; dropping keyboard input"); }
        Ok(_) => { WAKER.wake() }
        _ => { println!("Scancode queue uninitialized; dropping keyboard input"); }
    }
}


pub struct ScancodeStream {
    _private: (),
}


impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100)).expect("Scancode queue already initialized");
        ScancodeStream { _private: () }
    }
}


impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let queue = SCANCODE_QUEUE.try_get().expect("Scancode queue uninitialized");

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode))
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}


pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => { println!("{}", character) }
                    DecodedKey::RawKey(key) => { println!("{:?}", key) }
                }
            }
        }
    }
}
