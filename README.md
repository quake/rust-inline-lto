# rust-inline-lto

build without inline
```
➜  main git:(master) ✗ rm -rf target
➜  main git:(master) ✗ cargo build --release && lldb target/release/main -o 'disassemble -n main::main' -o 'exit'
   Compiling foo v0.1.0 (/Users/quake/workspace/rust-inline-lto/foo)
   Compiling main v0.1.0 (/Users/quake/workspace/rust-inline-lto/main)
    Finished release [optimized + debuginfo] target(s) in 0.90s
(lldb) target create "target/release/main"
Current executable set to '/Users/quake/workspace/rust-inline-lto/main/target/release/main' (x86_64).
(lldb) disassemble -n main::main
main`main::main::h1f0b4e272c20853e:
main[0x100000f80] <+0>:  pushq  %rbp
main[0x100000f81] <+1>:  movq   %rsp, %rbp
main[0x100000f84] <+4>:  subq   $0x30, %rsp
main[0x100000f88] <+8>:  leaq   0x35b21(%rip), %rax
main[0x100000f8f] <+15>: movq   %rax, -0x10(%rbp)
main[0x100000f93] <+19>: movq   $0x20, -0x8(%rbp)
main[0x100000f9b] <+27>: leaq   -0x30(%rbp), %rdi
main[0x100000f9f] <+31>: leaq   -0x10(%rbp), %rsi
main[0x100000fa3] <+35>: callq  0x100001080               ; foo::Byte32::unpack::h7803ee3106c2225d at lib.rs:19
main[0x100000fa8] <+40>: movl   $0x20, %edi
main[0x100000fad] <+45>: callq  0x10001bbd0               ; std::process::exit::haaecf55f77bac6a4 at process.rs:1909
(lldb) exit
```

build with inline
```
➜  main git:(master) ✗ rm -rf target
➜  main git:(master) ✗ cargo build --release --features 'inline-unpack' && lldb target/release/main -o 'disassemble -n main::main' -o 'exit'
   Compiling foo v0.1.0 (/Users/quake/workspace/rust-inline-lto/foo)
   Compiling main v0.1.0 (/Users/quake/workspace/rust-inline-lto/main)
    Finished release [optimized + debuginfo] target(s) in 0.89s
(lldb) target create "target/release/main"
Current executable set to '/Users/quake/workspace/rust-inline-lto/main/target/release/main' (x86_64).
(lldb) disassemble -n main::main
main`main::main::h7e3f4163719a7ed8:
main[0x100000f80] <+0>:  pushq  %rbp
main[0x100000f81] <+1>:  movq   %rsp, %rbp
main[0x100000f84] <+4>:  subq   $0x30, %rsp
main[0x100000f88] <+8>:  leaq   0x35b21(%rip), %rax
main[0x100000f8f] <+15>: movq   %rax, -0x10(%rbp)
main[0x100000f93] <+19>: movq   $0x20, -0x8(%rbp)
main[0x100000f9b] <+27>: leaq   -0x30(%rbp), %rdi
main[0x100000f9f] <+31>: leaq   -0x10(%rbp), %rsi
main[0x100000fa3] <+35>: callq  0x100001080               ; foo::Byte32::unpack::h9cf825d29a1109f2 at lib.rs:19
main[0x100000fa8] <+40>: movl   $0x20, %edi
main[0x100000fad] <+45>: callq  0x10001bbd0               ; std::process::exit::haaecf55f77bac6a4 at process.rs:1909
(lldb) exit
```