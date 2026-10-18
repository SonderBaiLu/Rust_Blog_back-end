# Rust 基础补全练习册

> 定位：`docs/学习计划.md` 用**你自己的博客**当素材；本练习册用 **16 个与博客无关的独立小项目**，
> 把 Rust 的基础语法与机制**刻进手**。两者互补：计划解决「把真实项目做完」，本册解决「基础不留洞」。
>
> 生成日期：2026-10-15。配套：卡住 → `docs/卡壳急救手册.md`；不懂概念 → `docs/学习计划.md` 第五章分支。

---

## 怎么用这本练习册

1. **一节一个项目，做完就丢**。练习项目不是要维护的代码，目的是让你在**没有业务干扰**的情况下
   把机制练成肌肉记忆。
2. **不许跳节**。每一节末尾有「出口关卡」= **到什么程度才能进下一节**。不达标就留在本节，
   重复做或用「记忆重构法」重做一遍。
3. **代码放哪**：
   - 单文件练习 → `src/bin/<名字>.rs`，用 `cargo run --bin <名字>` 跑；
   - 需要多文件的章节（第 8、15 节）→ `exercises/<名字>/` 独立 crate（`cargo new --lib` 或 `--bin`）。
   - 练习要用的额外依赖（如 `reqwest`）加到 `Cargo.toml`，不污染主 crate 就放 `[dev-dependencies]`。
4. **每节记一条教学复述**进 `docs/学习内容.md`（格式：教一个会 Python 的人懂这一节的核心）。
5. **节奏**：每节 3–8 小时，一次做不完可拆两天。全部做完约 60–80 小时，正好填满计划 Q1 的空档。
6. **通用出口规矩**（每节都要过，下面各节只写具体内容）：
   - **① 跑通**：项目能运行，满足本节「验收清单」里的每一条；
   - **② 讲清**：合上资料，能口述本节列的 3 个问题；
   - **③ 闭卷**：完成本节末尾的闭卷小练习，不查资料。
   三项全达成 → 进下一节。记一行到 `docs/踩坑记录.md`。

---

## 第 0 节 工具链与热身

**知识点**：`rustup` / `cargo` / `rustc` 分工；`cargo new / build / check / run / fmt / clippy`；
`src/bin/` 多二进制；读环境变量与命令行参数。

**项目题目**：`src/bin/s00_env.rs` —— 打印一份「环境报告」：Rust 版本（提示：`env!("CARGO_PKG_VERSION")`）、
当前工作目录、`PATH` 里第一个路径、命令行传入的所有参数。

**动手清单**
- [ ] 用 `cargo new` 建个临时练习 crate 熟悉命令，再删掉
- [ ] 写 `src/bin/s00_env.rs`，能 `cargo run --bin s00_env a b c` 并打印 `a b c`
- [ ] 跑一次 `cargo fmt` 和 `cargo clippy`，读懂 clippy 给的每条建议
- [ ] 故意写一个类型错误，跑 `cargo check`，读完整报错，再跑 `cargo build` 对比输出差异

**常见坑**：`cargo check` 不产出二进制（只检查）、`cargo build` 才编译出可执行文件；
`env!` 是编译期宏，`std::env::var` 是运行期读。

**出口关卡（三项全达成才能进第 1 节）**
- ① 报告能打印出全部四项内容，`--bin` 名指定正确。
- ② 口述：`cargo check` / `build` / `run` 三者区别？`src/bin/` 下每个文件会被当成什么？
- ③ 闭卷：写出「读取名为 `HOME` 的环境变量，读不到时打印默认值」的 3 行代码。

---

## 第 1 节 变量、类型、函数、控制流

**知识点**：`let` / `mut` / 遮蔽（shadowing）；标量类型；数组与元组；`fn` 与表达式返回值；
`if` / `loop` / `while` / `for`；`match` 入门；`println!` / `format!` 格式化。

**项目题目**：**猜数字游戏** `src/bin/s01_guess.rs` —— 程序随机生成 1~100 的整数，
循环读取用户输入并提示「太大 / 太小」，记录并最终打印猜中用的次数；输入非法（不是数字）时提示重试而不是崩溃。

**动手清单**
- [ ] 用 `rand` 生成 1..=100 的随机数（`use rand::Rng`）
- [ ] 用 `std::io::stdin().read_line` 读一行，`trim().parse::<u32>()` 解析
- [ ] `match` 处理 `parse` 的 `Ok` / `Err`：`Ok` 比较大小，`Err` 提示「请输入数字」
- [ ] 用 `loop` + `break` 控制循环，用计数器记录次数
- [ ] 全部函数化：至少抽出 `fn guess_once() -> Result<u32, _>` 之类的函数

**常见坑**：`read_line` 要传 `&mut String`；`parse` 返回 `Result`；`let` 默认不可变（忘了 `mut` 编译不过）；
整数之间比较时类型要统一（`u32` vs `i32`）。

**出口关卡**
- ① 游戏能玩通；非法输入不崩溃；结束时打印次数。
- ② 口述：遮蔽（shadowing）和 `mut` 的区别？`for` 和 `while` 各适合什么场景？为什么 `read_line` 要 `&mut`？
- ③ 闭卷：写 `fn max_of(v: &[i32]) -> Option<i32>`，返回最大值。

---

## 第 2 节 所有权与借用

**知识点**：移动（move）vs 复制（Copy）；借用 `&T` / `&mut T`；借用规则（同一作用域不能同时可变+不可变借用）；
切片；函数传参时的所有权转移。

**项目题目**：**字符串统计器** `src/bin/s02_text.rs` —— 输入一段文本，输出：
单词数、字符数、最长单词的长度、每个元音字母出现次数。**硬性要求：函数参数一律用 `&str` / `&mut`，
只有最终构造结果时才创建 owned 值，禁止用 `clone()` 图省事。**

**动手清单**
- [ ] 写 `fn count_words(text: &str) -> usize`
- [ ] 写 `fn count_vowels(text: &str) -> [usize; 5]`（返回数组，5 个元音）
- [ ] 写 `fn longest_len(text: &str) -> usize`
- [ ] 用切片 `&text[a..b]` 手动切出单词，体会「不拥有数据只引用」
- [ ] 检查代码里**没有一个多余的 clone**，每个都是「不用 clone 就编译不过」才留

**常见坑**：`String` 和 `&str` 混用；`s.len()` 是字节数不是字符数（中文会错，用 `chars().count()`）；
在持有不可变借用时想改数据 → E0499/E0502。

**出口关卡**
- ① 结果正确（含中文文本不 panic）；通读一遍确认没有多余 `clone`。
- ② 口述：移动后原变量为什么失效，而 `i32` 复制不失效？为什么不能同时有 `&T` 和 `&mut T`？`&s[..]` 是什么？
- ③ 闭卷：写 `fn double_evens(v: &mut Vec<i32>)`（把偶数翻倍）和 `fn sum(v: &[i32]) -> i32`（只读求和）。

---

## 第 3 节 结构体、枚举、模式匹配

**知识点**：`struct` 定义与方法（`impl`）；带数据的 `enum`；`match` 穷尽匹配；
`if let` / `while let`；`#[derive(Debug)]` 与 `{:?}`。

**项目题目**：**命令行待办清单（内存版）** `src/bin/s03_todo.rs` —— 支持 `add <内容>` / `list` /
`done <序号>` / `remove <序号>` / `quit`。把命令解析成枚举：
`enum Command { Add(String), List, Done(usize), Remove(usize), Quit }`，用 `match` 分发。

**动手清单**
- [ ] `struct Item { text: String, done: bool }`，`impl Item` 加个 `fn done()` 方法
- [ ] `enum Command { ... }` + `fn parse(input: &str) -> Option<Command>`
- [ ] 用 `match cmd { ... }` 分发，`list` 用 `for (i, item) in items.iter().enumerate()`
- [ ] `Done`/`Remove` 的序号越界时打印提示而非 panic
- [ ] 命令非法时打印用法帮助

**常见坑**：`match` 分支必须穷尽（用 `_ =>` 兜底）；`parse` 里对 `done 3` 要 `split_whitespace` 再解析；
`Option` 的解包别乱用 `unwrap`。

**出口关卡**
- ① 增删改查全跑通，非法命令/越界有序号提示。
- ② 口述：`enum` 相比「一堆 bool 字段」好在哪？`match` 为什么要求穷尽？`if let` 和 `match` 是什么关系？
- ③ 闭卷：定义一个 `enum Shape { Circle(f64), Rect(f64, f64) }` 并用 `match` 求面积。

---

## 第 4 节 集合 Vec / String / HashMap

**知识点**：`Vec` 增删查；`String` 的 `push_str` / `format!` / `split` / `trim`；
`HashMap` 的 `entry().or_insert()`；`iter` 入门；排序。

**项目题目**：**词频统计器** `src/bin/s04_wordcount.rs` —— 读一个文本文件，统计每个单词出现次数，
按出现次数从高到低输出前 N 个（N 从命令行参数取，默认 10）。

**动手清单**
- [ ] 用 `std::fs::read_to_string` 读文件，`to_lowercase()` 统一大小写
- [ ] 用 `split_whitespace()` 切词，`HashMap<String, usize>` 计数
- [ ] 用 `entry(word).or_insert(0) += 1` 计数（**必须用 entry，不许 get 后再 insert**）
- [ ] 把 map 转成 `Vec<(String, usize)>` 并 `sort_by` 按次数降序
- [ ] 用 `iter().take(n)` 输出前 N 个

**常见坑**：`String` 不能用 `s[0]` 下标（UTF-8 变长）；`HashMap` 遍历顺序随机；借用 map 时不能同时改。

**出口关卡**
- ① 输出前 N 个且排序正确；同频词顺序稳定或可接受。
- ② 口述：`entry` 为什么比 `get + insert` 好？`String` 为什么不能下标？`Vec` 扩容大致怎么发生？
- ③ 闭卷：写 `fn counts(items: &[&str]) -> HashMap<&str, usize>`。

---

## 第 5 节 错误处理 Option / Result / ?

**知识点**：`panic!` vs `Result`；`Option` 常用方法（`map` / `unwrap_or`）；
`Result` + `?`；`From` 转换为何让 `?` 能自动转错误类型；`main` 返回 `Result`。

**项目题目**：**迷你 CSV 解析器** `src/bin/s05_csv.rs` —— 读一个 CSV 文件，解析成
`Vec<Record>`（字段如 name/age/city）。要求：**任何格式问题都返回自定义错误而不是 panic**——
空行、列数不齐、年龄不是数字，都要能指出**出错的行号**。

**动手清单**
- [ ] 用 `thiserror` 定义 `enum CsvError { Io(..), BadColumns { line: usize }, BadField { line: usize } }`
- [ ] 逐行 `split(',')`，列数不对返回 `BadColumns`
- [ ] 年龄字段 `parse::<u32>()`，失败返回 `BadField`
- [ ] 用 `?` 传播错误，**全程不出现 `unwrap()`**（除非你能说出为什么这里安全）
- [ ] `main() -> Result<(), Box<dyn std::error::Error>>`，错误时打印出来

**常见坑**：`?` 要求错误类型能 `From` 转换；`expect` / `unwrap` 是恐慌不是返回；忘了给错误带上行号。

**出口关卡**
- ① 非法 CSV 返回错误且含行号；合法文件正常解析。
- ② 口述：`?` 替你做了哪两件事？什么时候能用 `unwrap`？`From` 在 `?` 里起什么作用？
- ③ 闭卷：写一个返回 `Result<T, 自定义错误>` 的函数，用它调用一个返回 `io::Error` 的函数并用 `?` 转换。

---

## 第 6 节 迭代器与闭包

**知识点**：闭包与捕获（`Fn` / `FnMut` / `FnOnce`）；迭代器适配器链
（`map` / `filter` / `fold` / `collect` / `zip` / `enumerate`）；迭代器是惰性的；
`iter` vs `into_iter` vs `iter_mut`。

**项目题目**：**日志分析器** `src/bin/s06_log.rs` —— 读一段日志（每行如 `200 12ms /api/posts`），
用**迭代器链**统计：各状态码出现次数、平均响应时间、最慢的 3 条。**硬性要求：不许用 `for` 循环遍历行。**

**动手清单**
- [ ] `lines().filter_map(|l| parse_line(l)).collect::<Vec<_>>()`
- [ ] 统计状态码：`.filter(|r| r.code == 200).count()` 或分组计数
- [ ] 平均响应时间：`.map(|r| r.ms).sum::<u64>() as f64 / n as f64`
- [ ] 最慢 3 条：`sort_by` + `take(3)`，或 `fold` 维护 top3
- [ ] 写出闭包捕获了哪些变量、按值还是按引用

**常见坑**：迭代器惰性——忘了 `collect` / `for_each` 什么都不发生；`into_iter` 会吃掉集合；
闭包借用和 `filter` 里改外部变量冲突。

**出口关卡**
- ① 全程迭代器链且结果正确。
- ② 口述：迭代器为什么惰性？闭包三种捕获有什么关系？`iter` / `into_iter` / `iter_mut` 区别？
- ③ 闭卷：用一条迭代器链把字符串里所有数字求和（`"a1b22c3"` → 26）。

---

## 第 7 节 泛型与 trait

**知识点**：泛型函数与泛型结构体；`trait` 定义 + `impl`；默认方法；`where` 边界；
泛型 vs `dyn`（动态分发的入口）；`#[derive]` 与手写 impl 的关系。

**项目题目**：**可插拔存储** `src/bin/s07_store.rs` —— 定义
`trait Storage { fn save(&mut self, k: &str, v: &str); fn get(&self, k: &str) -> Option<&str>; }`，
实现 `MemoryStorage`（用 `HashMap`）和 `FileStorage`（每对 k=v 存一行到文件）两个后端；
再写一个泛型函数 `fn dump<S: Storage>(s: &S)` 同时接受两者。用**同一套调用代码**驱动两个后端。

**动手清单**
- [ ] 定义 trait + 两个实现
- [ ] `dump` 用泛型 `<S: Storage>`，或 `where S: Storage`
- [ ] 额外写一个 `fn dump_dyn(s: &dyn Storage)` 版本，对比两者
- [ ] 给 `FileStorage` 实现 `Drop` 或 `save` 时 flush，保证写入可见

**常见坑**：trait 方法签名要对齐（`&self` vs `&mut self`）；泛型单态化会产生多份代码；
`&dyn Trait` 需要对象安全（不能有泛型方法）。

**出口关卡**
- ① 两个后端都能用，泛型函数不改就能吃两者。
- ② 口述：泛型和 trait 的边界各是什么？什么时候该把大 trait 拆成小 trait？`dyn` 和泛型分发位置差在哪？
- ③ 闭卷：写 `trait Shape { fn area(&self) -> f64; }` + 两个实现 + 泛型函数求总面积。

---

## 第 8 节 模块系统与 crate 组织

**知识点**：`mod` / `pub` / `use` / `super` / `crate`；文件与目录拆分；`lib.rs` vs `main.rs`；
`pub use` 重导出；可见性层级。

**项目题目**：把第 7 节的存储项目升级成**多模块 crate**，放进 `exercises/kvstore/`：
目录结构 `src/lib.rs`（导出 `Storage` trait）、`src/storage/mod.rs`、`src/storage/memory.rs`、
`src/storage/file.rs`、`src/main.rs`（用库）。

**动手清单**
- [ ] `cargo new --lib exercises/kvstore`，把代码按模块拆开
- [ ] `lib.rs` 里 `pub mod storage;` + `pub use storage::Storage;`
- [ ] 子模块用 `super::` / `crate::` 正确引用
- [ ] `main.rs` 用 `use kvstore::Storage;` 调用
- [ ] 每个 `pub` 都问一遍「真的需要公开吗」，能收窄就收窄

**常见坑**：`mod` 和文件/目录的对应关系搞错；`pub` 漏了导致外部不可见；`use` 路径写错 → E0432。

**出口关卡**
- ① 在 `exercises/kvstore/` 里 `cargo run` 和 `cargo test` 都跑通。
- ② 口述：`pub` / `pub(crate)` / 私有各什么范围？`lib.rs` 和 `main.rs` 的区别？什么时候该用子目录？
- ③ 闭卷：画出该 crate 的模块树，标出每个 `use` 的来源。

---

## 第 9 节 测试

**知识点**：`#[test]`；`assert!` / `assert_eq!`；`#[should_panic]`；
单元测试放模块内 `#[cfg(test)] mod tests`；集成测试放 `tests/`；测私有 vs 公有函数。

**项目题目**：给**第 5 节的 CSV 解析器**补测试 `src/bin/s05_csv.rs` 内（或迁到库 crate）——
覆盖 5 类用例：正常文件、空文件、列数不齐、字段类型错、含引号/逗号的行。

**动手清单**
- [ ] 写 `#[cfg(test)] mod tests { use super::*; ... }`
- [ ] 正常用例：断言解析出的记录数和字段值
- [ ] 错误用例：断言返回的是正确的 `CsvError` 变体（含行号）
- [ ] 边界用例：空文件返回空 Vec 而非报错
- [ ] 跑 `cargo test`，看输出里的通过/失败计数

**常见坑**：测试模块要 `use super::*` 才能访问被测函数；`assert_eq!` 要求类型实现 `PartialEq` + `Debug`；
断言错误变体要 `matches!`。

**出口关卡**
- ① `cargo test` 全绿，5 类用例都有。
- ② 口述：单元测试和集成测试放哪？为什么能测私有函数？一个测试的 Arrange/Act/Assert 对应哪几行？
- ③ 闭卷：给任意一个你写过的函数，写出 3 个测试（正常 / 边界 / 错误）。

---

## 第 10 节 智能指针 Box / Rc / RefCell / Arc

**知识点**：`Box<T>`（堆分配、解递归类型）；`Rc<T>`（共享所有权）与 `RefCell<T>`（内部可变）；
`Rc<RefCell<T>>` 组合；`Deref` / `Drop`；`Drop` 何时执行。

**项目题目**：**表达式求值器** `src/bin/s10_eval.rs` —— 把 `(1 + 2) * (3 - 4)` 解析成 AST
`enum Expr { Num(i64), Add(Box<Expr>, Box<Expr>), Sub(Box<Expr>, Box<Expr>), Mul(...) }` 并递归求值。
再做一个 `Rc<RefCell<...>>` 小实验：多个变量共享同一个可变计数器。

**动手清单**
- [ ] 定义递归 enum `Expr`（**必须用 `Box`，否则提示无限大小**）
- [ ] 写 `fn eval(e: &Expr) -> i64` 递归求值
- [ ] 手搭一个 AST 实例并算出结果（括号优先级由结构决定，不用处理运算符优先级）
- [ ] `Rc::new(RefCell::new(0))`，`Rc::clone` 出两份，`borrow_mut()` 改一个看另一个变
- [ ] 在 `Drop` 里打印一句话，观察值何时被释放

**常见坑**：递归类型不加 `Box` → E0072（无限大小）；`RefCell` 的 `borrow_mut` 冲突会**运行期** panic；
`Rc` 不是线程安全的（→ 第 12 节用 `Arc`）。

**出口关卡**
- ① 求值结果正确；`Rc<RefCell>` 实验观察成功。
- ② 口述：`Box` 为什么能解递归类型？`Rc` 和 `Arc` 差在哪？`RefCell` 的借用检查为什么在运行期？
- ③ 闭卷：用 `Box` 写出单链表节点 `enum List { Node(i32, Box<List>), Nil }`。

---

## 第 11 节 生命周期标注

**知识点**：显式 `'a`；函数返回值借用的约束；结构体持有引用；三条省略规则；`'static`。

**项目题目**：**借用重构练习** `src/bin/s11_lifetime.rs` —— 在第 2 节的基础上加两个**真正需要显式生命周期**
的东西：`fn longest_word<'a>(text: &'a str) -> &'a str`；
和 `struct Token<'a> { text: &'a str, len: usize }`（从文本里切 token 但不复制）。

**动手清单**
- [ ] 写 `longest_word<'a>(text: &'a str) -> &'a str` 并编译通过
- [ ] 写 `struct Token<'a>`，实现一个从 `&'a str` 构造的方法
- [ ] 挑项目里 3 个被「省略」的函数，手写显式 `'a` 版本验证编译
- [ ] 故意写一个省略规则覆盖不了的场景，记录报错原话再修对

**常见坑**：返回借用必须绑定到某个入参；多个入参时省略规则会失效；结构体带引用必须标 `'a`。

**出口关卡**
- ① 显式标注编译通过。
- ② 口述：三条省略规则各是什么？为什么返回值借用要绑到入参？`'static` 含义？
- ③ 闭卷：给 3 个已有函数手标 `'a`。

---

## 第 12 节 并发基础

**知识点**：`thread::spawn` + `join`；`move` 闭包；`Arc<Mutex<T>>`；`mpsc` channel；
为什么数据竞争在编译期就被挡住。

**项目题目**：**多线程词频统计** `src/bin/s12_parallel_wordcount.rs` —— 把一个大文件切成 N 段，
N 个线程各自统计局部 `HashMap`，用 `mpsc` channel 把结果发回主线程合并；**再写一个 `Arc<Mutex<HashMap>>` 版本**对比。

**动手清单**
- [ ] `std::thread::spawn(move || {...})` 起 N 个线程，`join` 收集
- [ ] channel 版本：`mpsc::channel()`，每个线程 `send` 局部结果，主线程 `recv` 合并
- [ ] 锁版本：`Arc::new(Mutex::new(HashMap::new()))`，`Arc::clone` 进线程，lock 后累加
- [ ] 两个版本结果与单线程对照，必须一致

**常见坑**：闭包必须 `move` 才能拿走数据进线程；`Arc::clone` 克隆的是指针不是数据；
忘记 `join` 主线程会提前结束；`Mutex` 的锁忘了 `drop` 会死锁。

**出口关卡**
- ① 两个版本结果都与单线程一致。
- ② 口述：`move` 闭包为什么必要？`Arc` 和 `Mutex` 各解决什么问题？channel 和共享+锁哪个更贴 Rust 风格？
- ③ 闭卷：写 4 个线程各对共享计数器加 1000 次（`Arc<Mutex<i32>>`），打印结果。

---

## 第 13 节 异步基础

**知识点**：`async fn` 与 `.await`；`Future` 是什么；tokio runtime（`#[tokio::main]`）；
`tokio::spawn`；`tokio::time::sleep`；`join!` / `select!`；`spawn_blocking`；`reqwest`。

**项目题目**：**并发 HTTP 抓取器** `src/bin/s13_fetch.rs` —— 给定 N 个 URL（可用 `https://httpbin.org/delay/1`），
并发抓取收集状态码，**对比串行版本 vs 并发版本的耗时**；再故意放一个 `std::thread::sleep` 感受卡顿，
改用 `spawn_blocking` 修复。

**动手清单**
- [ ] 加 `reqwest` 和 `tokio`（`full` 特性）依赖
- [ ] 串行版本：`for url in urls { reqwest::get(url).await?.status() }`，`Instant` 计时
- [ ] 并发版本：`futures::future::join_all` 或 `tokio::join!`，对比耗时
- [ ] 把其中一个任务改成 `std::thread::sleep`，观察它拖慢别的任务
- [ ] 用 `tokio::task::spawn_blocking` 包住阻塞调用，再对比

**常见坑**：忘了 `#[tokio::main]`；async 里调 `std::thread::sleep` 阻塞整个线程；`.await` 写在非 async 函数里。

**出口关卡**
- ① 并发版本明显快于串行；阻塞实验的现象说得出；`spawn_blocking` 修复有效。
- ② 口述：`.await` 挂起时任务去哪了？为什么 async 里不能调 `std::thread::sleep`？`spawn_blocking` 何时用？
- ③ 闭卷：用 tokio 起两个并发 `sleep` 任务并计时，证明它们并行。

---

## 第 14 节 serde 与文件 IO

**知识点**：`#[derive(Serialize, Deserialize)]`；文件读写（`read_to_string` / `write`）；
`serde_json`；配置结构体；`#[serde(default)]` / `rename`；错误处理。

**项目题目**：**JSON 配置管理器** `src/bin/s14_config.rs` —— 读取 `config.json` 成结构体
（含 `port`、`db_url`、`log_level` 等，缺失字段用 `#[serde(default)]` 兜底），
命令行支持 `get <key>` / `set <key> <value>` 并**写回文件**；文件不存在或 JSON 非法要给出清晰错误。

**动手清单**
- [ ] `#[derive(Serialize, Deserialize)] struct Config { ... }`
- [ ] 用 `serde_json::from_str` 读，`to_string_pretty` 写回
- [ ] `#[serde(default = "默认函数")]` 给缺失字段默认值
- [ ] `set` 修改后写回，再 `get` 验证往返一致
- [ ] 文件不存在 / JSON 非法分别返回不同错误

**常见坑**：写回时忘了整体覆盖导致内容错乱；`#[serde(rename)]` 和字段名大小写对不上；
`to_string` 里中文/特殊字符转义。

**出口关卡**
- ① 读写往返一致；容错生效（缺失字段有默认值，非法 JSON 报错不 panic）。
- ② 口述：`derive` 大致展开成了什么？`#[serde(default)]` 作用？为什么用 `to_writer` 而非 `to_string` 后再写？
- ③ 闭卷：定义一个含 3 字段的 struct，从 JSON 字符串反序列化并访问字段。

---

## 第 15 节 综合毕业题

**知识点**：前 14 节全部。

**项目题目**：**完整的 CLI 工具**（放 `exercises/` 独立 crate）——
任选其一：**A. 书籍管理**（增删改查 + 分类标签 + 持久化到文件）；**B. 记账本**
（记录收支 + 按类别/月份汇总 + 持久化）。要求刻意用上：**多模块拆分、trait 抽象存储后端、
serde 持久化、自定义错误、单元 + 集成测试**。若有余力，加一个可选异步或并发特性（如异步批量导入）。

**动手清单**
- [ ] 先写设计文档 `exercises/<名字>/设计.md`（数据 / 命令 / 模块 / 错误 / 测试五问）
- [ ] 多模块：`entity` / `storage`（trait + 两个实现）/ `service` / `cli`
- [ ] serde 持久化，程序重启后数据还在
- [ ] 自定义 `Error` + `?` 传播，全程无 `unwrap`
- [ ] 单元测试（算法/校验）+ 集成测试（跑一遍完整命令流）
- [ ] `cargo fmt` + `cargo clippy` 零警告

**常见坑**：模块循环依赖；错误类型设计太糙导致到处 `map_err`；测试和实现耦合太紧。

**出口关卡（毕业三项）**
- ① 工具功能完整，`cargo test` 全绿，`cargo clippy` 无警告。
- ② 能向别人 **10 分钟讲清**：结构怎么分的、存储为什么抽 trait、错误怎么流动。
- ③ 用**记忆重构法**：删掉 `storage/` 或 `service/` 一个模块，闭卷重建，`diff` 记录差异。

---

## 做完之后

- 16 个项目全部通过出口关卡 = 你的 Rust **语法与机制层**没有明显空洞了。
- 接下来把精力全部回到 `docs/学习计划.md` 的主线（真实项目）和 `docs/深入研究方向清单.md`（纵深）。
- 依旧：卡住 → `docs/卡壳急救手册.md`；不懂概念 → 计划的 B1–B15 分支；每节一条复述进 `docs/学习内容.md`。
