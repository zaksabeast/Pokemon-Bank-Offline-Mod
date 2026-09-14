pub type CtrResult<T> = Result<T, i32>;

pub fn parse_res(res: i32) -> CtrResult<()> {
    if res >= 0 {
        return Ok(());
    }

    Err(res)
}

pub fn parse_res_u32(res: u32) -> CtrResult<()> {
    let res = res as i32;
    if res >= 0 {
        return Ok(());
    }

    Err(res)
}
