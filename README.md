# opensi-editor 

![Rust](https://github.com/opensi/opensi/workflows/Rust/badge.svg)

Редактор вопросов для популярной реализации "Своей игры": [SiGame](https://vladimirkhil.com/si/game)

Доступен в веб формате: [OpenSI Editor Web](https://opensi.github.io/opensi-editor).

## Пакеты с игрой

OpenSI Editor совместим с пакетами формата `*.siq` из SIGame.

## Сборка и запуск

```shell
# Обычный запуск нативной версии
$ cargo run opensi-editor

# Запуск веб-версии (потребуется установка trunk)
$ trunk serve crates/opensi-editor/index.html
```
