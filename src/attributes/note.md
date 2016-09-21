## 用途

* 条件编译(conditional compilation)
* 设置 crate name, version and type(binary or library)
* 关闭警告提示
* 提供一些编译器功能 如 宏导入等
* 链接外部library
* 将函数标注(mark)为单元测试
* 标注一些基准(benchmark)函数


when attributes apply to a whole crate:
`#![crate_attribute]`

when attributes apply to a module or item
`#[item_attribute]`


argument sytax
```
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
```


## dead_code

```
fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

fn main() {
    used_function();
}
```

## Crates

```
// lib.rs
// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

## cfg

```
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```


custom condition

```
// custom.rs
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}
```

Some conditionals like target_os are implicitly provided by rustc, but custom conditionals must be passed to rustc using the --cfg flag.

```
rustc --cfg some_condition custom.rs && ./custom
```
