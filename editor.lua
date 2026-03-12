-- rust-analyzer (and Neovim in general) configuration for, well, Neovim.
-- Source this file before opening any .rs file, so rust-analyzer gets configured properly.
-- Change the specific features depending on what you're working on.

vim.lsp.config('rust_analyzer', {
  settings = {
    ['rust-analyzer'] = {
        cargo = {
            features = { "control_scheme_power_aux", "target_naegi" }
        }
    }
  }
})
