# QBT-Controller

A Windows system tray application, written in rust, to make quickly pausing and
resuming qBittorrent instances easy!

## Rationale

TODO

## Installation

Install using `cargo` in the usual way.

```sh
$ cargo install --locked --git https://github.com/jarulsamy/QBT-Controller
```
> Note: This is currently extremely Windows specific. At most, Linux support will be added in the future.

## Configuration

The default configuration file is created on program startup at
`C:\Users\YOUR_USERNAME\AppData\Roaming\qbt-controller\settings.ini`

Any number of hosts can be added in the following format:
```ini
[FriendlyHostName]
url=http://IP_TO_HOST:PORT
username=USERNAME
password=PASSWORD


[AnotherHost]
. . .
```

> Currently, username and password fields are ignored. Authentication support is
> yet to be added.

## Road Map

- [X] Read qBittorrent instance details from a config file.
- [X] Load a basic system tray application.
- [X] Context menu
- [X] Notifications for querying all hosts.
- [ ] Basic authentication
- [X] Option to pause/resume all hosts.
- [ ] Option to pause/resume invidiual hosts.

## License

Refer to [LICENCE](./LICENSE)
