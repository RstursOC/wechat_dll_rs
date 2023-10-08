# Rust 微信注入库
纯 Rust 语言编写，该项目仅能构建注入用的DLL，控制端未包含，需要自己实现，可以使用任意网络协议。

## 适用版本
微信 3.7.6.44

## 实现的功能
1. 检测登录状态
2. 自动同意好友请求
3. 修改备注
4. 发送文本消息
5. 发送图片消息
6. 根据 wxid 读取信息
7. 微信多开

## 构建教程

环境要求：Nightly Rust 1.69 以上

```shell
# 必须构建为32位dll
cargo build --target i686-pc-windows-msvc --release
```

## 免责声明
该项目仅用于学习交流使用，严禁用于商业用途。