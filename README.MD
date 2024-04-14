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
