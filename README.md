# mt

The arbitrary meeting launcher, rewritten in rust from perl.

# Calling the program

```sh
mt (meeting name or alias)
```

If you want `mt` to automatically determine the meeting you should join,

```sh
mt
```

# Installation

## Compiling from source

You need to have rust on your system.

```sh
git clone https://github.com/junikimm717/mt
cd mt
sudo make clean install
```

## Binaries

Check out the [Releases](https://github.com/junikimm717/mt/releases).

# Configuration

## Default 

In order to create a default configuration file, run

```sh
mt --config
```

The config file resides in `~/.config/mt/config_v2.toml` to distinguish itself
from its perl version.

This configuration file will contain an example that demonstrates some of the
features of mt.

## Validation

In order to check the validate your configuration file, run
```sh
mt --check
```

This will check the following:

- Syntax for the configuration file is valid.
- No aliases that are used on two or more different meetings
- Meetings referenced in the schedule actually exist

## Changing settings

```toml
[ settings ]
browser = "Your browser binary/application here"
time = "(Minutes before you want to start a meeting)"
```

In MacOS, the browser will be launched as an application, whereas in UNIX, it
will be called as a binary.

`time` is used when mt automatically determines the URL of the meeting to
join. For example, if you want `mt` to be able to automatically join a meeting 5
minutes before it starts, set time to `5`.

## Meetings

Each meeting is part of the `meetings` table in the configuration file.

All meetings must have a default url and may have a set of weekday-specific URLs
and/or aliases.

As an example,

```toml
[meetings.mathclass]
url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
aliases = ['m', 'ma']
wednesday = "https://www.youtube.com/watch?v=iik25wqIuFo"
```

**Note: all names for weekdays must be in lower case**

If I run `mt m`, or `mt ma`, or `mt mathclass`, they will all open up the same
link because `m` and `ma` are defined to be aliases. (mt will also validate that
you do not have duplicated aliases.)

If any of the above commands are run on Wednesday (in local time), mt will open the
special url for `wednesday`. 

## The Schedule

To define weekly schedules, define key-value pairs under the table name of
`schedule.(day of week)`

Each key corresponds to a certain time string in a 12-hour calendar. Currently
accepted formats are `%I:%M %p` and `%I %p`.

The value can either be a meeting name or an alias that corresponds to a
meeting.

```toml
[schedule.monday]
# meeting names work
9AM = "mathclass"
# aliases work (as long as you don't have them duplicated)
11AM = "lt"

```
