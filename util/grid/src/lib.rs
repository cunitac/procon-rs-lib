/// `(0, -1), (1, 0), (0, 1), (-1, 0)`
/// (下, 右) の座標系なら LDRU の順
pub const D4: [(usize, usize); 4] = [
    (0, 1usize.wrapping_neg()),
    (1, 0),
    (0, 1),
    (1usize.wrapping_neg(), 0),
];

/// `(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)`
/// (下, 右) の座標系なら LDRU の順
pub const D8: [(usize, usize); 8] = [
    (0, 1usize.wrapping_neg()),
    (1, 1usize.wrapping_neg()),
    (1, 0),
    (1, 1),
    (0, 1),
    (1usize.wrapping_neg(), 1),
    (1usize.wrapping_neg(), 0),
    (1usize.wrapping_neg(), 1usize.wrapping_neg()),
];

pub fn adj4(
    (i, j): (usize, usize),
    (h, w): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    D4.iter()
        .map(move |&(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
        .filter(move |&(i, j)| i < h && j < w)
}

/// `D4[ord[0]], D4[ord[1]], ...` の順に訪れる
/// `ord` の各要素は `0..4` に含まれる必要があるが、 `0..4` の順列である必要は**ない**
pub fn adj4_ord(
    (i, j): (usize, usize),
    (h, w): (usize, usize),
    ord: impl IntoIterator<Item = usize>,
) -> impl Iterator<Item = (usize, usize)> {
    ord.into_iter()
        .map(|i| D4[i])
        .map(move |(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
        .filter(move |&(i, j)| i < h && j < w)
}

pub fn adj8(
    (i, j): (usize, usize),
    (h, w): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    D8.iter()
        .map(move |&(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
        .filter(move |&(i, j)| i < h && j < w)
}

/// `D8[ord[0]], D8[ord[1]], ...` の順に訪れる
/// `ord` の各要素は `0..8` に含まれる必要があるが、`0..8` の順列である必要は**ない**
pub fn adj8_ord(
    (i, j): (usize, usize),
    (h, w): (usize, usize),
    ord: impl IntoIterator<Item = usize>,
) -> impl Iterator<Item = (usize, usize)> {
    ord.into_iter()
        .map(|i| D8[i])
        .map(move |(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
        .filter(move |&(i, j)| i < h && j < w)
}
