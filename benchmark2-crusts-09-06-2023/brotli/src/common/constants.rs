use libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliPrefixCodeRange {
    pub offset: u16,
    pub nbits: u8,
}
#[no_mangle]
pub static mut _kBrotliPrefixCodeRanges: [BrotliPrefixCodeRange; 26] = [
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 1,
            nbits: 2,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 5,
            nbits: 2,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 9,
            nbits: 2,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 13,
            nbits: 2,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 17,
            nbits: 3,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 25,
            nbits: 3,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 33,
            nbits: 3,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 41,
            nbits: 3,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 49,
            nbits: 4,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 65,
            nbits: 4,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 81,
            nbits: 4,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 97,
            nbits: 4,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 113,
            nbits: 5,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 145,
            nbits: 5,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 177,
            nbits: 5,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 209,
            nbits: 5,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 241,
            nbits: 6,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 305,
            nbits: 6,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 369,
            nbits: 7,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 497,
            nbits: 8,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 753,
            nbits: 9,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 1265,
            nbits: 10,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 2289,
            nbits: 11,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 4337,
            nbits: 12,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 8433,
            nbits: 13,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 16625,
            nbits: 24,
        };
        init
    },
];
