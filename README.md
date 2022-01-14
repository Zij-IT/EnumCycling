# Enum Cycling
Enum Cycling is just a single macro that aims to make working with enums easier in Rust.

# Including in a Project
Import enum_cycling into your project by adding this line to your
Cargo.toml.

 ```toml
 [dependencies]
 enum_cycling = "0.1.1"
 enum_cycling_derive = "0.2.0"

 # You can also just use the "derive" feature
 # enum_cycling = { version = "0.1.1", features = ["derive"]}
```
# Enum Cycling Macros
Enum Cycling has implemented these macros:

| Macro | Description |
------- | ----------- |
| EnumCycle| Adds two methods `.up()` and `.down()` to go from one enum variant to the one either above or below it.

# How to Use:
## EnumCycle

This derive was originally written while I was in the proccess of developing a game, and was working on the internal menus. I got really tired of having to write the matches by hand, and as such spent way longer learning how, and writing a macro to do the work for me.

```rust
use enum_cycling::EnumCycle;

#[derive(EnumCycle)]
pub enum Menu {
    Main,
    Settings,
    Quit,

    #[skip]
    Secret,
}

// KeyCode is not defined within the crate. It is used to demonstrate a use case for EnumCycle
pub fn current_menu(menu: Menu, player_input: KeyCode) -> Menu {
    match player_input {
        KeyCode::W => menu.up(),
        KeyCode::S => menu.down(),
        _ => menu,
    }
}
```

However, lets say that you don't want to have to worry about how your enum is sorted, or maybe you would like to keep it sorted alphabetically, and would still like to use EnumCycle. In this case, you may be a fan of using the 'cycle' attribute!

```rust
use enum_cycling::EnumCycle;

#[derive(EnumCycle)]
#[cycle(Main, Settings, Quit)]
pub enum Menu {
    Main,
    Quit,
    Secret,
    Settings,
}

pub fn current_menu(menu: Menu, player_input: KeyCode) -> Menu {
    match player_input {
        KeyCode::W => menu.up(),
        KeyCode::S => menu.down(),
        _ => menu,
    }
}
```

Both of these examples run exactly the same, and result in the same generated code!
