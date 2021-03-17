- [Qcpro](#qcpro)
  - [Notice](#notice)
  - [Install](#install)
  - [How to use](#how-to-use)
  - [About Project](#about-project)
  - [PS](#ps)

**中文** | [English](README_en.md)

# Qcpro
使用RUST编写的，简单的在Termux或命令行中创建和初始化一个c++项目的程序。

## Notice
这是一个用来练手Rust的项目，如果需要好的项目/包管理工具，推荐[Xmake](https://xmake.io/#/)

## Install
* [Rust环境](https://www.rust-lang.org/zh-CN/learn/get-started)
* 编译
* 设置环境变量
* Windows安装g++
  * Rust环境
    安装并配置好Rust环境（一般安装好[Rustup](https://www.rust-lang.org/learn/get-started)，Rust环境也配置好了）
  * 编译
    克隆该项目源码，打开控制台到该目录下，输入`Cargo build --release`完成编译
  * 设置环境变量
    * Cmd
      设置临时变量，在cmd中输入，(qcpro)为克隆地址
      ```
      path %path%;(qcpro)\target\release
      ```
      长期使用
      ```
      控制面板->系统->高级系统设置->环境变量->用户变量->添加->release
      ```
    * Shell
      临时变量
      ```sh
      export PATH=$PATH:~/(qcpro)/target/debug
      ``` 
      设置永久变量请在.bashrc或.zshrc中添加上面的export指令
  * Windows安装g++
    点击此处下载[MinGW](https://sourceforge.net/projects/mingw-w64/)，解压并安装，然后将MinGW目录下的bin文件夹添加到环境变量
  
## How to use
- 格式
  ```
  qcpro [option] [action] (subaction)
  ```

  |Action/Option|Subaction|Feature|
  |---|---|---|
  |`new`|`<diectory>`|新建一个项目（项目文件夹不存在）|
  |`init`|(`<directory>`)|初始化一个项目（项目文件夹存在）|
  |`build`||使用CMake构建项目，相当于`cmake -S <source> -B <build>`（确保安装了CMake并且添加了环境变量）|
  |`run`||Windows: 使用g++简单快速编译项目<br/>Shell: 先Build再使用make进行编译|
  |`-h`/`--help`||打印帮助|
  |`-v`/`--version`||Qcpro版本|

##  About Project
创建或初始化的项目拥有两个文件夹`include`和`src`，两个文件`src/main.cpp`和`CMakeList.txt`
- 目录结构
  ```
   .
   ├ CMakeLists.txt
   ├ include
   └ src
     └ main.cpp
   ```
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
下阶段完成**自定义项目结构**