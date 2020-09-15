


When this application is run with the Cargo.toml configuration of `panic = abort`
the following backtrace is supplied:

```sh
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/panic_abort`
thread 'main' panicked at 'rwlock read lock would result in deadlock', /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/macros.rs:13:23
stack backtrace:
   0:     0x55d49655f985 - backtrace::backtrace::libunwind::trace::h14d338b30b3ea0a7
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1:     0x55d49655f985 - backtrace::backtrace::trace_unsynchronized::h73ea91d74a3fd67f
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2:     0x55d49655f985 - std::sys_common::backtrace::_print_fmt::hd42948c952866e12
                               at src/libstd/sys_common/backtrace.rs:78
   3:     0x55d49655f985 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha8f928866ff7571e
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x55d49657c03c - core::fmt::write::he0c1e5f7426d2718
                               at src/libcore/fmt/mod.rs:1076
   5:     0x55d49655dd62 - std::io::Write::write_fmt::hf3afc6cfd57d0033
                               at src/libstd/io/mod.rs:1537
   6:     0x55d496561da0 - std::sys_common::backtrace::_print::hfc0110703f3696fd
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x55d496561da0 - std::sys_common::backtrace::print::h3f77c6990ddfaa22
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x55d496561da0 - std::panicking::default_hook::{{closure}}::heae49580a8d62d75
                               at src/libstd/panicking.rs:198
   9:     0x55d496561aec - std::panicking::default_hook::hecc34e3f729e213c
                               at src/libstd/panicking.rs:217
  10:     0x55d4965623e3 - std::panicking::rust_panic_with_hook::he82f5d0644692441
                               at src/libstd/panicking.rs:526
  11:     0x55d496558a19 - std::panicking::begin_panic::h54b82f26f8605773
                               at /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:456
  12:     0x55d496558da7 - std::sys::unix::rwlock::RWLock::read::hdf6526b69a03906b
                               at /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/macros.rs:13
```

When the same program is run w/o `panic = abort` the following backtrace is provided.

```sh
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/panic_abort`
thread 'main' panicked at 'rwlock read lock would result in deadlock', /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/macros.rs:13:23
stack backtrace:
   0:     0x562a85f9aeb5 - backtrace::backtrace::libunwind::trace::h14d338b30b3ea0a7
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1:     0x562a85f9aeb5 - backtrace::backtrace::trace_unsynchronized::h73ea91d74a3fd67f
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2:     0x562a85f9aeb5 - std::sys_common::backtrace::_print_fmt::hd42948c952866e12
                               at src/libstd/sys_common/backtrace.rs:78
   3:     0x562a85f9aeb5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha8f928866ff7571e
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x562a85fb7c2c - core::fmt::write::he0c1e5f7426d2718
                               at src/libcore/fmt/mod.rs:1076
   5:     0x562a85f99292 - std::io::Write::write_fmt::hf3afc6cfd57d0033
                               at src/libstd/io/mod.rs:1537
   6:     0x562a85f9d320 - std::sys_common::backtrace::_print::hfc0110703f3696fd
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x562a85f9d320 - std::sys_common::backtrace::print::h3f77c6990ddfaa22
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x562a85f9d320 - std::panicking::default_hook::{{closure}}::heae49580a8d62d75
                               at src/libstd/panicking.rs:198
   9:     0x562a85f9d06c - std::panicking::default_hook::hecc34e3f729e213c
                               at src/libstd/panicking.rs:217
  10:     0x562a85f9d963 - std::panicking::rust_panic_with_hook::he82f5d0644692441
                               at src/libstd/panicking.rs:526
  11:     0x562a85f912d5 - std::panicking::begin_panic::h370e18e3d6662bc4
                               at /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:456
  12:     0x562a85f93f47 - std::sys::unix::rwlock::RWLock::read::hdc6cf4fe72bf1ce7
                               at /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/macros.rs:13
  13:     0x562a85f9210a - std::sys_common::rwlock::RWLock::read::ha86caaca50e11302
                               at /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/sys_common/rwlock.rs:26
  14:     0x562a85f91939 - std::sync::rwlock::RwLock<T>::read::h6f90750d9eaca20d
                               at /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/sync/rwlock.rs:185
  15:     0x562a85f938f3 - panic_abort::f2::haaeb0879bea31e7c
                               at src/main.rs:11
  16:     0x562a85f93866 - panic_abort::f1::h4ddbe080d290c50f
                               at src/main.rs:6
  17:     0x562a85f93856 - panic_abort::main::hda6a7bb04125533e
                               at src/main.rs:2
  18:     0x562a85f928cb - std::rt::lang_start::{{closure}}::h99d424b0564450c3
                               at /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:67
  19:     0x562a85f9dd33 - std::rt::lang_start_internal::{{closure}}::h5d3ea623498f5f43
                               at src/libstd/rt.rs:52
  20:     0x562a85f9dd33 - std::panicking::try::do_call::hac65e71be769a440
                               at src/libstd/panicking.rs:348
  21:     0x562a85f9dd33 - std::panicking::try::hd4706e264bcf6712
                               at src/libstd/panicking.rs:325
  22:     0x562a85f9dd33 - std::panic::catch_unwind::h948a0fb4a8b3ee82
                               at src/libstd/panic.rs:394
  23:     0x562a85f9dd33 - std::rt::lang_start_internal::h72cc068ed2d0ac53
                               at src/libstd/rt.rs:51
  24:     0x562a85f928a7 - std::rt::lang_start::ha1aab72252c58928
                               at /home/rfm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/rt.rs:67
  25:     0x562a85f93afa - main
  26:     0x7f48a1bbd0b3 - __libc_start_main
  27:     0x562a85f9117e - _start
  28:                0x0 - <unknown>

```

Both backtraces are generated with the env var `RUST_BACKTRACE=full`

