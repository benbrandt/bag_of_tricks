# bag_of_tricks

A random D&amp;D character generator, written in Rust.

Currently supports:

| Book                    |          Races          | Classes |       Backgrounds       |
| ----------------------- | :---------------------: | :-----: | :---------------------: |
| Player's Handbook (PHB) | :ballot_box_with_check: |         | :ballot_box_with_check: |

I've been focusing on the necessary logic, so the "ui" is very bare-bones right now. To generate your first character:

- Make sure you have [rust installed](https://www.rust-lang.org/learn/get-started)
- Clone this repo
- Run `cargo run` from the root of the project

Go to `http://localhost:8000`, and you should see something like this:

```text
CHARACTER NAME: Tordek Frostbeard
RACE: Hill Dwarf (PHB p18,20)
BACKGROUND: Hermit (PHB p134)
ALIGNMENT: Chaotic Evil
LEVEL: 1

STR  +2 (15)
DEX  -2 (6)
CON  +2 (14)
INT  +2 (14)
WIS  +1 (13)
CHA  -1 (9)

SKILLS:
PROF  MOD  SKILL            BONUS:
      DEX  Acrobatics       -2
      WIS  Animal Handling  +1
      INT  Arcana           +2
      STR  Athletics        +2
      CHA  Deception        -1
      INT  History          +2
      WIS  Insight          +1
      CHA  Intimidation     -1
      INT  Investigation    +2
 X    WIS  Medicine         +3
      INT  Nature           +2
      WIS  Perception       +1
      CHA  Performance      -1
      CHA  Persuasion       -1
 X    INT  Religion         +4
      DEX  Sleight of Hand  -2
      DEX  Stealth          -2
      WIS  Survival         +1

SPEED: 25
PROFICIENCY BONUS: +2
RESISTANCES: Poison
LANGUAGES: Common, Dwarvish, Giant
PROFICIENCIES: Weapon(Specific(Battleaxe)), Weapon(Specific(Handaxe)), Weapon(Specific(LightHammer)), Weapon(Specific(Warhammer)), Tool(HerbalismKit), Tool(ArtisansTools(SmithsTools))

CHARACTERISTICS:
Age: 216
Gender: Male
Size: Medium
Height: 3'11"
Weight: 133 lb.

PERSONALITY TRAITS:
I am working on a grand philosophical theory and love sharing my ideas.
The leader of my community had something wise to say on every topic, and I am eager to share that wisdom.
IDEAL: Free Thinking. Inquiry and curiosity are the pillars of progress.  (Chaotic)
BOND: I entered seclusion to hide from the ones who might still be hunting me. I must someday confront them.
FLAW: Now that I've returned to the world, I enjoy its delights a little too much.

EQUIPMENT
A scroll case stuffed full of notes from your studies or prayers, a winter blanket, a set of common clothes, an herbalism kit, and 5 gp

FEATURES AND TRAITS:
- Darkvision (PHB p20)
- Dwarven Resilience (PHB p20)
- Stonecunning (PHB p20)
- Dwarven Toughness (PHB p20)
- Discovery (PHB p134)

BACKSTORY:
Life of Seclusion: I retreated from society after a life-altering event.
```

(Line count `git ls-files | grep -E '.*\.rs$' | xargs wc -l`)
