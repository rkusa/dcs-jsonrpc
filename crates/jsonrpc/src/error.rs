use lua51 as ffi;

pub fn assert_stack_size(l: *mut ffi::lua_State, expected: usize) -> Result<(), anyhow::Error> {
    let curr = unsafe { ffi::lua_gettop(l) } as usize;
    if curr != expected {
        Err(anyhow!(
            "Expected a Lua stack size of {}, got {}",
            expected,
            curr
        ))
    } else {
        Ok(())
    }
}

pub fn argument_type_error(name: &str, kind: &str) -> anyhow::Error {
    anyhow!("Expected argument {} to be of type {}", name, kind)
}
