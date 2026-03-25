# Cargo Check Failure Caused by `http_proxy` / `https_proxy`

## Symptom

Running `cargo check` fails before compilation starts:

```text
warning: spurious network error (3 tries remaining): [56] Failure when receiving data from the peer (CONNECT tunnel failed, response 502)
error: failed to get `eframe` as a dependency of package `egui_counter`
```

## Root Cause

The shell environment was forcing Cargo traffic through local proxy settings:

```text
http_proxy=http://127.0.0.1:1087
https_proxy=http://127.0.0.1:1087
HTTP_PROXY=http://127.0.0.1:1087
HTTPS_PROXY=http://127.0.0.1:1087
ALL_PROXY=socks5h://127.0.0.1:1080
all_proxy=socks5h://127.0.0.1:1080
```

Those proxy endpoints were not working correctly, so requests to `https://index.crates.io/config.json` failed with a `502` tunnel error.

## Verified Fix

Run Cargo with the proxy variables removed for that command:

```bash
env -u http_proxy -u https_proxy -u HTTP_PROXY -u HTTPS_PROXY -u ALL_PROXY -u all_proxy cargo check
```

This was verified to complete successfully.

### Command Explanation

这个命令的作用是临时清除代理环境变量，然后运行 `cargo check`。

#### `env` 命令

`env` 用于在修改过的环境中运行程序。

#### `-u VAR` 选项

`-u` 的意思是 `unset`，即从环境中移除指定变量。这里移除了以下代理变量：

| Variable | 说明 |
| --- | --- |
| `http_proxy` | HTTP 代理（小写） |
| `https_proxy` | HTTPS 代理（小写） |
| `HTTP_PROXY` | HTTP 代理（大写） |
| `HTTPS_PROXY` | HTTPS 代理（大写） |
| `ALL_PROXY` | 全局代理（大写） |
| `all_proxy` | 全局代理（小写） |

大小写都清除是因为不同程序对环境变量大小写的读取行为不一致，全部清掉可以避免遗漏。

#### `cargo check`

`cargo check` 会在没有代理干扰的环境下执行 Rust 编译检查。

#### 为什么要这样做

常见场景是系统代理已经开启，但 Cargo 访问 `crates.io` 或内网 registry 时通过代理反而失败，例如连接不上、超时或证书异常。这个命令可以临时绕过代理，直接验证是否是代理导致的问题。

这个改动只影响当前这一条命令，不会修改 shell 的全局环境变量。

## Permanent Fix Options

1. Remove the broken proxy exports from your shell startup files such as `.bashrc`, `.profile`, or any proxy bootstrap script.
2. Correct the proxy settings so that `127.0.0.1:1087` and `127.0.0.1:1080` actually forward traffic.
3. If only Cargo should bypass the proxy, unset these variables only in the shell session where you build Rust projects.

Example for the current shell session:

```bash
unset http_proxy https_proxy HTTP_PROXY HTTPS_PROXY ALL_PROXY all_proxy
```

Then run:

```bash
cargo check
```

## Additional Compile Fix in This Project

After registry access was restored, the project had one real compile error caused by the `eframe 0.33` API.

The app creator closure in `src/main.rs` must return `Result<Box<dyn App>, _>`.

Working version:

```rust
eframe::run_native(
    "egui Counter App",
    options,
    Box::new(|_cc| Ok(Box::new(CounterApp::default()))),
)
```

## Final Working Command

```bash
env -u http_proxy -u https_proxy -u HTTP_PROXY -u HTTPS_PROXY -u ALL_PROXY -u all_proxy cargo check
```