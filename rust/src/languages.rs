use std::cmp::*;

use std::fmt;
use std::io;

use std::io::Read;

// From Project Gutenberg
static ENGLISH_UTF8: Frequencies = Frequencies { 
    histogram: [
        13, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        384035805, 1123327, 7321259, 14835, 28553, 4135, 165612, 4830175, 963365, 965784, 419617, 88192, 31583405, 7372705, 21790098, 98002, 
        1437354, 2840886, 1899425, 1527604, 1452646, 1410993, 1363304, 1264624, 1311226, 1122150, 991787, 4093888, 31901, 100809, 32710, 1165544, 
        11296, 5633685, 2871772, 3294048, 2039758, 3729813, 1910711, 1875503, 3540484, 8040216, 1250024, 647512, 2414273, 3527972, 2585121, 2394119, 
        2242129, 146172, 2964304, 4777371, 6295483, 752026, 706806, 2631954, 200577, 965475, 86984, 562022, 1423, 554950, 9351, 3465454, 
        2340, 130884173, 23438350, 42243181, 69024434, 210634657, 38708937, 31191472, 100015788, 110295057, 1536641, 10939137, 65914244, 39493750, 116305618, 126880206, 
        28589079, 1731052, 100440282, 104205185, 149461559, 46929093, 16342510, 34023166, 2754435, 30591688, 1063748, 11025, 223609, 10426, 9606, 1, 
        8392, 4143, 984, 443, 1145, 251, 4024, 884, 919, 5633, 191, 198, 162, 1623, 284, 106, 
        181, 680, 263, 1081, 1492, 33, 641, 208, 49, 2516, 1157, 432, 2485, 1902, 376, 35, 
        22703, 18820, 19806, 8549, 4281, 225, 16019, 7986, 17220, 405194, 7526, 3797, 496, 6050, 3462, 2489, 
        12434, 8146, 1211, 4675, 7051, 700, 9283, 2496, 264, 2568, 4495, 2517, 8480, 2700, 1332, 1498, 
        3, 0, 372566, 254259, 35, 868, 18, 0, 0, 19, 0, 2, 773, 26, 5453, 3236, 
        0, 0, 0, 2, 0, 0, 3, 148, 0, 0, 0, 0, 0, 0, 7, 0, 
        130, 1582, 7723, 0, 132, 0, 180, 87, 226, 1144, 106, 87, 4, 6, 7, 64, 
        0, 259, 3, 7, 86, 0, 149, 2, 0, 9, 6, 32, 91, 0, 19, 0], 
    total: 2190181928 
};

pub struct Frequencies {
    histogram: [u32; 256],
    total: u64,
}

impl Frequencies {
    pub fn from_bytes(bytes: &[u8]) -> Frequencies {
        let mut histogram = [0u32; 256];
        let mut total = 0;

        for &b in bytes {
            histogram[b as usize] += 1;
            total += 1;
        }

        Frequencies { histogram: histogram, total: total }
    }

    pub fn read_from<T: Read>(source: &mut T) -> io::Result<Frequencies> {
        let mut bytes = Vec::new();

        let total = try!(source.read_to_end(&mut bytes)) as u64;

        let mut histogram = [0u32; 256];

        for b in bytes {
            histogram[b as usize] += 1;
        }

        Ok(Frequencies { histogram: histogram, total: total })
    }

    pub fn compare(&self, other: Frequencies) -> f32 {
        let sum_x =  self.total;
        let sum_y = other.total;

        let xs =  self.histogram.iter().map(|&x| sum_y * (x as u64));
        let ys = other.histogram.iter().map(|&y| sum_x * (y as u64));

        xs.zip(ys)
            .map(|(x, y)| max(x, y) - min(x, y))
            .map(|diff| diff as f32 / (sum_x * sum_y) as f32)
            .fold(0.0f32, |acc, x| acc + x)
    }
}

// Need this in order to impl Debug for [u32; 256]
// which is needed in order to impl Debug for Frequencies
struct ArrayWrapper<'a>(&'a [u32; 256]);

impl<'a> fmt::Debug for ArrayWrapper<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list().entries(self.0.iter()).finish()
    }
}

impl fmt::Debug for Frequencies {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Frequencies")
            .field("histogram", &ArrayWrapper(&self.histogram))
            .field("total", &self.total)
            .finish()
    }
}

#[derive(Clone)]
pub enum Language {
    EnglishUtf8,
}

impl Language {
    fn frequencies(&self) -> &'static Frequencies {
        match *self {
            Language::EnglishUtf8 => &ENGLISH_UTF8
        }
    }

    pub fn compare(&self, bytes: &[u8]) -> f32 {
        self.frequencies().compare(Frequencies::from_bytes(bytes))
    }
}
