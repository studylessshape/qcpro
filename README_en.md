- [Qcpro](#qcpro)
  - [Notice](#notice)
  - [Install](#install)
  - [How to use](#how-to-use)
  - [About Project](#about-project)
  - [PS](#ps)

[中文](Readme.md) | **English**

# Qcpro
Quick c++ project manager, the full name of qcpro, writen by [rust](https://www.rust-lang.org/), is a simple application arm to create and initialize a c++ project.

## Notice
This is a project used to practice my ability of rust. If need perfect tool, I recommand [Xmake](https://xmake.io/#/)

## Install
* [Rust environment](https://www.rust-lang.org/learn/get-started)
* Compile
* Set environmental variable
  * Rust environment
    Install and assemble Rustenvironment by [Rustup](https://www.rust-lang.org/learn/get-started)
  * Compile
    First, clone this repository
    ```
    git clone https://github.com/studylessshape/qcpro.git
    ```
    Then open the directory `qcpro`
    Last, open cmd or use shell to run command below
    ```
    cargo build --release
    ```
  * Set environment variable
    * Cmd(Windows)
      open cmd to run command below. (qcpro) is the directory that clone.
      ```
      path %path%;(qcpro)\target\release
      ```
      Use it forever
      ```
      Control panel->System->Advanced system settings->Environment variables->User->Add->Paste the path of release directory
      ```
    * Shell(Other OS)
      Temporary variable
      ```sh
      export PATH=$PATH:~/(qcpro)/target/release
      ``` 
      Please add the command above in file `.bashrc` or `.zshrc` if want to use it forever.
   * You can also transform the executable to other path.

## How to use
- Format like this
  ```
  qcpro [option] [action] (subaction)
  ```
- All arguments:

  |Action/Option|Subaction|Feature|
  |---|---|---|
  |`new`|`<diectory>`|Create a project(the same name of directory don't exist)|
  |`init`|(`<directory>`)|Initialize a project(the same name of directory exist)|
  |`build`||Use CMake to build project, eq.`cmake -S <source> -B <build>`(ensure that have installed [CMake](https://cmake.org/))|
  |`run`||Windows: Use `g++` simply compile project and run<br/>Shell: After CMake build,use `make` compile and run|
  |`-h`/`--help`||Print help|
  |`-v`/`--version`||Qcpro version|

##  About Project
There are two directories, `include` and `src`, and two file, `src/main.cpp` and `CMakeList.txt` by creating or initializing project.This is default project contents.
- Directory contents
  ```
   .
   ├ CMakeLists.txt
   ├ include
   └ src
     └ main.cpp
   ```
  
- Files contents
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
The customize config for creating and initializing project contents will come soon.
