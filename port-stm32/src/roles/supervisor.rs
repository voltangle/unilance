use core_supervisor::State;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use littlefs2::consts::{U16, U256};
use littlefs2::driver::Storage;
use littlefs2::fs::{Allocation, Filesystem};
use littlefs2::io::Result;
use proc_macros::for_role;
use static_cell::StaticCell;

use crate::roles::MemChannelCoreLink;

static SUPERVISOR_STATE: StaticCell<Mutex<NoopRawMutex, State>> = StaticCell::new();
#[for_role("combined")]
type PlatformCoreLink<'a> = MemChannelCoreLink<'a>;
static SUPERVISOR_CORELINK: StaticCell<PlatformCoreLink> = StaticCell::new();

// TODO: All of this littlefs shit is very much "get out of my sight". Redo properly
static LITTLEFS_STORAGE: StaticCell<RamStorage> = StaticCell::new();
static LITTLEFS_ALLOC: StaticCell<Allocation<RamStorage>> = StaticCell::new();
static LITTLEFS: StaticCell<Mutex<NoopRawMutex, Filesystem<'static, RamStorage>>> =
    StaticCell::new();

const BLOCK_SIZE: usize = 4096;
const BLOCK_COUNT: usize = 2; // small for example

/// Start all control stuff. This function HAS to return, as its supposed to only spawn
/// tasks.
pub fn start(spawner: &Spawner, link: MemChannelCoreLink<'static>) {
    let state = SUPERVISOR_STATE.init(Mutex::new(State::new()));
    let corelink = SUPERVISOR_CORELINK.init(link);

    // 128 blocks × 4096 bytes = 512 KiB RAM filesystem
    let storage = LITTLEFS_STORAGE.init(RamStorage::new());

    // must format before first mount
    Filesystem::format(storage).unwrap();
    // must allocate state statically before use
    let alloc = LITTLEFS_ALLOC.init(Filesystem::allocate());
    let fs = Filesystem::mount(alloc, storage).unwrap();
    let fs_ref = LITTLEFS.init(Mutex::new(fs));

    spawner.spawn(
        corelink_heartbeat(state, corelink)
            .expect("CORElink heartbeat should be able to start"),
    );
    spawner.spawn(
        main_task(state, fs_ref, corelink).expect("Main task should be able to start"),
    );
    spawner
        .spawn(input_task(state, corelink).expect("Input task should be able to start"));
}

pub fn init() {
    // Nothing to do
}

#[for_role("combined")]
#[embassy_executor::task]
async fn corelink_heartbeat(
    state: &'static Mutex<NoopRawMutex, State>,
    link: &'static MemChannelCoreLink<'static>,
) {
    core_supervisor::corelink_heartbeat(state, link).await;
}

#[for_role("combined")]
#[embassy_executor::task]
async fn main_task(
    state: &'static Mutex<NoopRawMutex, State>,
    fs: &'static Mutex<NoopRawMutex, Filesystem<'static, RamStorage>>,
    link: &'static MemChannelCoreLink<'static>,
) {
    core_supervisor::main_task(state, fs, link).await;
}

#[embassy_executor::task]
async fn input_task(
    state: &'static Mutex<NoopRawMutex, State>,
    link: &'static MemChannelCoreLink<'static>,
) {
    core_supervisor::input_task(state, link).await;
}

// WARNING: This shit was pasted in from ChatGPT, because I didn't give enough fucks to
// make a proper littlefs driver implementation. Fuck you. That's a future me problem lmaooo

pub struct RamStorage {
    mem: [u8; BLOCK_SIZE * BLOCK_COUNT],
}

impl RamStorage {
    pub const fn new() -> Self {
        Self {
            mem: [0xFF; BLOCK_SIZE * BLOCK_COUNT],
        }
    }
}

impl Storage for RamStorage {
    const READ_SIZE: usize = 16;
    const WRITE_SIZE: usize = 16;

    const BLOCK_SIZE: usize = BLOCK_SIZE;
    const BLOCK_COUNT: usize = BLOCK_COUNT;
    const BLOCK_CYCLES: isize = -1;

    // Associated *types* (typenum / generic-array lengths)
    type CACHE_SIZE = U256; // bytes (must be multiple of READ_SIZE/WRITE_SIZE, factor of BLOCK_SIZE)
    type LOOKAHEAD_SIZE = U16; // number of u64s => 2 * 8 = 16 bytes lookahead

    fn read(&mut self, off: usize, buf: &mut [u8]) -> Result<usize> {
        // You can add bounds checks if you want; keeping it minimal here.
        buf.copy_from_slice(&self.mem[off..off + buf.len()]);
        Ok(buf.len())
    }

    fn write(&mut self, off: usize, data: &[u8]) -> Result<usize> {
        self.mem[off..off + data.len()].copy_from_slice(data);
        Ok(data.len())
    }

    fn erase(&mut self, off: usize, len: usize) -> Result<usize> {
        // LittleFS will call this with len multiple of BLOCK_SIZE.
        self.mem[off..off + len].fill(0xFF);
        Ok(len)
    }
}
