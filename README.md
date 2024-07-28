<div align="center">

# Little Emulator

[![Rust](https://img.shields.io/badge/Rust-f74c00?logo=rust)](https://www.rust-lang.org)
[![Brain made](https://img.shields.io/badge/Brainmade-grey?logo=data:image/svg%2bxml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjxzdmcKICAgd2lkdGg9IjY2LjUzOCIKICAgaGVpZ2h0PSI3OC43ODIiCiAgIGZpbGw9Im5vbmUiCiAgIHZlcnNpb249IjEuMSIKICAgaWQ9InN2ZzMiCiAgIHNvZGlwb2RpOmRvY25hbWU9IndoaXRlLWxvZ28taGVhZDMuc3ZnIgogICBpbmtzY2FwZTp2ZXJzaW9uPSIxLjMuMiAoMDkxZTIwZWYwZiwgMjAyMy0xMS0yNSwgY3VzdG9tKSIKICAgeG1sbnM6aW5rc2NhcGU9Imh0dHA6Ly93d3cuaW5rc2NhcGUub3JnL25hbWVzcGFjZXMvaW5rc2NhcGUiCiAgIHhtbG5zOnNvZGlwb2RpPSJodHRwOi8vc29kaXBvZGkuc291cmNlZm9yZ2UubmV0L0RURC9zb2RpcG9kaS0wLmR0ZCIKICAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogICB4bWxuczpzdmc9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8c29kaXBvZGk6bmFtZWR2aWV3CiAgICAgaWQ9Im5hbWVkdmlldzMiCiAgICAgcGFnZWNvbG9yPSIjZmZmZmZmIgogICAgIGJvcmRlcmNvbG9yPSIjMDAwMDAwIgogICAgIGJvcmRlcm9wYWNpdHk9IjAuMjUiCiAgICAgaW5rc2NhcGU6c2hvd3BhZ2VzaGFkb3c9IjIiCiAgICAgaW5rc2NhcGU6cGFnZW9wYWNpdHk9IjAuMCIKICAgICBpbmtzY2FwZTpwYWdlY2hlY2tlcmJvYXJkPSIwIgogICAgIGlua3NjYXBlOmRlc2tjb2xvcj0iI2QxZDFkMSIKICAgICBpbmtzY2FwZTp6b29tPSIxMC43NjM4OCIKICAgICBpbmtzY2FwZTpjeD0iMzMuMjU5MzgxIgogICAgIGlua3NjYXBlOmN5PSIzOS4zOTA5OTkiCiAgICAgaW5rc2NhcGU6d2luZG93LXdpZHRoPSIxOTIwIgogICAgIGlua3NjYXBlOndpbmRvdy1oZWlnaHQ9IjEwNTIiCiAgICAgaW5rc2NhcGU6d2luZG93LXg9IjAiCiAgICAgaW5rc2NhcGU6d2luZG93LXk9IjAiCiAgICAgaW5rc2NhcGU6d2luZG93LW1heGltaXplZD0iMSIKICAgICBpbmtzY2FwZTpjdXJyZW50LWxheWVyPSJzdmczIiAvPgogIDxnCiAgICAgY2xpcC1wYXRoPSJ1cmwoI2EpIgogICAgIHN0eWxlPSJmaWxsOiNmZmZmZmYiCiAgICAgZmlsbD0iI2ZmZiIKICAgICBpZD0iZzIiPgogICAgPHBhdGgKICAgICAgIGQ9Ik01Mi42MTIgNzguNzgySDIzLjMzYTIuNTU5IDIuNTU5IDAgMCAxLTIuNTYtMi41NTl2LTcuNjc2aC03Ljk3M2EyLjU2IDIuNTYgMCAwIDEtMi41Ni0yLjU2VjU1LjMxNWwtOC44Mi00LjM5N2EyLjU1OSAyLjU1OSAwIDAgMS0uOTg2LTMuNzFsOS44MDctMTQuNzE0di00LjM1QzEwLjI0IDEyLjU5OSAyMi44NDMgMCAzOC4zODggMCA1My45MzIgMCA2Ni41MzQgMTIuNiA2Ni41MzggMjguMTQzYy0uNjMyIDI3LjgyNC0xMC43NiAyMy41MTYtMTEuMTggMzQuMDQ1bC0uMTg3IDE0LjAzNWEyLjU5IDIuNTkgMCAwIDEtLjc1IDEuODEgMi41NSAyLjU1IDAgMCAxLTEuODA5Ljc1em0tMjYuNzIzLTUuMTE3aDI0LjE2NGwuMjg2LTE0LjU0MmMtLjI2My02LjY1NiAxMS43MTYtOC4yNDMgMTEuMDgtMzAuNzM0LS4zNTgtMTIuNzEzLTEwLjMxMy0yMy4yNzEtMjMuMDMxLTIzLjI3MS0xMi43MTggMC0yMy4wMjkgMTAuMzA3LTIzLjAzMiAyMy4wMjV2NS4xMThjMCAuNTA1LS4xNS45OTktLjQzIDEuNDJsLTguNjMgMTIuOTQgNy42NDUgMy44MmEyLjU1OSAyLjU1OSAwIDAgMSAxLjQxNSAyLjI5MXY5LjY5N2g3Ljk3NGEyLjU1OSAyLjU1OSAwIDAgMSAyLjU2IDIuNTU5eiIKICAgICAgIHN0eWxlPSJmaWxsOiNmZmZmZmYiCiAgICAgICBpZD0icGF0aDEiIC8+CiAgICA8cGF0aAogICAgICAgZD0iTTQwLjM3MiA1OC4yMjJWMzguOTM0Yy4xMTggMCAuMjM3LjAxOC4zNTUuMDE4IDkuNzY5LS4wMTIgMTcuMDUtOS4wMTIgMTUuMDIyLTE4LjU2N2EyLjM2NiAyLjM2NiAwIDAgMC0xLjgyMS0xLjgyMmMtOC4xMDYtMS43My0xNi4xMjEgMy4yOTItMTguMDk4IDExLjM0MS0uMDI0LS4wMjQtLjA0My0uMDUtLjA2Ni0uMDczYTE1LjMyMyAxNS4zMjMgMCAwIDAtMTQuMDYtNC4xNyAyLjM2NSAyLjM2NSAwIDAgMC0xLjgyMSAxLjgyYy0yLjAyOCA5LjU1NSA1LjI1MiAxOC41NTQgMTUuMDIgMTguNTY4LjIzNiAwIC40OTItLjAyOC43MzgtLjA0djEyLjIxM1ptMi44MzktMzIuMTQzYTEwLjY0NiAxMC42NDYgMCAwIDEgOC4xMjQtMy4xMDZjLjM1IDYuMzQtNC44ODggMTEuNTc3LTExLjIyOCAxMS4yM2ExMC41OCAxMC41OCAwIDAgMSAzLjEwNC04LjEyNHpNMjcuNDAzIDM4LjE5M2ExMC42MDcgMTAuNjA3IDAgMCAxLTMuMTE4LTguMTIzYzYuMzQ0LS4zNTggMTEuNTg3IDQuODg2IDExLjIyOCAxMS4yMy0zLjAyMy4xNjktNS45NzMtLjk2MS04LjExLTMuMTA3eiIKICAgICAgIHN0eWxlPSJmaWxsOiNmZmZmZmYiCiAgICAgICBpZD0icGF0aDIiIC8+CiAgPC9nPgogIDxkZWZzCiAgICAgaWQ9ImRlZnMzIj4KICAgIDxjbGlwUGF0aAogICAgICAgaWQ9ImEiPgogICAgICA8cGF0aAogICAgICAgICBmaWxsPSIjZmZmIgogICAgICAgICBkPSJNMCAwaDI1NnY4MEgweiIKICAgICAgICAgaWQ9InBhdGgzIiAvPgogICAgPC9jbGlwUGF0aD4KICA8L2RlZnM+Cjwvc3ZnPgo=)](https://brainmade.org)
[![GNU AGPLv3.0 License](https://img.shields.io/badge/License-GNU%20AGPLv3.0-dark_green?logo=gnu)](https://choosealicense.com/licenses/agpl-3.0)
[![Buymeacoffee](https://img.shields.io/badge/Buymeacoffee-gray?logo=buymeacoffee)](https://buymeacoffee.com/nicolabelluti)
<br>
[![CI Badge](https://git.nicolabelluti.me/little-emulator/little-emulator/actions/workflows/check-format-and-test.yaml/badge.svg)](https://git.nicolabelluti.me/little-emulator/little-emulator/actions/?workflow=check-format-and-test.yaml)
[![GitHub Stars](https://img.shields.io/github/stars/little-emulator)](https://github.com/little-emulator)

</div><br>

> An emulator for the LC2 ISA 🤖

## Library Usage

1. Add the library to you project:

   ```shell
   cargo add architectures --git https://git.nicolabelluti.me/little-emulator/little-emulator.git
   ```

2. Use the emulator in you project:

   ```rust
   use architectures::{lc2::Lc2, Architecture, WatcherType};
   
   fn main() {
       // Create a new LC2
       let mut cpu = Lc2::new(0x3000);
   
       // Setup the Machine Control Register
       cpu.set_memory(0xffff, 0x8000);
   
       // Add the memory watcher for the Video Data Register
       cpu.add_memory_watcher(0xf3ff, WatcherType::OnWrite, |x| {
           print!("{}", char::from_u32(x as u32).unwrap());
       });
   
       // Load the program
       cpu.load_bytes(0x3000, &[
           0xe2, 0x0a, 0x60, 0x40, 0x04, 0x06, 0xb0, 0x08, 0x12, 0x61, 0x40, 0x01,
           0x50, 0x20, 0xb0, 0x09, 0xf3, 0xff, 0xff, 0xff, 0x00, 0x48, 0x00, 0x65,
           0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57,
           0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x0a,
           0x00, 0x00,
       ]).unwrap();
   
       // Run the program
       while cpu.get_memory(0xffff) & 0x8000 != 0 {
           cpu.step_instruction();
       }
   }
   ```
