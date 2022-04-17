# cam

A nifty command alias manager

## Quick Start


### Install

```bash
# cargo
cargo install camer
# pnpm
pnpm add camer -g
```

### Usage

```bash
$ camer -h
```

```bash
camer 1.0.0

USAGE:
    camer <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add       Add a command alias
    clean     Clean up CAM application files
    help      Print this message or the help of the given subcommand(s)
    init      Initialisation of the configuration file
    ls        Show all alias set
    remove    Remove existing alias
```

#### Init
The configuration file needs to be initialised before the `camer` can be used. This is done by executing `camer init`.

```bash
$ camer init
[CAM INFO]: cam initialization succeeded, please run `source /Users/kim/.zshrc`
```
When the above message is output on the command line, the initialisation is successful.

#### Ls
The `camer ls` command lists all alias set.

```bash
$ camer ls
```
```bash
     gs --------- git status
     ga --------- git add
     gd --------- git diff
     gb --------- git branch
     gco -------- git checkout
     gcl -------- git clone
     gm --------- git merge
     gpl -------- git pull
     gps -------- git push
     grh -------- git reset HEAD
     grm -------- git rm
     gst -------- git stash
     gsta ------- git stash apply
     gstd ------- git stash drop
     gstl ------- git stash list
     gstp ------- git stash pop
     gsh -------- git show
     gshh ------- git show HEAD
     gsl -------- git shortlog
     gt --------- git tag
     gmv -------- git mv
     ni --------- npm install
     ya --------- yarn add
     pa --------- pnpm add
     nr --------- npm run
     s ---------- npm run start
     t ---------- npm run test
     p ---------- npm run prod
     d ---------- npm run dev
     b ---------- npm run build
```
These are the default alias settings.

#### Add
The `camer add` command adds a new alias. The alias name and the command to be executed are required.

Let's add an alias for the `npm run deploy` command
```bash
$ camer add nd "npm run deploy"
[CAM INFO]: nd added successfully, please run `source /Users/kim/.zshrc`
```
The above result means that this was added successfully, so let's run the prompt command to make it work.

#### Remove
The `camer remove` command removes an existing alias. The alias name is required.

Let's remove the alias added in Add
```bash
$ camer remove nd
[CAM INFO]: remove nd alias succeeded, please run `source /Users/kim/.zshrc`
```
The above result means that this was removed successfully, so let's run the prompt command to make it work.

#### Clean
When you want to uninstall the camer application, please run `camer clean` before downloading if you need the configuration file to be removed along with it.

```bash
$ camer clean
[CAM Info]: Cam cleanup succeeded
```
The above result means that the cleanup was successful.

If you want to continue using it, please run `camer init` again to initialise it.
# License
MIT
