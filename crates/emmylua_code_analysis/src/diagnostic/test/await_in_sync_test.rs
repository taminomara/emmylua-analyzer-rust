#[cfg(test)]
mod test {
    use crate::DiagnosticCode;

    #[test]
    fn test_await_in_sync() {
        let mut ws = crate::VirtualWorkspace::new_with_init_std_lib();

        assert!(!ws.check_file_for(
            DiagnosticCode::AwaitInSync,
            r#"
        local function name(callback)

        end

        name(function()
            local a = coroutine.yield(1)
        end)
        "#
        ));

        assert!(ws.check_file_for(
            DiagnosticCode::AwaitInSync,
            r#"
        local function name(callback)

        end

        ---@async
        name(function()
            local a = coroutine.yield(1)
        end)
        "#
        ));

        assert!(ws.check_file_for(
            DiagnosticCode::AwaitInSync,
            r#"
        ---@param callback async fun()
        local function name(callback)

        end

        name(function()
            local a = coroutine.yield(1)
        end)
        "#
        ));
    }
}
