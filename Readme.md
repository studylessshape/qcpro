[TOC]

# Qcpro
使用RUST编写的，简单的在Termux或命令行中创建和初始化一个c++项目的程序

## Install
* [Rust环境](https://www.rust-lang.org/zh-CN/learn/get-started)
* 编译
* 设置环境变量
  * Rust环境
    安装并配置好Rust环境（一般安装好[Rustup](https://www.rust-lang.org/learn/get-started)，Rust环境也配置好了）
  * 编译
    克隆该项目源码，打开控制台到该目录下，输入`Cargo build`完成编译
  * 设置环境变量
    * Cmd
      设置临时变量，在cmd中输入，(qcpro)为克隆地址
      ```
      path %path%;(qcpro)\target\debug
      ```
      长期使用
      ```
      控制面板->系统->高级系统设置->环境变量->用户变量->添加->将debug目录地址复制并粘贴
      ```
    * Shell
      临时变量
      ```
      export PATH=$PATH:~/(qcpro)/target/debug
      ``` 
      设置永久变量请在.bashrc或.zshrc中添加上面的export指令

## How to use
配置好变量后输入qcpro加以下指令即可食用（括号表示该参数可以没有）
|Action|Subaction|Feature|
|---|---|---|
|`new`|`<diectory>`|新建一个项目（项目文件夹不存在）|
|`init`|(`<directory>`)|初始化一个项目（项目文件夹存在）|
|`--help`||打印帮助|
|`build`||使用CMake构建项目，相当于`cmake -S <source> -B <build>`（确保安装了CMake并且添加了环境变量）|
|`run`||Windows: 使用g++简单快速编译项目<br/>Shell: 先Build再使用make进行编译|
##  About Project
创建或初始化的项目拥有两个文件夹`include`和`src`，两个文件`src/main.cpp`和`CMakeList.txt`
- 文件内容
  - main.cpp
    ```c++
    #include<iostream>
    int main()
    {
        std::cout<<"Hello, world!"<<std::endl;
        return 0;
    }
    ```
  - CMakeList.txt
    ```cmake
    cmake_minimum_required(VERSION 3.10)
    project(<project/directory name>)
    add_executable(<project/directory name> src/main.cpp)
    ```
## PS
该项目仅为满足本人需求，日后准备实现**快捷编译**和**自定义初始化项目列表**