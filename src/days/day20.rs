use crate::common::Solution;

struct Image {
    pixels: Vec<Vec<u16>>,
    infinity: u16,
}

#[allow(unused)]
fn print_img(img: &Image) {
    for row in &img.pixels {
        println!(
            "{}",
            row.iter()
                .map(|i| if *i == 1 { '#' } else { '.' })
                .collect::<String>()
        );
    }
}

fn expand(mut img: Image) -> Image {
    for row in &mut img.pixels {
        row.splice(0..0, std::iter::repeat(img.infinity).take(2));
        row.push(img.infinity);
        row.push(img.infinity);
    }
    img.pixels.splice(
        0..0,
        std::iter::repeat(vec![img.infinity; img.pixels[0].len()]).take(2),
    );
    img.pixels.push(vec![img.infinity; img.pixels[0].len()]);
    img.pixels.push(vec![img.infinity; img.pixels[0].len()]);
    img
}

fn enhance(alg: &[u16], img: Image) -> Image {
    let img = expand(img);
    Image {
        pixels: (1..(img.pixels.len() - 1))
            .map(|i_row| {
                (1..(img.pixels[0].len() - 1))
                    .map(|i_col| {
                        ((i_row - 1)..=(i_row + 1))
                            .flat_map(|conv_row| {
                                ((i_col - 1)..=(i_col + 1))
                                    .map(move |conv_col| (conv_row, conv_col))
                            })
                            .fold(0, |addr, (conv_row, conv_col)| {
                                (addr << 1) | img.pixels[conv_row][conv_col]
                            })
                    })
                    .map(|i| alg[usize::from(i)])
                    .collect()
            })
            .collect(),
        infinity: alg[if img.infinity == 1 { 511 } else { 0 }],
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let alg: Vec<u16> = lines[0]
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();

    let img: Image = Image {
        pixels: lines[2..]
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
            .collect(),
        infinity: 0,
    };

    let enhanced = enhance(&alg, img);
    let enhanced2 = enhance(&alg, enhanced);

    let sol_a = enhanced2
        .pixels
        .iter()
        .flat_map(|row| row.iter())
        .filter(|i| **i != 0)
        .count();
    let sol_b = (2..50)
        .fold(enhanced2, |img, _| enhance(&alg, img))
        .pixels
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|i| *i != 0)
        .count();

    (sol_a.to_string(), sol_b.to_string())
}
