# Rust语言作业

## 作业内容及说明

### 冒泡排序算法

- 以i32的数据为例，先用双循环的方法去写，但是，看不出算法的逻辑。

- 递归的写法，可以简单的看明白逻辑，个人喜欢这个。一个额外的收获是，递归的写法，还可以控制排序到数组的哪一个位置。

### 交通信号灯

- 需要独立出一个trait来实现。

### 整数求和

- 需要处理边界，传入的参数为&[u32]。

### 图形面积计算

- 需要使用泛型和泛型约束来实现。

## 通用操作

### 创建新的Rust项目

- 使用`cargo new {PROJECT_NAME}` 可以用来创建Cargo的项目，会建立好目录，并创建**Cargo.toml**配置文件。本作业，创建了4个项目。

    ```Bash
        #task1: 冒泡算法
        cargo new sort

        #task2: 信号灯trait
        cargo new traffic

        #task3: u32整数求和
        cargo new sum

        #task4: 图形面积计算
        cargo new area
    ```

- 创建后，再对应的目录，使用`cargo run`就可以看到正常输出的**hello world**


### 资源

- Rust教程: [https://www.runoob.com/rust/rust-tutorial.html](https://www.runoob.com/rust/rust-tutorial.html)

- 冒泡算法的说明: [https://blog.csdn.net/guoweimelon/article/details/50902597](https://blog.csdn.net/guoweimelon/article/details/50902597)