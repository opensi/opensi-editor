<div align="center">

![banner](assets/banner.png)

# OpenSI Editor

[![CI](https://github.com/opensi/opensi-editor/actions/workflows/rust.yml/badge.svg)](https://github.com/opensi/opensi-editor/actions/workflows/rust.yml)
[![Github Pages](https://github.com/opensi/opensi-editor/actions/workflows/pages.yml/badge.svg)](https://github.com/opensi/opensi-editor/actions/workflows/pages.yml)
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue)](#лицензия)

</div>

<!-- TODO: добавить скриншот редактора (assets/screenshot.png) после обновления UI -->

## О проекте

Редактор пакетов с вопросами для «Своей игры», написанный на Rust. Позволяет создавать и редактировать пакеты формата `*.siq`, совместимые с популярной реализацией «Своей игры» [SIGame](https://vladimirkhil.com/si/game).

- **Кроссплатформенность**: работает нативно на Linux, Windows и macOS.
- **Веб-версия**: [запускается прямо в браузере](https://opensi.github.io/opensi-editor) без установки.
- **Формат SIGame**: импорт и редактирование пакетов `*.siq`, которыми и пользуются в основной игре.

## Совместимость

OpenSI Editor совместим с некоторыми пакетами формата `*.siq`.
Ведётся работа над полной совместимостью.

| Версия формата | Совместимость |
|----------------|---------------|
| [Version 4](https://github.com/VladimirKhil/SI/wiki/SIQ-file-format-(version-4)) | ✔️ *Совместимо* |
| [Version 5](https://github.com/VladimirKhil/SI/wiki/SIQ-file-format-version-5) | ⚙️ *В работе* |

## Сборка и запуск

### Требования

- [Rust](https://www.rust-lang.org/tools/install).
- Для веб-версии: [Trunk](https://trunkrs.dev/): `cargo install trunk`

### Нативная версия

```shell
cargo run
```

### Веб-версия

```shell
trunk serve --config crates/opensi-editor/Cargo.toml --release false
```

## Структура проекта

| Крейт | Назначение |
|-------|------------|
| [`opensi-core`](crates/opensi-core) | Модель данных и (де)сериализация пакетов `*.siq` |
| [`opensi-editor`](crates/opensi-editor) | Приложение-редактор (нативное и веб) на [egui](https://github.com/emilk/egui) |

## Лицензия

Проект распространяется под двойной лицензией -- на выбор:

- MIT ([LICENSE-MIT](LICENSE-MIT))
- Apache 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
