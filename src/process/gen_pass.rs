use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SPECIAL: &[u8] = b"!@#$%^&*()_+-=";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    special: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = vec![];
    let mut chars = Vec::new();
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("LOWER won't be empty in this context"),
        );
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("NUMBER won't be empty in this context"),
        );
    }
    if special {
        chars.extend_from_slice(SPECIAL);
        password.push(
            *SPECIAL
                .choose(&mut rng)
                .expect("SPECIAL won't be empty in this context"),
        );
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);
    let estimate = zxcvbn(&password, &[]);
    eprintln!("{:?}", estimate.score());
    Ok(())
}
