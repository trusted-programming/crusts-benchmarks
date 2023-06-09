use libc;
extern "C" {
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgba_t {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct named_color {
    pub name: *const i8,
    pub val: u32,
}
static mut named_colors: [named_color; 149] = [
    {
        let mut init = named_color {
            name: b"transparent\0" as *const u8 as *const i8,
            val: 0xffffff00,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"aliceblue\0" as *const u8 as *const i8,
            val: 0xf0f8ffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"antiquewhite\0" as *const u8 as *const i8,
            val: 0xfaebd7ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"aqua\0" as *const u8 as *const i8,
            val: 0xffffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"aquamarine\0" as *const u8 as *const i8,
            val: 0x7fffd4ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"azure\0" as *const u8 as *const i8,
            val: 0xf0ffffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"beige\0" as *const u8 as *const i8,
            val: 0xf5f5dcff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"bisque\0" as *const u8 as *const i8,
            val: 0xffe4c4ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"black\0" as *const u8 as *const i8,
            val: 0xff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"blanchedalmond\0" as *const u8 as *const i8,
            val: 0xffebcdff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"blue\0" as *const u8 as *const i8,
            val: 0xffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"blueviolet\0" as *const u8 as *const i8,
            val: 0x8a2be2ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"brown\0" as *const u8 as *const i8,
            val: 0xa52a2aff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"burlywood\0" as *const u8 as *const i8,
            val: 0xdeb887ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cadetblue\0" as *const u8 as *const i8,
            val: 0x5f9ea0ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"chartreuse\0" as *const u8 as *const i8,
            val: 0x7fff00ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"chocolate\0" as *const u8 as *const i8,
            val: 0xd2691eff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"coral\0" as *const u8 as *const i8,
            val: 0xff7f50ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cornflowerblue\0" as *const u8 as *const i8,
            val: 0x6495edff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cornsilk\0" as *const u8 as *const i8,
            val: 0xfff8dcff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"crimson\0" as *const u8 as *const i8,
            val: 0xdc143cff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cyan\0" as *const u8 as *const i8,
            val: 0xffffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkblue\0" as *const u8 as *const i8,
            val: 0x8bff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkcyan\0" as *const u8 as *const i8,
            val: 0x8b8bff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgoldenrod\0" as *const u8 as *const i8,
            val: 0xb8860bff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgray\0" as *const u8 as *const i8,
            val: 0xa9a9a9ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgreen\0" as *const u8 as *const i8,
            val: 0x6400ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgrey\0" as *const u8 as *const i8,
            val: 0xa9a9a9ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkkhaki\0" as *const u8 as *const i8,
            val: 0xbdb76bff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkmagenta\0" as *const u8 as *const i8,
            val: 0x8b008bff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkolivegreen\0" as *const u8 as *const i8,
            val: 0x556b2fff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkorange\0" as *const u8 as *const i8,
            val: 0xff8c00ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkorchid\0" as *const u8 as *const i8,
            val: 0x9932ccff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkred\0" as *const u8 as *const i8,
            val: 0x8b0000ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darksalmon\0" as *const u8 as *const i8,
            val: 0xe9967aff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkseagreen\0" as *const u8 as *const i8,
            val: 0x8fbc8fff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkslateblue\0" as *const u8 as *const i8,
            val: 0x483d8bff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkslategray\0" as *const u8 as *const i8,
            val: 0x2f4f4fff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkslategrey\0" as *const u8 as *const i8,
            val: 0x2f4f4fff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkturquoise\0" as *const u8 as *const i8,
            val: 0xced1ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkviolet\0" as *const u8 as *const i8,
            val: 0x9400d3ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"deeppink\0" as *const u8 as *const i8,
            val: 0xff1493ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"deepskyblue\0" as *const u8 as *const i8,
            val: 0xbfffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"dimgray\0" as *const u8 as *const i8,
            val: 0x696969ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"dimgrey\0" as *const u8 as *const i8,
            val: 0x696969ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"dodgerblue\0" as *const u8 as *const i8,
            val: 0x1e90ffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"firebrick\0" as *const u8 as *const i8,
            val: 0xb22222ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"floralwhite\0" as *const u8 as *const i8,
            val: 0xfffaf0ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"forestgreen\0" as *const u8 as *const i8,
            val: 0x228b22ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"fuchsia\0" as *const u8 as *const i8,
            val: 0xff00ffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"gainsboro\0" as *const u8 as *const i8,
            val: 0xdcdcdcff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"ghostwhite\0" as *const u8 as *const i8,
            val: 0xf8f8ffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"gold\0" as *const u8 as *const i8,
            val: 0xffd700ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"goldenrod\0" as *const u8 as *const i8,
            val: 0xdaa520ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"gray\0" as *const u8 as *const i8,
            val: 0x808080ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"green\0" as *const u8 as *const i8,
            val: 0x8000ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"greenyellow\0" as *const u8 as *const i8,
            val: 0xadff2fff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"grey\0" as *const u8 as *const i8,
            val: 0x808080ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"honeydew\0" as *const u8 as *const i8,
            val: 0xf0fff0ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"hotpink\0" as *const u8 as *const i8,
            val: 0xff69b4ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"indianred\0" as *const u8 as *const i8,
            val: 0xcd5c5cff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"indigo\0" as *const u8 as *const i8,
            val: 0x4b0082ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"ivory\0" as *const u8 as *const i8,
            val: 0xfffff0ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"khaki\0" as *const u8 as *const i8,
            val: 0xf0e68cff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lavender\0" as *const u8 as *const i8,
            val: 0xe6e6faff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lavenderblush\0" as *const u8 as *const i8,
            val: 0xfff0f5ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lawngreen\0" as *const u8 as *const i8,
            val: 0x7cfc00ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lemonchiffon\0" as *const u8 as *const i8,
            val: 0xfffacdff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightblue\0" as *const u8 as *const i8,
            val: 0xadd8e6ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightcoral\0" as *const u8 as *const i8,
            val: 0xf08080ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightcyan\0" as *const u8 as *const i8,
            val: 0xe0ffffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgoldenrodyellow\0" as *const u8 as *const i8,
            val: 0xfafad2ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgray\0" as *const u8 as *const i8,
            val: 0xd3d3d3ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgreen\0" as *const u8 as *const i8,
            val: 0x90ee90ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgrey\0" as *const u8 as *const i8,
            val: 0xd3d3d3ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightpink\0" as *const u8 as *const i8,
            val: 0xffb6c1ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightsalmon\0" as *const u8 as *const i8,
            val: 0xffa07aff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightseagreen\0" as *const u8 as *const i8,
            val: 0x20b2aaff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightskyblue\0" as *const u8 as *const i8,
            val: 0x87cefaff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightslategray\0" as *const u8 as *const i8,
            val: 0x778899ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightslategrey\0" as *const u8 as *const i8,
            val: 0x778899ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightsteelblue\0" as *const u8 as *const i8,
            val: 0xb0c4deff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightyellow\0" as *const u8 as *const i8,
            val: 0xffffe0ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lime\0" as *const u8 as *const i8,
            val: 0xff00ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"limegreen\0" as *const u8 as *const i8,
            val: 0x32cd32ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"linen\0" as *const u8 as *const i8,
            val: 0xfaf0e6ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"magenta\0" as *const u8 as *const i8,
            val: 0xff00ffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"maroon\0" as *const u8 as *const i8,
            val: 0x800000ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumaquamarine\0" as *const u8 as *const i8,
            val: 0x66cdaaff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumblue\0" as *const u8 as *const i8,
            val: 0xcdff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumorchid\0" as *const u8 as *const i8,
            val: 0xba55d3ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumpurple\0" as *const u8 as *const i8,
            val: 0x9370dbff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumseagreen\0" as *const u8 as *const i8,
            val: 0x3cb371ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumslateblue\0" as *const u8 as *const i8,
            val: 0x7b68eeff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumspringgreen\0" as *const u8 as *const i8,
            val: 0xfa9aff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumturquoise\0" as *const u8 as *const i8,
            val: 0x48d1ccff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumvioletred\0" as *const u8 as *const i8,
            val: 0xc71585ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"midnightblue\0" as *const u8 as *const i8,
            val: 0x191970ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mintcream\0" as *const u8 as *const i8,
            val: 0xf5fffaff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mistyrose\0" as *const u8 as *const i8,
            val: 0xffe4e1ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"moccasin\0" as *const u8 as *const i8,
            val: 0xffe4b5ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"navajowhite\0" as *const u8 as *const i8,
            val: 0xffdeadff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"navy\0" as *const u8 as *const i8,
            val: 0x80ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"oldlace\0" as *const u8 as *const i8,
            val: 0xfdf5e6ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"olive\0" as *const u8 as *const i8,
            val: 0x808000ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"olivedrab\0" as *const u8 as *const i8,
            val: 0x6b8e23ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"orange\0" as *const u8 as *const i8,
            val: 0xffa500ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"orangered\0" as *const u8 as *const i8,
            val: 0xff4500ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"orchid\0" as *const u8 as *const i8,
            val: 0xda70d6ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"palegoldenrod\0" as *const u8 as *const i8,
            val: 0xeee8aaff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"palegreen\0" as *const u8 as *const i8,
            val: 0x98fb98ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"paleturquoise\0" as *const u8 as *const i8,
            val: 0xafeeeeff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"palevioletred\0" as *const u8 as *const i8,
            val: 0xdb7093ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"papayawhip\0" as *const u8 as *const i8,
            val: 0xffefd5ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"peachpuff\0" as *const u8 as *const i8,
            val: 0xffdab9ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"peru\0" as *const u8 as *const i8,
            val: 0xcd853fff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"pink\0" as *const u8 as *const i8,
            val: 0xffc0cbff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"plum\0" as *const u8 as *const i8,
            val: 0xdda0ddff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"powderblue\0" as *const u8 as *const i8,
            val: 0xb0e0e6ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"purple\0" as *const u8 as *const i8,
            val: 0x800080ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"red\0" as *const u8 as *const i8,
            val: 0xff0000ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"rosybrown\0" as *const u8 as *const i8,
            val: 0xbc8f8fff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"royalblue\0" as *const u8 as *const i8,
            val: 0x4169e1ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"saddlebrown\0" as *const u8 as *const i8,
            val: 0x8b4513ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"salmon\0" as *const u8 as *const i8,
            val: 0xfa8072ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"sandybrown\0" as *const u8 as *const i8,
            val: 0xf4a460ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"seagreen\0" as *const u8 as *const i8,
            val: 0x2e8b57ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"seashell\0" as *const u8 as *const i8,
            val: 0xfff5eeff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"sienna\0" as *const u8 as *const i8,
            val: 0xa0522dff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"silver\0" as *const u8 as *const i8,
            val: 0xc0c0c0ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"skyblue\0" as *const u8 as *const i8,
            val: 0x87ceebff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"slateblue\0" as *const u8 as *const i8,
            val: 0x6a5acdff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"slategray\0" as *const u8 as *const i8,
            val: 0x708090ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"slategrey\0" as *const u8 as *const i8,
            val: 0x708090ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"snow\0" as *const u8 as *const i8,
            val: 0xfffafaff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"springgreen\0" as *const u8 as *const i8,
            val: 0xff7fff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"steelblue\0" as *const u8 as *const i8,
            val: 0x4682b4ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"tan\0" as *const u8 as *const i8,
            val: 0xd2b48cff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"teal\0" as *const u8 as *const i8,
            val: 0x8080ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"thistle\0" as *const u8 as *const i8,
            val: 0xd8bfd8ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"tomato\0" as *const u8 as *const i8,
            val: 0xff6347ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"turquoise\0" as *const u8 as *const i8,
            val: 0x40e0d0ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"violet\0" as *const u8 as *const i8,
            val: 0xee82eeff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"wheat\0" as *const u8 as *const i8,
            val: 0xf5deb3ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"white\0" as *const u8 as *const i8,
            val: 0xffffffff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"whitesmoke\0" as *const u8 as *const i8,
            val: 0xf5f5f5ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"yellow\0" as *const u8 as *const i8,
            val: 0xffff00ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"yellowgreen\0" as *const u8 as *const i8,
            val: 0x9acd32ff,
        };
        init
    },
    {
        let mut init = named_color {
            name: 0 as *const i8,
            val: 0,
        };
        init
    },
];
extern "C" fn h(mut c: i8) -> i32 {
    match c as i32 {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            return c as i32 - '0' as i32;
        }
        97 | 98 | 99 | 100 | 101 | 102 => {
            return c as i32 - 'a' as i32 + 10;
        }
        65 | 66 | 67 | 68 | 69 | 70 => {
            return c as i32 - 'A' as i32 + 10;
        }
        _ => {}
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn rgba_new(mut rgba: u32) -> rgba_t {
    let mut color = rgba_t {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    color.r = (rgba >> 24i32) as f64 / 255 as f64;
    color.g = (rgba >> 16 & 0xffu32) as f64 / 255 as f64;
    color.b = (rgba >> 8 & 0xffu32) as f64 / 255 as f64;
    color.a = (rgba & 0xffu32) as f64 / 255 as f64;
    return color;
}

#[no_mangle]
pub extern "C" fn rgba_to_string(mut rgba: rgba_t, mut buf: *mut i8, mut len: u64) {
    unsafe {
        if 1 as f64 == rgba.a {
            snprintf(
                buf,
                len,
                b"#%.2x%.2x%.2x\0" as *const u8 as *const i8,
                (rgba.r * 255 as f64) as i32,
                (rgba.g * 255 as f64) as i32,
                (rgba.b * 255 as f64) as i32,
            );
        } else {
            snprintf(
                buf,
                len,
                b"rgba(%d, %d, %d, %.2f)\0" as *const u8 as *const i8,
                (rgba.r * 255 as f64) as i32,
                (rgba.g * 255 as f64) as i32,
                (rgba.b * 255 as f64) as i32,
                rgba.a,
            );
        };
    }
}

#[inline]
extern "C" fn rgba_from_rgba(mut r: u8, mut g: u8, mut b: u8, mut a: u8) -> u32 {
    return ((r as i32) << 24 | (g as i32) << 16 | (b as i32) << 8 | a as i32) as u32;
}

extern "C" fn rgba_from_rgb(mut r: u8, mut g: u8, mut b: u8) -> i32 {
    return rgba_from_rgba(r, g, b, 255) as i32;
}

extern "C" fn rgba_from_hex6_string(mut str: *const i8) -> u32 {
    unsafe {
        return rgba_from_rgb(
            ((h(*str.offset(0 as isize)) << 4i32) + h(*str.offset(1 as isize))) as u8,
            ((h(*str.offset(2 as isize)) << 4i32) + h(*str.offset(3 as isize))) as u8,
            ((h(*str.offset(4 as isize)) << 4i32) + h(*str.offset(5 as isize))) as u8,
        ) as u32;
    }
}

extern "C" fn rgba_from_hex3_string(mut str: *const i8) -> i32 {
    unsafe {
        return rgba_from_rgb(
            ((h(*str.offset(0 as isize)) << 4i32) + h(*str.offset(0 as isize))) as u8,
            ((h(*str.offset(1 as isize)) << 4i32) + h(*str.offset(1 as isize))) as u8,
            ((h(*str.offset(2 as isize)) << 4i32) + h(*str.offset(2 as isize))) as u8,
        );
    }
}

extern "C" fn rgba_from_rgb_string(mut str: *const i8, mut ok: *mut i16) -> i32 {
    unsafe {
        if str == strstr(str, b"rgb(\0" as *const u8 as *const i8) as *const i8 {
            str = str.offset(4 as isize);
            while ' ' as i32 == *str as i32 {
                str = str.offset(1);
            }
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let mut c: i32 = 0;
            c = 0;
            if *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
                loop {
                    c *= 10;
                    let fresh0 = str;
                    str = str.offset(1);
                    c += *fresh0 as i32 - '0' as i32;
                    if !(*str as i32 >= '0' as i32 && *str as i32 <= '9' as i32) {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            r = c as u8;
            while ' ' as i32 == *str as i32 || ',' as i32 == *str as i32 {
                str = str.offset(1);
            }
            c = 0;
            if *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
                loop {
                    c *= 10;
                    let fresh1 = str;
                    str = str.offset(1);
                    c += *fresh1 as i32 - '0' as i32;
                    if !(*str as i32 >= '0' as i32 && *str as i32 <= '9' as i32) {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            g = c as u8;
            while ' ' as i32 == *str as i32 || ',' as i32 == *str as i32 {
                str = str.offset(1);
            }
            c = 0;
            if *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
                loop {
                    c *= 10;
                    let fresh2 = str;
                    str = str.offset(1);
                    c += *fresh2 as i32 - '0' as i32;
                    if !(*str as i32 >= '0' as i32 && *str as i32 <= '9' as i32) {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            b = c as u8;
            while ' ' as i32 == *str as i32 || ',' as i32 == *str as i32 {
                str = str.offset(1);
            }
            *ok = 1;
            return rgba_from_rgb(r, g, b);
        }
        *ok = 0;
        return *ok as i32;
    }
}

extern "C" fn rgba_from_rgba_string(mut str: *const i8, mut ok: *mut i16) -> i32 {
    unsafe {
        if str == strstr(str, b"rgba(\0" as *const u8 as *const i8) as *const i8 {
            str = str.offset(5 as isize);
            while ' ' as i32 == *str as i32 {
                str = str.offset(1);
            }
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let mut c: i32 = 0;
            let mut a = 0 as libc::c_float;
            c = 0;
            if *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
                loop {
                    c *= 10;
                    let fresh3 = str;
                    str = str.offset(1);
                    c += *fresh3 as i32 - '0' as i32;
                    if !(*str as i32 >= '0' as i32 && *str as i32 <= '9' as i32) {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            r = c as u8;
            while ' ' as i32 == *str as i32 || ',' as i32 == *str as i32 {
                str = str.offset(1);
            }
            c = 0;
            if *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
                loop {
                    c *= 10;
                    let fresh4 = str;
                    str = str.offset(1);
                    c += *fresh4 as i32 - '0' as i32;
                    if !(*str as i32 >= '0' as i32 && *str as i32 <= '9' as i32) {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            g = c as u8;
            while ' ' as i32 == *str as i32 || ',' as i32 == *str as i32 {
                str = str.offset(1);
            }
            c = 0;
            if *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
                loop {
                    c *= 10;
                    let fresh5 = str;
                    str = str.offset(1);
                    c += *fresh5 as i32 - '0' as i32;
                    if !(*str as i32 >= '0' as i32 && *str as i32 <= '9' as i32) {
                        break;
                    }
                }
            } else {
                return 0;
            }
            if c > 255 {
                c = 255;
            }
            b = c as u8;
            while ' ' as i32 == *str as i32 || ',' as i32 == *str as i32 {
                str = str.offset(1);
            }
            if *str as i32 >= '1' as i32 && *str as i32 <= '9' as i32 {
                a = 1 as libc::c_float;
            } else {
                if '0' as i32 == *str as i32 {
                    str = str.offset(1);
                }
                if '.' as i32 == *str as i32 {
                    str = str.offset(1);
                    let mut n = 0.1f64 as libc::c_float;
                    while *str as i32 >= '0' as i32 && *str as i32 <= '9' as i32 {
                        let fresh6 = str;
                        str = str.offset(1);
                        a += (*fresh6 as i32 - '0' as i32) as libc::c_float * n;
                        n = (n as f64 * 0.1f64) as libc::c_float;
                    }
                }
            }
            *ok = 1;
            return rgba_from_rgba(r, g, b, (a * 255 as libc::c_float) as u8) as i32;
        }
        *ok = 0;
        return *ok as i32;
    }
}

extern "C" fn rgba_from_hex_string(mut str: *const i8, mut ok: *mut i16) -> i32 {
    unsafe {
        let mut len = strlen(str);
        *ok = 1;
        if 6 == len {
            return rgba_from_hex6_string(str) as i32;
        }
        if 3 == len {
            return rgba_from_hex3_string(str);
        }
        *ok = 0;
        return *ok as i32;
    }
}

extern "C" fn rgba_from_name_string(mut str: *const i8, mut ok: *mut i16) -> i32 {
    unsafe {
        let mut i = 0;
        let mut color = named_color {
            name: 0 as *const i8,
            val: 0,
        };
        loop {
            let fresh7 = i;
            i = i + 1;
            color = named_colors[fresh7 as usize];
            if (color.name).is_null() {
                break;
            }
            if *str as i32 == *color.name as i32 && 0 == strcmp(str, color.name) {
                *ok = 1;
                return color.val as i32;
            }
        }
        *ok = 0;
        return *ok as i32;
    }
}

#[no_mangle]
pub extern "C" fn rgba_from_string(mut str: *const i8, mut ok: *mut i16) -> u32 {
    unsafe {
        if '#' as i32 == *str.offset(0 as isize) as i32 {
            str = str.offset(1);
            return rgba_from_hex_string(str, ok) as u32;
        }
        if str == strstr(str, b"rgba\0" as *const u8 as *const i8) as *const i8 {
            return rgba_from_rgba_string(str, ok) as u32;
        }
        if str == strstr(str, b"rgb\0" as *const u8 as *const i8) as *const i8 {
            return rgba_from_rgb_string(str, ok) as u32;
        }
        return rgba_from_name_string(str, ok) as u32;
    }
}

#[no_mangle]
pub extern "C" fn rgba_inspect(mut rgba: u32) {
    print!("rgba({},{},{},{})\n,9999,9999,9999,9999");
}
