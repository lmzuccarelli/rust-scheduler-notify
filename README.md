# Overview

A simple rust based scheduler with notify (pop-up)

It uses a simple yaml file to set a cron, send notification (gui pop-up), and execute commands (direct or via shell scripts)

# Usage

clone the repo

```
cd rust-scheduler-notify

# compile

make build
```

update the example (config/scheduler.yaml)

execute the scheduler

```
./target/release/rust-scheduler-notify --config-file config/scheduler.yaml --loglevel info
```

execute in the background

```
nohup ./execute_scheduler.sh > scheduler.log &
```
