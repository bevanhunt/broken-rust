# Broken Rust

## Run
- `brew install rustup` (if needed)
- `cargo build`

## Backtrace

```bash
error: internal compiler error: unexpected region in query response: `ReScope(Destruction(216))`

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:354:17
note: Run with `RUST_BACKTRACE=full` environment variable to display a backtrace.

stack backtrace:
   0:        0x109ee97d3 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::hf2949dcf16da83bd
   1:        0x109ee1d32 - std::sys_common::backtrace::_print::h4b87ed6ba0cd5304
   2:        0x109ee5d76 - std::panicking::default_hook::{{closure}}::hde39fed432870543
   3:        0x109ee5b1f - std::panicking::default_hook::hfce8820d41dbdcbf
   4:        0x1087dc142 - rustc::util::common::panic_hook::h4f5124211bf4d99d
   5:        0x109ee65d0 - std::panicking::rust_panic_with_hook::hfcf2d0777bc6c409
   6:        0x109c30b14 - std::panicking::begin_panic::h0be534f66e16b25b
   7:        0x109c42253 - <rustc_errors::Handler as core::ops::drop::Drop>::drop::hdbb6a1b3ac33a4d0
   8:        0x105dff365 - core::ptr::real_drop_in_place::h06d3690d180c4f4e
   9:        0x105e0a4d5 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h83941e59b7d6a257
  10:        0x105dc730e - core::ptr::real_drop_in_place::h5195310840c69719
  11:        0x105dc33ec - rustc_interface::interface::run_compiler_in_existing_thread_pool::hb5d8ee0071615c8d
  12:        0x105da68e6 - std::thread::local::LocalKey<T>::with::h17e9facfd8094edc
  13:        0x105dfe5d5 - scoped_tls::ScopedKey<T>::set::hc604ecee3ae9cd95
  14:        0x105e2acc2 - syntax::with_globals::hd03b25ed65b3b40c
  15:        0x105dc4e66 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1de56c47cf04fdcc
  16:        0x109ef59ce - __rust_maybe_catch_panic
  17:        0x105dc5be6 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h7a351a099979b634
  18:        0x109ec850d - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hf8766009c029bc09
  19:        0x109ef488d - std::sys::unix::thread::Thread::new::thread_start::hfdef4649a42cf26c
  20:     0x7fff6d79e2ea - _pthread_body
  21:     0x7fff6d7a1248 - _pthread_start
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.35.0 (3c235d560 2019-05-20) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `excel_test`.
```

## System Info

```bash
rustc 1.35.0 (3c235d560 2019-05-20)
binary: rustc
commit-hash: 3c235d5600393dfe6c36eeed34042efad8d4f26e
commit-date: 2019-05-20
host: x86_64-apple-darwin
release: 1.35.0
LLVM version: 8.0
```