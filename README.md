# Ivory

Ivory is an in-progress programming language for making character sheets in tabletop games. It is functional, stateless, and has first-class support for handling dice rolls.

The main goal of Ivory is to be a language that lets you write the mechanics of your tabletop character as quickly and expressively as writing their backstory.

Code is easy to read, and because everything is functional, functions are simple to understand with no possible side effects.

```
str = 15 + 1;
dex = 15;
con = 10;
int = 11 + 1;
wis = 10;
cha = 19 + 2;

ability_mod = ability -> ( ability - 10 ) /_ 2;

str_mod = ability_mod(str);
dex_mod = ability_mod(dex);
con_mod = ability_mod(con);
int_mod = ability_mod(int);
wis_mod = ability_mod(wis);
cha_mod = ability_mod(cha);

ability_check = ability -> 1d20 + ability_mod(ability);

str_check = ability_check(str);
dex_check = ability_check(dex);
con_check = ability_check(con);
int_check = ability_check(int);
wis_check = ability_check(wis);
cha_check = ability_check(cha);
```

Currently, there's a simple command line tool for Ivory that lets you query a sheet's values.

```
$ ivory ./my_character_sheet.ivory

ivory ~ 11 + 3
11 + 3 = 14

ivory ~ 1d20 + 4
<1d20: 20> + 4 = 24

ivory ~ str_mod + 5
3 + 5 = 8
```


## Stateless and Functionial

Ivory is *stateless*, meaning values cannot change, and *functional*, meaning functions act like functions in math: they take inputs, give back an output, and don't do anything else. An example of a functional language is [Haskell](https://www.haskell.org/).

This choice is limiting by design- Ivory is stateless because physical character sheets are also stateless. When you change your character on a physical sheet, you do it by writing on the sheet. Similarly, when you change your character in Ivory, you do it by editing the file. With no need to store floating, possibly mutated info about your character, you have a guarantee that what's in your character file is the start and end of what data your character is made of.

Characters are living things, and change over time as they level up and progress. Stateless Ivory makes it easy to track these changes through source control like git. Every time you level up, or gain an item, or finish a session, you can commit the changes your character has undergone.


# Todo List

There's still _lots_ to be done for Ivory, plenty of features are missing, or ones I want to add.

- [ ] Comments
- [ ] Typechecking struct creation
- [ ] Typechecking function inputs
- [ ] Casting
- [ ] Ternary operator (Conditionals)
- [ ] Rust style `match` statements
- [ ] Iterating over arrays etc.
- [ ] Javascript interoperation
- [ ] Module loading, both from URLs Deno-style, and locally. Loading remote modules will always be safe because of statelessness.
- [ ] Automatically reload changes to source file(s) without having to reload the CLI
- [ ] Ability to run edits from the CLI
	- Being able to run `hp = 23` in the CLI, and finding the `hp = ...` line in the source file, changing it to `hp = 23;`
- [ ] GUI character sheets (some day!)