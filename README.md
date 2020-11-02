<p align="center">
  <img width="500" alt="qute" src="https://github.com/qurity/qute/blob/master/doc/img/qute-logo-standard.png?raw=true" />
  <p align="center">
    <a href="https://github.com/qurity">
      <img alt="github" src="https://img.shields.io/badge/github.com/qurity-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">
    </a>
    <a href="https://crates.io/crates/qute">
      <img alt="crates.io" src="https://img.shields.io/crates/v/qute.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">
    </a>
    <a href="https://docs.rs/qute">
      <img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-qute-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">
    </a>
    <a href="https://crate-trends.herokuapp.com/qute">
<img alt="Crates.io" src="https://img.shields.io/crates/d/qute?style=for-the-badge&labelColor=555555" height="20">
    </a>
  </p>
</p>

---

> *designify your terminal with a large ansi colors palette*

[Examples](#examples) &mdash; [API](#api) &mdash; [Goals](#goals) &mdash; [License](#license)

a nice ANSI escape codes `API` to use ansi color in a elegant way that support *8 color, 16 color, 88 color, 256 color and 16 millions color mode.* You can manage the color mode by yourself, you got the control dude!

## <a name="examples"></a> Examples

**snippet**

```rust
extern crate qute;

use qute::prelude::*;

fn main () {
  let quted = qute!("my string").background_black().white();

  println!("{}", quted);
}
```


```rust
extern crate qute;

use qute::prelude::*;

fn main () {
  let quted = style!({
    value: "my string";
    color: "purple";
    background: "black";
    font-weight: none;
  });

  println!("{}", quted);
}

```
**cargo**

| example | command                           |
|---------|-----------------------------------|
| 256     | cargo run --example hello_colors  |
| basic   | cargo run --example basic         |
| css     | cargo run --example css           |
| rgb     | cargo run --example rgb           |

## Usage

Add the following to your Cargo.toml:

```toml
[dependencies]
qute = "0.0.12"
```

Now you can use the crate in your code

```rust
extern crate qute;
```

## <a name="api"></a> API

the `qute!` macro contains all method to designify your terminal as you want. `Select Graphic Rendition` are parameters applied into the escape sequence format string. See the `SGR` table below   

### Modifier

**no params**

**returns**

`self` **{ Designer }**: the current instance

| color         | snippet                                  | preview                                                            |
|---------------|------------------------------------------|--------------------------------------------------------------------|
| bold          | ```qute!("my string").bold()```          | <img src="./doc/preview/hello-color-modifier-bold.svg"/>           |
| dim           | ```qute!("my string").dim()```           | <img src="./doc/preview/hello-color-modifier-dim.svg"/>            |
| underlined    | ```qute!("my string").underline()```     | <img src="./doc/preview/hello-color-modifier-underline.svg"/>      |
| blink         | ```qute!("my string").blink()```         | <img src="./doc/preview/hello-color-modifier-blink.svg"/>          |
| reverse       | ```qute!("my string").reverse()```       | <img src="./doc/preview/hello-color-modifier-reverse.svg"/>        |
| hidden        | ```qute!("my string").hidden()```        | <img src="./doc/preview/hello-color-modifier-hidden.svg"/>         |
| strikethrough | ```qute!("my string").strikethrough()``` | <img src="./doc/preview/hello-color-modifier-strikethrough.svg"/>  |

### Foreground

**no params**

**returns**

`self` **{ Designer }**: the current instance

| color         | snippet                                  | preview                                                              |
|---------------|------------------------------------------|----------------------------------------------------------------------|
| black         | ```qute!("my string").black()```         | <img src="./doc/preview/hello-color-foreground-black.svg"/>          |
| red           | ```qute!("my string").red()```           | <img src="./doc/preview/hello-color-foreground-red.svg"/>            |
| green         | ```qute!("my string").green()```         | <img src="./doc/preview/hello-color-foreground-green.svg"/>          |
| yellow        | ```qute!("my string").yellow()```        | <img src="./doc/preview/hello-color-foreground-yellow.svg"/>         |
| blue          | ```qute!("my string").blue()```          | <img src="./doc/preview/hello-color-foreground-blue.svg"/>           |
| magenta       | ```qute!("my string").magenta()```       | <img src="./doc/preview/hello-color-foreground-magenta.svg"/>        |
| cyan          | ```qute!("my string").cyan()```          | <img src="./doc/preview/hello-color-foreground-cyan.svg"/>           |
| gray          | ```qute!("my string").gray()```          | <img src="./doc/preview/hello-color-foreground-gray.svg"/>           |
| light_red     | ```qute!("my string").light_red()```     | <img src="./doc/preview/hello-color-foreground-light-red.svg"/>      |
| light_green   | ```qute!("my string").light_green()```   | <img src="./doc/preview/hello-color-foreground-light-green.svg"/>    |
| light_yellow  | ```qute!("my string").light_yellow()```  | <img src="./doc/preview/hello-color-foreground-light-yellow.svg"/>   |
| light_blue    | ```qute!("my string").light_blue()```    | <img src="./doc/preview/hello-color-foreground-light-blue.svg"/>     |
| light_magenta | ```qute!("my string").light_magenta()``` | <img src="./doc/preview/hello-color-foreground-light-magenta.svg"/>  |
| light_cyan    | ```qute!("my string").light_cyan()```    | <img src="./doc/preview/hello-color-foreground-light-cyan.svg"/>     |
| white         | ```qute!("my string").white()```         | <img src="./doc/preview/hello-color-foreground-white.svg"/>          |

### Background

**no params**

**returns**

`self` **{ Designer }**: the current instance

| color         | snippet                                             | preview                                                             |
|---------------|-----------------------------------------------------|---------------------------------------------------------------------|
| black         | ```qute!("my string").background_black()```         | <img src="./doc/preview/hello-color-background-black.svg"/>         |
| red           | ```qute!("my string").background_red()```           | <img src="./doc/preview/hello-color-background-red.svg"/>           |
| green         | ```qute!("my string").background_green()```         | <img src="./doc/preview/hello-color-background-green.svg"/>         |
| yellow        | ```qute!("my string").background_yellow()```        | <img src="./doc/preview/hello-color-background-yellow.svg"/>        |
| blue          | ```qute!("my string").background_blue()```          | <img src="./doc/preview/hello-color-background-blue.svg"/>          |
| magenta       | ```qute!("my string").background_magenta()```       | <img src="./doc/preview/hello-color-background-magenta.svg"/>       |
| cyan          | ```qute!("my string").background_cyan()```          | <img src="./doc/preview/hello-color-background-cyan.svg"/>          |
| gray          | ```qute!("my string").background_gray()```          | <img src="./doc/preview/hello-color-background-gray.svg"/>          |
| light_red     | ```qute!("my string").background_light_red()```     | <img src="./doc/preview/hello-color-background-light-red.svg"/>     |
| light_green   | ```qute!("my string").background_light_green()```   | <img src="./doc/preview/hello-color-background-light-green.svg"/>   |
| light_yellow  | ```qute!("my string").background_light_yellow()```  | <img src="./doc/preview/hello-color-background-light-yellow.svg"/>  |
| light_blue    | ```qute!("my string").background_light_blue()```    | <img src="./doc/preview/hello-color-background-light-blue.svg"/>    |
| light_magenta | ```qute!("my string").background_light_magenta()``` | <img src="./doc/preview/hello-color-background-light-magenta.svg"/> |
| light_cyan    | ```qute!("my string").background_light_cyan()```    | <img src="./doc/preview/hello-color-background-light-cyan.svg"/>    |
| white         | ```qute!("my string").background_white()```         | <img src="./doc/preview/hello-color-background-white.svg"/>         |

### <a name="256-colors"></a> 256 Colors

**params**

`n` **{ u8 }**: the color number - *from 0 to 255*

**returns**

`self` **{ Designer }**: the current instance

| mode       | snippet                                      | preview                                                   |
|------------|----------------------------------------------|-----------------------------------------------------------|
| foreground | ```qute!("my string").set_color(231)```      | <img src="./doc/preview/hello-color-256-color.svg"/>      |
| background | ```qute!("my string").set_background(220)``` | <img src="./doc/preview/hello-color-256-background.svg"/> |

### <a name="rgb"></a> RGB

**params**

`r` **{ u8 }**: the standard red
`g` **{ u8 }**: the standard green
`b` **{ u8 }**: the standard blue

**returns**

`self` **{ Designer }**: the current instance

| mode       | snippet                                                    | preview                                                   |
|------------|------------------------------------------------------------|-----------------------------------------------------------|
| foreground | ```qute!("my string").set_rgb_color(255, 255, 255)```      | <img src="./doc/preview/hello-color-rgb-color.svg"/>      |
| background | ```qute!("my string").set_rgb_background(128, 128, 128)``` | <img src="./doc/preview/hello-color-rgb-background.svg"/> |

### CSS

CSS colors keyword are supported. See the complete list [here](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#svg-color)

**params**

`keyword` **{ String }**: the css color keyword

**returns**

`self` **{ Designer }**: the current instance

| mode       | snippet                                                    | preview                                                    |
|------------|------------------------------------------------------------|------------------------------------------------------------|
| foreground | ```qute!("my string").set_css_color("lime")```             | <img src="./doc/preview/hello-color-css-color.svg"/>       |
| background | ```qute!("my string").set_css_background("aquamarine")```  | <img src="./doc/preview/hello-color-css-background.svg"/>  |

## <a name="goals"></a> Goals

* [x] support modifiers
* [x] support vga
* [x] support rgb
* [x] support css colors keywords
* [ ] unit testing (wip)
* [ ] support hexa
* [ ] support hsl
* [ ] force color
* [ ] convert to css
* [ ] cross platform
* [x] documentation

## <a name="license"></a> License   

Copyright ©️ 2020 Qurity    

Released under the [MIT](LICENSE) license    
