
## TODO
- auto suggest action - pickup, bash, -> exec with space
- Nemesis system, where bosses remember player, advance to boss
- Monsters have names! "Gunroc the Orc attacks you"
- list of things which are visible, a
  - <0> pickup purple potion <space> <- neareast item
  - <1> attack sleeping Orc

- Mobs have states: Sleeping, Hesitating, Afraid, Waiting...

## Stats
- Strength
  - how much can carry
- Fitness
- Intelligence

- hp = Str + Con

- character levels up, has str: 12, con: 10 => gains 10-22 hp

dex hit chance
str dmg
- Bow dex+2, str-1
- Axe     str, dex
- Sword   str, dex


## Weapons

```rust
Club { size: Size.M }
```

## Installation



### libcotd
wget http://roguecentral.org/doryen/?file_id=5

```
hg clone https://bitbucket.org/jice/libtcod
```

Then make libtcod with the following
```
cd libtcod
hg checkout 1.5.x
wget https://gist.githubusercontent.com/jaredonline/daf3c5f1ea6c7ca00e29/raw/ae91b3e47bf0de5b772eff882e477d8144cfbaf8/makefile-osx -O makefiles/makefile-osx
wget https://dl.dropboxusercontent.com/u/169446/osx.tar.gz
tar -xzvf osx.tar.gz
make -f makefiles/makefile-osx
make -f makefiles/makefile-samples-linux
```
