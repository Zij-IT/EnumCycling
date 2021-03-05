# Enum Cycling
Enum Cycling is just a single macro that aims to make working with enums easier in Rust.

# Including in a Project
Import enum_cycling into your project by adding this line to your
Cargo.toml.

 ```toml                                                            
 [dependencies]                                                     
 enum_cycling = "0.1.0"
 enum_cycling_derive = "0.1.0"
 
 # You can also just use the "derive" feature
 # enum_cycling = { version = "0.1.0", features = ["derive"]}                                             
```                                                                 

# Enum Cycling Macros
Enum Cycling has implemented these macros:

| Macro | Description |
------- | ----------- |
| EnumCycle| Adds two methods `.up()` and `.down()` to go from one enum variant to the one either above or below it.
