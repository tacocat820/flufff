# flufff
backs up things like git repos and updates them all at once

# usage
the available subcommands are: 
1. `flufff new [NAME] [TYPE] [OTHER ARGS]`
creates a new tracked backup
2. `flufff remove [PATH]`
makes the backup no longer tracked (requires an absolute path to it)
3. `flufff update`
updates all tracked backups

# configuration
it's located in ~/.config/flufff/conf.ini
this program automatically creates all config files when first ran

```
[types]
git=url:git clone [url] .:git pull
```

you can add other types of backups in the \[types\] category
syntax:
```
NAME=ARGS:INIT COMMAND:UPDATE COMMAND
```
the arguments are separated with a comma
you can inject things into a command by adding a variable in square brackets
the init command is only ran when using `flufff new` and the update command runs each update
