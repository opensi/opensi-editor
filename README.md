# opensi-editor 

![Rust](https://github.com/opensi/opensi/workflows/Rust/badge.svg)

Редактор вопросов для популярной реализации "Своей игры": [SiGame](https://vladimirkhil.com/si/game)

Доступен в веб формате: [OpenSI Editor Web](https://opensi.github.io/opensi-editor).

## Пакеты с игрой

OpenSI Editor совместим с некоторыми пакетами формата `*.siq` из SIGame. Ведется работа над полной совместимостью.

|Версия|Совместимость|
|------|-------------|
|[Version 4](https://github.com/VladimirKhil/SI/wiki/SIQ-file-format-(version-4))|✔️ *Совместимо*|
|[Version 5](https://github.com/VladimirKhil/SI/wiki/SIQ-file-format-version-5)|⚙️ *В работе*|

## Сборка и запуск

```shell
# Обычный запуск нативной версии
$ cargo run opensi-editor

# Запуск веб-версии (потребуется установка trunk)
$ trunk serve --config crates/opensi-editor/Cargo.toml --release false
```
