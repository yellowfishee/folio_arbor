# 项目介绍

## src-tauri

Tauri 后端代码，负责数据库操作、API 接口等。

## 目录结构

```text
folio_arbor/
├── src/                     # 前端代码 (Vue/React等)
├── public/                  # 静态资源
├── src-tauri/               # Rust后端代码
│   ├── src/
│   │   ├── main.rs          # 应用入口
│   │   ├── lib.rs           # 核心逻辑与命令注册
│   │   ├── commands/        # Tauri命令 (前端接口)
│   │   │   ├── mod.rs
│   │   │   ├── note.rs
│   │   │   └── user.rs
│   │   ├── services/        # 业务逻辑层
│   │   │   ├── mod.rs
│   │   │   ├── note_service.rs
│   │   │   └── user_service.rs
│   │   ├── db/              # 数据库相关
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs # 数据库连接
│   │   │   ├── migrations/  # 数据库迁移
│   │   │   └── models/      # 数据库模型
│   │   │       ├── mod.rs
│   │   │       ├── note.rs
│   │   │       └── user.rs
│   │   ├── models/          # 业务模型/VO
│   │   │   ├── mod.rs
│   │   │   ├── note_dto.rs
│   │   │   └── user_vo.rs
│   │   └── utils/           # 工具函数
│   │       ├── mod.rs
│   │       └── helpers.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
└── README.md                # 项目文档
```
