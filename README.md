# opensi 

![Rust](https://github.com/snpefk/opensi/workflows/Rust/badge.svg)

Open SI Game - реализация популярной телеигры на Rust

## Пакеты с игрой

opensi совместимыми пакетим формата `*.siq` из популярной реализации "Своей игры" - [SiGame](https://vladimirkhil.com/si/game)

## Разработка

Редактор зависит от GTK, поэтому перед разработкой стоит [установить](http://gtk-rs.org/docs/requirements.html) **GTK+**, **GLib** и **Cairo**

### Debian & Ubuntu

```shell
sudo apt-get install libgtk-3-dev
```

### Fedora

```shell
$ sudo dnf install gtk3-devel glib2-devel

### Fedora 21 and earlier
$ sudo yum install gtk3-devel glib2-devel

```

## Сборка и запуск

Для клиента игры и для редактора существуют отдельные bin-конфигурации:

```shell
# Клиент
$ cargo run --bin client

# Редактор
$ cargo run --bin editor
```
