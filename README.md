# qute!

> *designify your terminal with a large ansi colors palette*

[Examples](#examples) &mdash; [API](#api) &mdash; [Goals](#goals) &mdash; [License](#license)

a nice ANSI escape codes `API` to use ansi color in a elegant way that support *8 color, 16 color, 88 color, 256 color and 16 millions color mode.* You can manage the color mode by yourself, you got the control dude!

## <a name="examples"></a> Examples

```rust
extern crate qute;


use qute::*;
use qute::design::Designer;


fn main () {
  println!("{}", qute!(" BLACK WHITE (BASIC) ").background_black().white());
}
```

## <a name="api"></a> API

the `qute!` macro contains all method to designify your terminal as you want. `Select Graphic Rendition` are parameters applied into the escape sequence format string. See the `SGR` table below   

### Modifier

**no params**

**returns**

`self` **{ Designer }**: the current instance

| color         | snippet                                  | preview                                                          |
|---------------|------------------------------------------|------------------------------------------------------------------|
| bold          | ```qute!("my string").bold()```          | <img src="doc/preview/hello-color-modifier-bold.svg"/>           |
| dim           | ```qute!("my string").dim()```           | <img src="doc/preview/hello-color-modifier-dim.svg"/>            |
| underlined    | ```qute!("my string").underline()```     | <img src="doc/preview/hello-color-modifier-underline.svg"/>      |
| blink         | ```qute!("my string").blink()```         | <img src="doc/preview/hello-color-modifier-blink.svg"/>          |
| reverse       | ```qute!("my string").reverse()```       | <img src="doc/preview/hello-color-modifier-reverse.svg"/>        |
| hidden        | ```qute!("my string").hidden()```        | <img src="doc/preview/hello-color-modifier-hidden.svg"/>         |
| strikethrough | ```qute!("my string").strikethrough()``` | <img src="doc/preview/hello-color-modifier-strikethrough.svg"/>  |

### Foreground

**no params**

**returns**

`self` **{ Designer }**: the current instance

| color         | snippet                                  | preview                                                            |
|---------------|------------------------------------------|--------------------------------------------------------------------|
| black         | ```qute!("my string").black()```         | <img src="doc/preview/hello-color-foreground-black.svg"/>          |
| red           | ```qute!("my string").red()```           | <img src="doc/preview/hello-color-foreground-red.svg"/>            |
| green         | ```qute!("my string").green()```         | <img src="doc/preview/hello-color-foreground-green.svg"/>          |
| yellow        | ```qute!("my string").yellow()```        | <img src="doc/preview/hello-color-foreground-yellow.svg"/>         |
| blue          | ```qute!("my string").blue()```          | <img src="doc/preview/hello-color-foreground-blue.svg"/>           |
| magenta       | ```qute!("my string").magenta()```       | <img src="doc/preview/hello-color-foreground-magenta.svg"/>        |
| cyan          | ```qute!("my string").cyan()```          | <img src="doc/preview/hello-color-foreground-cyan.svg"/>           |
| gray          | ```qute!("my string").gray()```          | <img src="doc/preview/hello-color-foreground-gray.svg"/>           |
| light_red     | ```qute!("my string").light_red()```     | <img src="doc/preview/hello-color-foreground-light-red.svg"/>      |
| light_green   | ```qute!("my string").light_green()```   | <img src="doc/preview/hello-color-foreground-light-green.svg"/>    |
| light_yellow  | ```qute!("my string").light_yellow()```  | <img src="doc/preview/hello-color-foreground-light-yellow.svg"/>   |
| light_blue    | ```qute!("my string").light_blue()```    | <img src="doc/preview/hello-color-foreground-light-blue.svg"/>     |
| light_magenta | ```qute!("my string").light_magenta()``` | <img src="doc/preview/hello-color-foreground-light-magenta.svg"/>  |
| light_cyan    | ```qute!("my string").light_cyan()```    | <img src="doc/preview/hello-color-foreground-light-cyan.svg"/>     |
| white         | ```qute!("my string").white()```         | <img src="doc/preview/hello-color-foreground-white.svg"/>          |

### Background

**no params**

**returns**

`self` **{ Designer }**: the current instance

| color         | snippet                                             | preview                                                           |
|---------------|-----------------------------------------------------|-------------------------------------------------------------------|
| black         | ```qute!("my string").background_black()```         | <img src="doc/preview/hello-color-background-black.svg"/>         |
| red           | ```qute!("my string").background_red()```           | <img src="doc/preview/hello-color-background-red.svg"/>           |
| green         | ```qute!("my string").background_green()```         | <img src="doc/preview/hello-color-background-green.svg"/>         |
| yellow        | ```qute!("my string").background_yellow()```        | <img src="doc/preview/hello-color-background-yellow.svg"/>        |
| blue          | ```qute!("my string").background_blue()```          | <img src="doc/preview/hello-color-background-blue.svg"/>          |
| magenta       | ```qute!("my string").background_magenta()```       | <img src="doc/preview/hello-color-background-magenta.svg"/>       |
| cyan          | ```qute!("my string").background_cyan()```          | <img src="doc/preview/hello-color-background-cyan.svg"/>          |
| gray          | ```qute!("my string").background_gray()```          | <img src="doc/preview/hello-color-background-gray.svg"/>          |
| light_red     | ```qute!("my string").background_light_red()```     | <img src="doc/preview/hello-color-background-light-red.svg"/>     |
| light_green   | ```qute!("my string").background_light_green()```   | <img src="doc/preview/hello-color-background-light-green.svg"/>   |
| light_yellow  | ```qute!("my string").background_light_yellow()```  | <img src="doc/preview/hello-color-background-light-yellow.svg"/>  |
| light_blue    | ```qute!("my string").background_light_blue()```    | <img src="doc/preview/hello-color-background-light-blue.svg"/>    |
| light_magenta | ```qute!("my string").background_light_magenta()``` | <img src="doc/preview/hello-color-background-light-magenta.svg"/> |
| light_cyan    | ```qute!("my string").background_light_cyan()```    | <img src="doc/preview/hello-color-background-light-cyan.svg"/>    |
| white         | ```qute!("my string").background_white()```         | <img src="doc/preview/hello-color-background-white.svg"/>         |

### <a name="256-colors"></a> 256 Colors

**params**

`n` **{ u8 }**: the color number - *from 0 to 255*

**returns**

`self` **{ Designer }**: the current instance

| mode       | snippet                                          | preview                                                 |
|------------|--------------------------------------------------|---------------------------------------------------------|
| foreground | ```qute!("my string").set_rgb_color(231)```      | <img src="doc/preview/hello-color-256-color.svg"/>      |
| background | ```qute!("my string").set_rgb_background(220)``` | <img src="doc/preview/hello-color-256-background.svg"/> |

### <a name="rgb"></a> RGB

**params**

`r` **{ u8 }**: the standard red
`g` **{ u8 }**: the standard green
`b` **{ u8 }**: the standard blue

**returns**

`self` **{ Designer }**: the current instance

| mode       | snippet                                                    | preview                                                 |
|------------|------------------------------------------------------------|---------------------------------------------------------|
| foreground | ```qute!("my string").set_rgb_color(255, 255, 255)```      | <img src="doc/preview/hello-color-rgb-color.svg"/>      |
| background | ```qute!("my string").set_rgb_background(128, 128, 128)``` | <img src="doc/preview/hello-color-rgb-background.svg"/> |

### CSS

CSS colors keyword are supported. See the complete list [here](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#svg-color)

**params**

`keyword` **{ String }**: the css color keyword

**returns**

`self` **{ Designer }**: the current instance

| mode       | snippet                                                  | preview                                                 |
|------------|----------------------------------------------------------|---------------------------------------------------------|
| foreground | ```qute!("my string").set_css_color("lime")```           | <img src="doc/preview/hello-color-css-color.svg"/>      |
| background | ```qute!("my string").set_css_background("aquamarine")``` | <img src="doc/preview/hello-color-css-background.svg"/> |

## <a name="goals"></a> Goals

* [x] support modifiers
* [x] support vga
* [x] support rgb
* [x] support css colors keywords
* [ ] support hexa
* [ ] support hsl
* [ ] support format string
* [ ] detect color terminal support
* [ ] hsl to rgb
* [ ] unit testing
* [ ] convert to css
* [ ] cross platform
* [ ] documentation
* [ ] port js (?)

## <a name="license"></a> License   

Copyright ©️ 2020 Qurity    

Released under the [MIT](LICENSE) license    
