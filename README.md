# You'reMyCDN
這是一個用來監測 Cloudflare 網站狀態的小工具，每當出現變動時則在 Telegram 發出通知，使用 Rust 開發。

## 為何叫做 You'reMyCDN
這是致敬顏社的夜貓組（Leo王+春豔 feat. 國蛋）所演唱的「[妳是我的Wifi](https://www.youtube.com/watch?v=96KZZhIFIAs)」

## 使用方法
```shell
git clone https://github.com/FutaGuard/YoureMyCDN.git
cargo build --release

cp config.example.yml config.yml
# 編輯 config.yml
```