# 功能介绍

指定文件和待查找的字符串，它们需要用户从命令行给予输入，然后我们在程序内进行读取

## 接收命令行参数

首先来思考下，如果要传入文件路径和待搜索的字符串，那这个命令该长啥样，我觉得大概率是这样:

```rust
cargo run -- searchstring example-filename.txt
```

`--` 告诉 cargo 后面的参数是给我们的程序使用的，而不是给 cargo 自己使用，例如 -- 前的 run 就是给它用的。

在程序中读取传入的参数，这个很简单，下面代码就可以:

```rust
// in main.rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

首先通过 `use` 引入标准库中的 env 包，然后 `env::args` 方法会读取并分析传入的命令行参数，最终通过 `collect` 方法输出一个集合类型 `Vector`。`collect` 方法其实并不是`std::env`包提供的，而是迭代器自带的方法(`env::args()` 会返回一个迭代器)，它会将迭代器消费后转换成我们想要的集合类型，关于迭代器和 collect 的具体介绍。

最后，代码中使用 dbg! 宏来输出读取到的数组内容，来看看长啥样：

```sh

$ cargo run -- searchstring example-filename.txt

[src/main.rs:5:5] args = [
    "target/debug/minigrep",
    "searchstring",
    "example-filename.txt",
]
```

> 所有的用户输入都不可信！不可信！不可信！

## 存储读取到的参数

在编程中，给予清晰合理的变量名是一项基本功，咱总不能到处都是 args[1] 、args[2] 这样的糟糕代码吧。

因此我们需要两个变量来存储文件路径和待搜索的字符串:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
```

在项目根目录创建 poem.txt 文件

```text
I'm nobody! Who are you?
我啥也不是，你呢？
Are you nobody, too?
牛逼如你也是无名之辈吗？
Then there's a pair of us - don't tell!
那我们就是天生一对，嘘！别说话！
They'd banish us, you know.
你知道，我们不属于这里。
How dreary to be somebody!
因为这里属于没劲的大人物！
How public, like a frog
他们就像青蛙一样呱噪，
To tell your name the livelong day
成天将自己的大名
To an admiring bog!
传遍整个无聊的沼泽！
```

接下来修改 `main.rs` 来读取文件内容：

```rust
use std::env;
use std::fs;

fn main() {
    // --省略之前的内容--
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
```

# 增加模块化和错误处理

但凡稍微没那么糟糕的程序，都应该具有代码模块化和错误处理，不然连玩具都谈不上。
梳理我们的代码和目标后，可以整理出大致四个改进点:

- 单一且庞大的函数。对于 minigrep 程序而言， `main` 函数当前执行两个任务：解析命令行参数和读取文件。但随着代码的增加，`main` 函数承载的功能也将快速增加。从软件工程角度来看，一个函数具有的功能越多，越是难以阅读和维护。因此最好的办法是将大的函数拆分成更小的功能单元。

- 配置变量散乱在各处。还有一点要考虑的是，当前 `main` 函数中的变量都是独立存在的，这些变量很可能被整个程序所访问，在这个背景下，独立的变量越多，越是难以维护，因此我们还可以将这些用于配置的变量整合到一个结构体中。

- 细化错误提示。 目前的实现中，我们使用 `expect` 方法来输出文件读取失败时的错误信息，这个没问题，但是无论任何情况下，都只输出 `Should have been able to read the file` 这条错误提示信息，显然是有问题的，毕竟文件不存在、无权限等等都是可能的错误，一条大一统的消息无法给予用户更多的提示。

- 使用错误而不是异常。 假如用户不给任何命令行参数，那我们的程序显然会无情崩溃，原因很简单：`index out of bounds`，一个数组访问越界的 `panic`，但问题来了，用户能看懂吗？甚至于未来接收的维护者能看懂吗？因此需要增加合适的错误处理代码，来给予使用者给详细友善的提示。还有就是需要在一个统一的位置来处理所有错误，利人利己！

## 分离 main 函数

关于如何处理庞大的 `main` 函数，Rust 社区给出了统一的指导方案:

- 将程序分割为 `main.rs` 和 `lib.rs`，并将程序的逻辑代码移动到后者内
- 命令行解析属于非常基础的功能，严格来说不算是逻辑代码的一部分，因此还可以放在 `main.rs` 中

按照这个方案，将我们的代码重新梳理后，可以得出 main 函数应该包含的功能:

- 解析命令行参数
- 初始化其它配置
- 调用 `lib.rs` 中的 `run` 函数，以启动逻辑代码的运行
- 如果 `run` 返回一个错误，需要对该错误进行处理

这个方案有一个很优雅的名字: 关注点分离(Separation of Concerns)。简而言之，`main.rs` 负责启动程序，`lib.rs` 负责逻辑代码的运行。从测试的角度而言，这种分离也非常合理： `lib.rs` 中的主体逻辑代码可以得到简单且充分的测试，至于 `main.rs` ？确实没办法针对其编写额外的测试代码，但是它的代码也很少啊，很容易就能保证它的正确性。

### 分离命令行解析

根据之前的分析，我们需要将命令行解析的代码分离到一个单独的函数，然后将该函数放置在 `main.rs` 中：

```rust
// in main.rs
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --省略--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
```

### 聚合配置变量

配置变量并不适合分散的到处都是，因此使用一个结构体来统一存放是非常好的选择，这样修改后，后续的使用以及未来的代码维护都将更加简单明了。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // --snip--
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

值得注意的是，Config 中存储的并不是 &str 这样的引用类型，而是一个 String 字符串，也就是 Config 并没有去借用外部的字符串，而是拥有内部字符串的所有权。`clone` 方法的使用也可以佐证这一点。大家可以尝试不用 `clone` 方法。

clone 的得与失:

在上面的代码中，除了使用 clone ，还有其它办法来达成同样的目的，但 clone 无疑是最简单的方法：直接完整的复制目标数据，无需被所有权、借用等问题所困扰，但是它也有其缺点，那就是有一定的性能损耗。

因此是否使用 clone 更多是一种性能上的权衡，对于上面的使用而言，由于是配置的初始化，因此整个程序只需要执行一次，性能损耗几乎是可以忽略不计的。

总之，判断是否使用 clone:

是否严肃的项目，玩具项目直接用 clone 就行，简单不好吗？

要看所在的代码路径是否是热点路径(hot path)，例如执行次数较多的显然就是热点路径，热点路径就值得去使用性能更好的实现方式

下面我们试着来优化下，通过构造函数来初始化一个 Config 实例，而不是直接通过函数返回实例，典型的，标准库中的 String::new 函数就是一个范例

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
```

## 错误处理

回顾一下，如果用户不输入任何命令行参数，我们的程序会怎么样？

```sh
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:27:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

结果喜闻乐见，由于 args 数组没有任何元素，因此通过索引访问时，会直接报出数组访问越界的 `panic`。

报错信息对于开发者会很明确，但是对于使用者而言，就相当难理解了，下面一起来解决它。

### 改进报错信息

还记得在错误处理章节，我们提到过 `panic` 的两种用法: `被动触发`和`主动调用`嘛？上面代码的出现方式很明显是被动触发，这种报错信息是不可控的，下面我们先改成主动调用的方式:

```rust
// in main.rs
 // --snip--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        // --snip--
```

目的很明确，一旦传入的参数数组长度小于 3，则报错并让程序崩溃推出，这样后续的数组访问就不会再越界了。

```sh
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at 'not enough arguments', src/main.rs:26:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

不错，用户看到了更为明确的提示，但是还是有一大堆 debug 输出，这些我们其实是不想让用户看到的。这么看来，想要输出对用户友好的信息, panic 是不太适合的，它更适合告知开发者，哪里出现了问题。

### 返回 Result 来替代直接 panic

那只能祭出之前学过的错误处理大法了，也就是返回一个 Result：成功时包含 Config 实例，失败时包含一条错误信息。

有一点需要额外注意下，从代码惯例的角度出发，`new` 往往不会失败，毕竟新建一个实例没道理失败，对不？因此修改为 `build` 会更加合适。

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

### 处理返回的 Result

接下来就是在调用 build 函数时，对返回的 Result 进行处理了，目的就是给出准确且友好的报错提示, 为了让大家更好的回顾我们修改过的内容，这里给出整体代码:

```rust
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

上面代码有几点值得注意:

- 当 Result 包含错误时，我们不再调用 `panic` 让程序崩溃，而是通过 `process::exit(1)` 来终结进程，其中 `1` 是一个信号值(事实上非 0 值都可以)，通知调用我们程序的进程，程序是因为错误而退出的。

- `unwrap_or_else` 是定义在 `Result<T,E>` 上的常用方法，如果 Result 是 Ok，那该方法就类似 unwrap：返回 Ok 内部的值；如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理

综上可知，`config` 变量的值是一个 `Config` 实例，而 `unwrap_or_else` 闭包中的 `err`参数，它的类型是 `'static str`，值是 `"not enough arguments"` 那个字符串字面量。

终于，我们得到了自己想要的输出：既告知了用户为何报错，又消除了多余的 debug 信息，非常棒。可能有用户疑惑，`cargo run` 底下还有一大堆 `debug` 信息呢，实际上，这是 `cargo run` 自带的，大家可以试试编译成二进制可执行文件后再调用，会是什么效果。

## 分离主体逻辑

接下来可以继续精简 main 函数，那就是将主体逻辑( 例如业务逻辑 )从 main 中分离出去，这样 main 函数就保留主流程调用，非常简洁。

```rust
// in main.rs
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// --snip--
```

如上所示，main 函数仅保留主流程各个环节的调用，一眼看过去非常简洁清晰。

### 使用 ? 和特征对象来返回错误

`run` 函数没有错误处理，因为在文章开头我们提到过，错误处理最好统一在一个地方完成，这样极其有利于后续的代码维护。

```rust
//in main.rs
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
```

值得注意的是这里的 `Result<(), Box<dyn Error>>` 返回类型，首先我们的程序无需返回任何值，但是为了满足 `Result<T,E>` 的要求，因此使用了 `Ok(())` 返回一个单元类型 `()`。

最重要的是 `Box<dyn Error>`， 如果按照顺序学到这里，大家应该知道这是一个 `Error` 的特征对象(为了使用 Error，我们通过 `use std::error::Error;` 进行了引入)，它表示函数返回一个类型，该类型实现了 Error 特征，这样我们就无需指定具体的错误类型，否则你还需要查看 fs::read_to_string 返回的错误类型，然后复制到我们的 run 函数返回中，这么做一个是麻烦，最主要的是，一旦这么做，意味着我们无法在上层调用时统一处理错误，但是`Box<dyn Error>` 不同，其它函数也可以返回这个特征对象，然后调用者就可以使用统一的方式来处理不同函数返回的 `Box<dyn Error>`。

### 处理返回的错误

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

没错 `if let` 的使用让代码变得更简洁，可读性也更加好，原因是，我们并不关注 `run` 返回的 Ok 值，因此只需要用 `if let` 去匹配是否存在错误即可。

好了，截止目前，代码看起来越来越美好了，距离我们的目标也只差一个：将主体逻辑代码分离到一个独立的文件 `lib.rs` 中。

## 分离逻辑代码到库包中

首先，创建一个 src/lib.rs 文件，然后将所有的非 main 函数都移动到其中。

```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}
```

为了内容的简洁性，这里忽略了具体的实现，下一步就是在 main.rs 中引入 lib.rs 中定义的 Config 类型。

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        // --snip--
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

很明显，这里的 `mingrep::run` 的调用，以及 `Config` 的引入，跟使用其它第三方包已经没有任何区别，也意味着我们成功的将逻辑代码放置到一个独立的库包中，其它包只要引入和调用就行。
