use anyhow::Result;
use std::path::Path;

pub fn main(_path: &Path) -> Result<(usize, Option<usize>)> {
    Ok((0, None))
}

#[cfg(test)]
mod test {
    use super::*;

    // test_real_input!(1, 0, 0);
}
