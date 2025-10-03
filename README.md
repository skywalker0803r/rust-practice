# Rust 學習與實踐 (Rust Practice)

這是一個用來學習和實踐 Rust 程式語言的專案。其中包含了多個獨立的 `.rs` 檔案，每個檔案都演示了 Rust 的一個特定概念或是一個小練習。

## 專案結構

- `function.rs`: 演示如何在 Rust 中定義和使用函式。
- `datatype.rs`: 包含 Rust 基本數據類型的範例。
- `control_flow_demo.rs`: 演示 Rust 中的控制流程，例如 `if-else`、`loop`、`while` 和 `for`。
- `guessnumber.rs`: 一個經典的猜數字小遊戲，用來綜合練習變數、控制流程、輸入/輸出等基礎知識。

## 如何編譯與執行

你可以使用 `rustc` 編譯器來單獨編譯和執行每一個範例。

```bash
# 選擇一個你想執行的檔案，例如 guessnumber.rs
# 編譯
rustc guessnumber.rs

# 執行 (在 Windows 上)
.\guessnumber.exe

# 執行 (在 Linux 或 macOS 上)
./guessnumber
```