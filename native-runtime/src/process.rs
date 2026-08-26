// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The process runtime: BEAM-style lightweight processes as corosensei
//! fibers polled by a tokio multi-thread scheduler.
//!
//! Every process owns an isolated set of values — messages are deep-copied
//! on send ([`crate::deep_copy`]), so non-atomic reference counts never
//! cross threads on live values. What *is* shared between threads is the
//! [`Shared`] structure behind each process: its mailbox (a locked queue
//! of already-copied values), its links and monitors, and its life flags.
//!
//! A process suspends only inside this module — `receive`, `sleep`,
//! `yield`, and death — by yielding a [`Suspension`] to its scheduler
//! future. The future maps suspensions to wakers and tokio timers, and
//! rewrites the process's pool pointer to the current worker's pool on
//! every resume, so generated code always allocates from the pool of the
//! thread actually running it. Processes migrate freely between workers.
//!
//! Selective receive uses a save queue: messages scanned but not matched
//! move from the mailbox to a process-local queue in arrival order, and
//! every receive scans the save queue before the mailbox — so a receive
//! for one subject never loses or reorders messages for another.
//!
//! A crashed process (panic, `let assert` failure, bit array error) does
//! not unwind: generated frames carry no unwind information, so the fiber
//! yields a final [`Suspension::Died`] and its stack is deliberately
//! leaked. Its links and monitors are notified all the same. Reclaiming
//! crashed stacks is a planned hardening step.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::{
    NIL, ProcessContext, TRUE, box_string, deep_copy, gleam_native_dec, is_small_int, make_error,
    make_ok, make_tuple2, subject_payload, tag_small_int,
};

/// The stack size (in bytes) every process fiber reserves, set from the
/// project configuration before the root process starts. Reservations are
/// virtual — pages commit lazily — so a generous size costs address
/// space, not memory.
pub(crate) static PROCESS_STACK_SIZE: AtomicUsize = AtomicUsize::new(1024 * 1024);

static NEXT_PID: AtomicU64 = AtomicU64::new(1);
static NEXT_TAG: AtomicU64 = AtomicU64::new(1);

/// Live processes by pid, for `link`/`monitor` lookups. A process is
/// removed when it finishes; senders holding a subject reach the mailbox
/// directly and never consult the table.
static PROCESS_TABLE: Mutex<Option<std::collections::HashMap<u64, Arc<Shared>>>> =
    Mutex::new(None);

fn table_insert(shared: &Arc<Shared>) {
    let mut table = PROCESS_TABLE.lock().expect("process table");
    table
        .get_or_insert_with(std::collections::HashMap::new)
        .insert(shared.pid, shared.clone());
}

fn table_remove(pid: u64) {
    let mut table = PROCESS_TABLE.lock().expect("process table");
    if let Some(table) = table.as_mut() {
        let _ = table.remove(&pid);
    }
}

fn table_lookup(pid: u64) -> Option<Arc<Shared>> {
    let table = PROCESS_TABLE.lock().expect("process table");
    table.as_ref().and_then(|table| table.get(&pid).cloned())
}

/// A message in flight: its subject tag and the (already deep-copied,
/// owned) value. Dropping an undelivered envelope releases the value.
pub struct Envelope {
    pub tag: u64,
    value: u64,
}

impl Envelope {
    /// Takes ownership of the value out of the envelope.
    pub fn take_value(self) -> u64 {
        let value = self.value;
        std::mem::forget(self);
        value
    }
}

impl Drop for Envelope {
    fn drop(&mut self) {
        let _ = gleam_native_dec(self.value);
    }
}

struct MailboxInner {
    queue: VecDeque<Envelope>,
    /// Incremented on every push; a parked receiver records the version it
    /// scanned so the scheduler can tell whether anything arrived between
    /// the scan and the park.
    version: u64,
    waker: Option<std::task::Waker>,
}

/// The cross-thread face of a process: everything senders, links, and
/// monitors touch. The mailbox holds deep-copied values, so the only
/// cross-thread traffic on Gleam values is the handoff of exclusive
/// ownership through the queue's lock.
pub struct Shared {
    pub(crate) pid: u64,
    alive: AtomicBool,
    /// An exit signal delivered but not yet acted on: the process dies
    /// with this reason at its next suspension point.
    killed: Mutex<Option<String>>,
    /// Zero when exits are not trapped; otherwise the subject tag exit
    /// messages are delivered under instead of killing the process.
    trap_tag: AtomicU64,
    mailbox: Mutex<MailboxInner>,
    /// Processes linked to this one (bidirectional; both sides hold an
    /// entry).
    links: Mutex<Vec<Arc<Shared>>>,
    /// Watchers to notify when this process finishes: the subject tag the
    /// down message is delivered under, and the watching process.
    monitors: Mutex<Vec<(u64, Arc<Shared>)>>,
}

// SAFETY: the u64 values inside the mailbox are exclusively-owned Gleam
// values whose ownership transfers through the queue's lock; no live value
// is ever reachable from two threads at once.
unsafe impl Send for Shared {}
unsafe impl Sync for Shared {}

impl Shared {
    fn new(pid: u64) -> Self {
        Shared {
            pid,
            alive: AtomicBool::new(true),
            killed: Mutex::new(None),
            trap_tag: AtomicU64::new(0),
            mailbox: Mutex::new(MailboxInner {
                queue: VecDeque::new(),
                version: 0,
                waker: None,
            }),
            links: Mutex::new(Vec::new()),
            monitors: Mutex::new(Vec::new()),
        }
    }
}

/// The payload of a subject value ([`crate::KIND_SUBJECT`]): the process
/// the subject delivers to and the tag its messages carry. Copying a
/// subject between processes clones the handle — subjects are the one
/// value deliberately shared across heaps, which is safe because
/// [`Shared`] is internally synchronized.
pub struct SubjectPayload {
    pub(crate) shared: Arc<Shared>,
    pub(crate) tag: u64,
}

impl Clone for SubjectPayload {
    fn clone(&self) -> Self {
        SubjectPayload {
            shared: self.shared.clone(),
            tag: self.tag,
        }
    }
}

/// Why a fiber handed control back to its scheduler future.
enum Suspension {
    /// Parked in a receive: nothing matched at mailbox version
    /// `seen_version`; wake on new mail (or at the deadline).
    Receive {
        seen_version: u64,
        deadline: Option<Instant>,
    },
    Sleep {
        until: Instant,
    },
    Yielded,
    /// The process crashed (or was killed): notify links and monitors and
    /// abandon the fiber without unwinding it.
    Died(String),
}

type Coroutine = corosensei::Coroutine<(), Suspension, ()>;
type Yielder = corosensei::Yielder<(), Suspension>;

/// The scheduler-side state of one process, owned by its future. The
/// fiber reaches it through [`CURRENT_PROCESS`] while running.
pub(crate) struct ProcessState {
    pub(crate) shared: Arc<Shared>,
    pub(crate) context: ProcessContext,
    /// The fiber's yielder, stored at fiber start: how runtime calls deep
    /// inside Gleam frames suspend. Stable for the fiber's whole life.
    yielder: Cell<*const Yielder>,
    /// Messages scanned but not matched by earlier receives, in arrival
    /// order; scanned before the mailbox by every receive.
    save_queue: RefCell<VecDeque<Envelope>>,
    root: bool,
}

thread_local! {
    /// The process currently running on this thread, set by its future
    /// around every resume.
    static CURRENT_PROCESS: Cell<*const ProcessState> = const { Cell::new(std::ptr::null()) };
}

/// The running process's state. Must only be called from a fiber (any
/// process external reaching here from outside one is a runtime bug).
fn current() -> &'static ProcessState {
    let state = CURRENT_PROCESS.with(|current| current.get());
    assert!(
        !state.is_null(),
        "process operation outside a Gleam process"
    );
    unsafe { &*state }
}

/// Whether the caller is inside a non-root process fiber — where a crash
/// should kill the process, not the program.
pub fn in_child_process() -> bool {
    let state = CURRENT_PROCESS.with(|current| current.get());
    !state.is_null() && !unsafe { &*state }.root
}

/// Suspends the running fiber with the given reason, returning when the
/// scheduler resumes it. On return, a delivered exit signal (from a link,
/// while not trapping) terminates the process here — the signal's whole
/// point is to interrupt waits.
fn fiber_suspend(state: &ProcessState, suspension: Suspension) {
    let yielder = state.yielder.get();
    assert!(!yielder.is_null(), "fiber suspending before its start");
    unsafe { (*yielder).suspend(suspension) };
    check_killed(state);
}

fn check_killed(state: &ProcessState) {
    let killed = state.shared.killed.lock().expect("kill flag").take();
    if let Some(reason) = killed {
        die(state, reason);
    }
}

/// Terminates the running process abnormally: yields the final
/// [`Suspension::Died`] and never returns — the scheduler abandons the
/// fiber without resuming it.
fn die(state: &ProcessState, reason: String) -> ! {
    let yielder = state.yielder.get();
    unsafe { (*yielder).suspend(Suspension::Died(reason)) };
    unreachable!("a dead fiber was resumed");
}

/// Terminates the current process abnormally when running inside a child
/// process; exits the program otherwise. The tail of every runtime error
/// report (`panic`, failed asserts, bit array errors).
pub fn exit_current_abnormally(reason: String) -> ! {
    let state = CURRENT_PROCESS.with(|current| current.get());
    if !state.is_null() && !unsafe { &*state }.root {
        die(unsafe { &*state }, reason);
    }
    std::process::exit(1);
}

/// Delivers an already-copied, owned value to a process under a subject
/// tag. Callable from any thread. Delivery to a finished process releases
/// the value instead.
fn send_raw(shared: &Shared, tag: u64, value: u64) {
    if !shared.alive.load(Ordering::Acquire) {
        let _ = gleam_native_dec(value);
        return;
    }
    let waker = {
        let mut mailbox = shared.mailbox.lock().expect("mailbox");
        mailbox.queue.push_back(Envelope { tag, value });
        mailbox.version += 1;
        mailbox.waker.take()
        // A finish() racing this push drains the queue under the same
        // lock afterwards, or the envelope is released when the mailbox
        // itself drops; either way nothing leaks.
    };
    if let Some(waker) = waker {
        waker.wake();
    }
}

/// Delivers an exit signal to a process: if it traps exits, an
/// `#(pid, reason)` message on its trap subject; otherwise it is killed —
/// marked to die at its next suspension point, and woken if parked.
fn deliver_exit(target: &Shared, from_pid: u64, reason: &str) {
    let trap_tag = target.trap_tag.load(Ordering::Acquire);
    if trap_tag != 0 {
        let message = make_tuple2(tag_small_int(from_pid as i64), box_string(reason));
        send_raw(target, trap_tag, message);
        return;
    }
    if reason == "normal" {
        // A normal exit kills no one; only trapping processes observe it.
        return;
    }
    *target.killed.lock().expect("kill flag") = Some(format!("killed: {reason}"));
    let waker = target.mailbox.lock().expect("mailbox").waker.take();
    if let Some(waker) = waker {
        waker.wake();
    }
}

/// Marks a process finished and notifies everyone watching: monitors get
/// a down message (the reason as a string), links get an exit signal.
/// Undelivered mail is released.
fn finish(state: &ProcessState, reason: &str) {
    let shared = &state.shared;
    shared.alive.store(false, Ordering::Release);
    table_remove(shared.pid);
    // Drain the mailbox under its lock: a sender that saw `alive` just
    // before the store has either pushed already (drained here) or will
    // find `alive` false. Envelopes release their values on drop.
    {
        let mut mailbox = shared.mailbox.lock().expect("mailbox");
        mailbox.queue.clear();
        mailbox.waker = None;
    }
    state.save_queue.borrow_mut().clear();
    let monitors = std::mem::take(&mut *shared.monitors.lock().expect("monitors"));
    for (tag, watcher) in monitors {
        send_raw(&watcher, tag, box_string(reason));
    }
    let links = std::mem::take(&mut *shared.links.lock().expect("links"));
    for linked in links {
        // Drop our entry on the other side so a later death over there
        // does not signal a finished process.
        linked
            .links
            .lock()
            .expect("links")
            .retain(|entry| entry.pid != shared.pid);
        deliver_exit(&linked, shared.pid, reason);
    }
}

/// The future driving one process fiber. Each poll pins the process to
/// the current worker (pool pointer, thread-local context) and resumes
/// it; suspensions become wakers and timers.
pub(crate) struct ProcessFuture {
    coroutine: Option<Coroutine>,
    state: Box<ProcessState>,
    /// The armed receive-deadline or sleep timer, if any.
    timer: Option<std::pin::Pin<Box<tokio::time::Sleep>>>,
    finished: bool,
}

// SAFETY: the fiber suspends only inside this module, which never holds a
// thread-local borrow across a suspension; the context's pool is rewritten
// to the running worker's pool on every resume. Values on the fiber stack
// belong exclusively to this process (messages are deep-copied on send).
unsafe impl Send for ProcessFuture {}

impl std::future::Future for ProcessFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        task: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let this = self.get_mut();
        if this.finished {
            return std::task::Poll::Ready(());
        }
        this.timer = None;
        this.state.context.pool = crate::current_pool();
        crate::set_current_context(&raw mut this.state.context);
        CURRENT_PROCESS.with(|current| current.set(&raw const *this.state));
        let coroutine = this.coroutine.as_mut().expect("live coroutine");
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            coroutine.resume(())
        }));
        CURRENT_PROCESS.with(|current| current.set(std::ptr::null()));
        crate::set_current_context(std::ptr::null_mut());
        match result {
            Ok(corosensei::CoroutineResult::Return(())) => {
                finish(&this.state, "normal");
                this.finished = true;
                std::task::Poll::Ready(())
            }
            Ok(corosensei::CoroutineResult::Yield(Suspension::Died(reason))) => {
                finish(&this.state, &reason);
                this.finished = true;
                // The fiber is suspended mid-crash and must never be
                // unwound; forget it, deliberately leaking its stack.
                std::mem::forget(this.coroutine.take());
                std::task::Poll::Ready(())
            }
            Ok(corosensei::CoroutineResult::Yield(Suspension::Yielded)) => {
                task.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Ok(corosensei::CoroutineResult::Yield(Suspension::Sleep { until })) => {
                this.arm_timer(until, task);
                std::task::Poll::Pending
            }
            Ok(corosensei::CoroutineResult::Yield(Suspension::Receive {
                seen_version,
                deadline,
            })) => {
                let stale = {
                    let mut mailbox = this.state.shared.mailbox.lock().expect("mailbox");
                    mailbox.waker = Some(task.waker().clone());
                    mailbox.version != seen_version
                };
                if stale {
                    // Mail arrived between the fiber's scan and the park.
                    task.waker().wake_by_ref();
                } else if let Some(deadline) = deadline {
                    this.arm_timer(deadline, task);
                }
                std::task::Poll::Pending
            }
            Err(panic) => {
                // A Rust panic unwound out of the fiber (a runtime bug or
                // a std failure — Gleam crashes never unwind). The fiber
                // has fully unwound, so the coroutine is safely droppable.
                finish(&this.state, "rust panic");
                this.finished = true;
                if this.state.root {
                    std::panic::resume_unwind(panic);
                }
                eprintln!("process {} crashed: rust panic", this.state.shared.pid);
                std::task::Poll::Ready(())
            }
        }
    }
}

impl ProcessFuture {
    fn arm_timer(&mut self, until: Instant, task: &mut std::task::Context<'_>) {
        let mut timer = Box::pin(tokio::time::sleep_until(tokio::time::Instant::from_std(
            until,
        )));
        if timer.as_mut().poll(task).is_ready() {
            task.waker().wake_by_ref();
        }
        self.timer = Some(timer);
    }
}

impl Drop for ProcessFuture {
    fn drop(&mut self) {
        // A future dropped mid-life (scheduler shutdown after the root
        // process finished) holds a suspended fiber whose Gleam frames
        // cannot be unwound: leak it, and tell its watchers.
        if let Some(coroutine) = self.coroutine.take() {
            if self.finished || coroutine.done() {
                drop(coroutine);
            } else {
                finish(&self.state, "shutdown");
                std::mem::forget(coroutine);
            }
        }
    }
}

use std::future::Future;

/// Builds a process and its scheduler future. The body runs on a fresh
/// guard-paged fiber stack; the caller decides where the future is
/// spawned.
pub(crate) fn new_process(
    root: bool,
    body: impl FnOnce() + 'static,
) -> Result<(Arc<Shared>, ProcessFuture), String> {
    let stack_size = PROCESS_STACK_SIZE.load(Ordering::Relaxed);
    let stack = corosensei::stack::DefaultStack::new(stack_size)
        .map_err(|error| format!("could not allocate a process stack: {error}"))?;
    let (stack_high, stack_low) = {
        use corosensei::stack::Stack;
        (stack.base().get(), stack.limit().get())
    };
    let pid = NEXT_PID.fetch_add(1, Ordering::Relaxed);
    let shared = Arc::new(Shared::new(pid));
    let state = Box::new(ProcessState {
        shared: shared.clone(),
        context: ProcessContext {
            pool: std::ptr::null(),
            stack_low,
            stack_high,
            reductions: 0,
        },
        yielder: Cell::new(std::ptr::null()),
        save_queue: RefCell::new(VecDeque::new()),
        root,
    });
    table_insert(&shared);
    let coroutine = Coroutine::with_stack(stack, move |yielder, ()| {
        // Publish the yielder so runtime calls deep inside Gleam frames
        // can suspend; the state is found through the thread-local the
        // scheduler set before this resume.
        current().yielder.set(yielder as *const Yielder);
        body();
    });
    Ok((
        shared,
        ProcessFuture {
            coroutine: Some(coroutine),
            state,
            timer: None,
            finished: false,
        },
    ))
}

/// Spawns a child process onto the running scheduler, returning its
/// shared handle. Used by the test runner; Gleam code spawns through the
/// `gleam_native_process_spawn` external.
pub fn spawn_child(body: impl FnOnce() + 'static) -> Arc<Shared> {
    let (shared, future) = new_process(false, body).expect("spawn a process");
    drop(tokio::spawn(future));
    shared
}

/// Registers a monitor on the given process from the current one,
/// returning the tag its down message (the exit reason as a string) will
/// arrive under. An already-finished process is reported down
/// immediately.
pub fn monitor_arc(target: &Arc<Shared>) -> u64 {
    let state = current();
    let tag = NEXT_TAG.fetch_add(1, Ordering::Relaxed);
    if target.alive.load(Ordering::Acquire) {
        target
            .monitors
            .lock()
            .expect("monitors")
            .push((tag, state.shared.clone()));
        // The process may have finished between the check and the push;
        // its finish() drained monitors it saw. Re-check and deliver the
        // down ourselves if our entry can no longer be seen.
        if !target.alive.load(Ordering::Acquire) {
            let still_registered = target
                .monitors
                .lock()
                .expect("monitors")
                .iter()
                .any(|(entry, _)| *entry == tag);
            if still_registered {
                // finish() ran before our push: it will never see this
                // entry, so deliver the down here.
                target
                    .monitors
                    .lock()
                    .expect("monitors")
                    .retain(|(entry, _)| *entry != tag);
                send_raw(&state.shared, tag, box_string("noproc"));
            }
        }
    } else {
        send_raw(&state.shared, tag, box_string("noproc"));
    }
    tag
}

/// Receives the next message carrying one of the given tags, scanning the
/// save queue then the mailbox and parking until a match or the timeout.
/// Returns `None` on timeout.
pub fn receive_tags(tags: &[u64], timeout: Option<Duration>) -> Option<Envelope> {
    let state = current();
    let deadline = timeout.map(|timeout| Instant::now() + timeout);
    loop {
        check_killed(state);
        {
            let mut saved = state.save_queue.borrow_mut();
            if let Some(position) = saved.iter().position(|envelope| tags.contains(&envelope.tag))
            {
                return saved.remove(position);
            }
        }
        let seen_version = {
            let mut mailbox = state.shared.mailbox.lock().expect("mailbox");
            loop {
                match mailbox.queue.pop_front() {
                    Some(envelope) if tags.contains(&envelope.tag) => return Some(envelope),
                    Some(envelope) => state.save_queue.borrow_mut().push_back(envelope),
                    None => break,
                }
            }
            mailbox.version
        };
        if let Some(deadline) = deadline
            && Instant::now() >= deadline
        {
            return None;
        }
        fiber_suspend(
            state,
            Suspension::Receive {
                seen_version,
                deadline,
            },
        );
    }
}

// ---------------------------------------------------------------------------
// The externals generated code calls. All follow the external calling
// convention: arguments are borrowed, results are owned.

/// The current process's pid.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_self() -> u64 {
    tag_small_int(current().shared.pid as i64)
}

/// Spawns a process running the given zero-argument closure, optionally
/// linked to the current one. The closure is deep-copied into the new
/// process. Returns the new pid.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_spawn(closure: u64, link: u64) -> u64 {
    let state = current();
    let copied = deep_copy(closure);
    let (shared, future) = match new_process(false, move || {
        let result = crate::call_closure(copied, &[]);
        let _ = gleam_native_dec(result);
        let _ = gleam_native_dec(copied);
    }) {
        Ok(spawned) => spawned,
        Err(error) => {
            eprintln!("runtime error: {error}");
            std::process::exit(1);
        }
    };
    if link == TRUE {
        state.shared.links.lock().expect("links").push(shared.clone());
        shared
            .links
            .lock()
            .expect("links")
            .push(state.shared.clone());
    }
    let pid = shared.pid;
    drop(tokio::spawn(future));
    tag_small_int(pid as i64)
}

/// A fresh subject delivering to the current process.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_subject_new() -> u64 {
    let state = current();
    crate::box_subject(SubjectPayload {
        shared: state.shared.clone(),
        tag: NEXT_TAG.fetch_add(1, Ordering::Relaxed),
    })
}

/// Sends a message on a subject: the value is deep-copied into the
/// receiving process's mailbox. Sending to a finished process discards
/// the message.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_send(subject: u64, message: u64) -> u64 {
    let payload = subject_payload(subject);
    send_raw(&payload.shared, payload.tag, deep_copy(message));
    NIL
}

/// The milliseconds in a tagged timeout value; negative means forever.
fn untag_timeout(timeout_ms: u64) -> Option<Duration> {
    assert!(is_small_int(timeout_ms), "timeouts are small integers");
    let ms = (timeout_ms as i64) >> 1;
    if ms < 0 {
        None
    } else {
        Some(Duration::from_millis(ms as u64))
    }
}

/// Receives the next message on a subject within the timeout:
/// `Ok(message)`, or `Error(Nil)` on timeout.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_receive(subject: u64, timeout_ms: u64) -> u64 {
    let payload = subject_payload(subject);
    match receive_tags(&[payload.tag], untag_timeout(timeout_ms)) {
        Some(envelope) => make_ok(envelope.take_value()),
        None => make_error(NIL),
    }
}

/// Receives the next message on a subject, waiting as long as it takes.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_receive_forever(subject: u64) -> u64 {
    let payload = subject_payload(subject);
    receive_tags(&[payload.tag], None)
        .expect("a receive without a timeout always yields a message")
        .take_value()
}

/// Receives the next message on any subject in the given list within the
/// timeout: `Ok(#(index, message))` with the subject's position, or
/// `Error(Nil)` on timeout. The mailbox is scanned in arrival order — the
/// selective-receive substrate selectors build on.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_select(subjects: u64, timeout_ms: u64) -> u64 {
    let mut tags = Vec::new();
    let mut list = subjects;
    while !crate::is_immediate(list) {
        tags.push(subject_payload(crate::record_field(list, 0)).tag);
        list = crate::record_field(list, 1);
    }
    match receive_tags(&tags, untag_timeout(timeout_ms)) {
        Some(envelope) => {
            let index = tags
                .iter()
                .position(|tag| *tag == envelope.tag)
                .expect("a received tag was selected");
            make_ok(make_tuple2(
                tag_small_int(index as i64),
                envelope.take_value(),
            ))
        }
        None => make_error(NIL),
    }
}

/// Suspends the current process for the given number of milliseconds.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_sleep(milliseconds: u64) -> u64 {
    let state = current();
    assert!(is_small_int(milliseconds), "sleep takes a small integer");
    let ms = ((milliseconds as i64) >> 1).max(0) as u64;
    let until = Instant::now() + Duration::from_millis(ms);
    while Instant::now() < until {
        fiber_suspend(state, Suspension::Sleep { until });
    }
    NIL
}

/// Yields the scheduler: the process goes to the back of the run queue.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_yield() -> u64 {
    fiber_suspend(current(), Suspension::Yielded);
    NIL
}

/// Monitors the process with the given pid: returns a subject on which a
/// single down message (the exit reason as a string) arrives when it
/// finishes — immediately, with reason `"noproc"`, if it already has.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_monitor(pid: u64) -> u64 {
    let state = current();
    let target = table_lookup((pid as i64 >> 1) as u64);
    let tag = match target {
        Some(target) => monitor_arc(&target),
        None => {
            let tag = NEXT_TAG.fetch_add(1, Ordering::Relaxed);
            send_raw(&state.shared, tag, box_string("noproc"));
            tag
        }
    };
    crate::box_subject(SubjectPayload {
        shared: state.shared.clone(),
        tag,
    })
}

/// Links the current process to the one with the given pid (idempotent
/// per call; both sides record the link). Linking to a finished process
/// delivers an immediate `noproc` exit signal.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_link(pid: u64) -> u64 {
    let state = current();
    match table_lookup((pid as i64 >> 1) as u64) {
        Some(target) => {
            state.shared.links.lock().expect("links").push(target.clone());
            target
                .links
                .lock()
                .expect("links")
                .push(state.shared.clone());
        }
        None => {
            deliver_exit(&state.shared, (pid as i64 >> 1) as u64, "noproc");
            check_killed(state);
        }
    }
    NIL
}

/// Removes any link between the current process and the given pid.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_unlink(pid: u64) -> u64 {
    let state = current();
    let pid = (pid as i64 >> 1) as u64;
    state
        .shared
        .links
        .lock()
        .expect("links")
        .retain(|entry| entry.pid != pid);
    if let Some(target) = table_lookup(pid) {
        target
            .links
            .lock()
            .expect("links")
            .retain(|entry| entry.pid != state.shared.pid);
    }
    NIL
}

/// Makes the current process trap exits: instead of being killed by a
/// linked process's death, it receives `#(pid, reason)` messages on the
/// returned subject.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_trap_exits() -> u64 {
    let state = current();
    let tag = NEXT_TAG.fetch_add(1, Ordering::Relaxed);
    state.shared.trap_tag.store(tag, Ordering::Release);
    crate::box_subject(SubjectPayload {
        shared: state.shared.clone(),
        tag,
    })
}

/// Whether the process with the given pid is still running.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_is_alive(pid: u64) -> u64 {
    if table_lookup((pid as i64 >> 1) as u64).is_some() {
        TRUE
    } else {
        crate::FALSE
    }
}

/// A small integer identifying the worker thread currently running the
/// process — for observing scheduling (work stealing) in tests and tools.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_process_scheduler_id() -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::hash::DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    tag_small_int((hasher.finish() % 1_000_000) as i64)
}

/// The symbols of the process externals, for the JIT's symbol table.
pub fn process_symbols() -> Vec<(&'static str, *const u8)> {
    vec![
        (
            "gleam_native_process_self",
            gleam_native_process_self as *const u8,
        ),
        (
            "gleam_native_process_spawn",
            gleam_native_process_spawn as *const u8,
        ),
        (
            "gleam_native_process_subject_new",
            gleam_native_process_subject_new as *const u8,
        ),
        (
            "gleam_native_process_send",
            gleam_native_process_send as *const u8,
        ),
        (
            "gleam_native_process_receive",
            gleam_native_process_receive as *const u8,
        ),
        (
            "gleam_native_process_receive_forever",
            gleam_native_process_receive_forever as *const u8,
        ),
        (
            "gleam_native_process_select",
            gleam_native_process_select as *const u8,
        ),
        (
            "gleam_native_process_sleep",
            gleam_native_process_sleep as *const u8,
        ),
        (
            "gleam_native_process_yield",
            gleam_native_process_yield as *const u8,
        ),
        (
            "gleam_native_process_monitor",
            gleam_native_process_monitor as *const u8,
        ),
        (
            "gleam_native_process_link",
            gleam_native_process_link as *const u8,
        ),
        (
            "gleam_native_process_unlink",
            gleam_native_process_unlink as *const u8,
        ),
        (
            "gleam_native_process_trap_exits",
            gleam_native_process_trap_exits as *const u8,
        ),
        (
            "gleam_native_process_is_alive",
            gleam_native_process_is_alive as *const u8,
        ),
        (
            "gleam_native_process_scheduler_id",
            gleam_native_process_scheduler_id as *const u8,
        ),
    ]
}
