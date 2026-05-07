# Koka for Zed

A [Zed](https://zed.dev) extension that adds support for the [Koka](https://koka-lang.github.io/koka/doc/index.html) programming language.

Features:

- Syntax highlighting (via [tree-sitter-koka](https://github.com/koka-community/tree-sitter-koka))
- Indentation rules
- Bracket / autoclose pair handling
- Integration with the language server bundled in the Koka compiler (`koka --language-server`)

## Requirements

- Zed (recent stable; tested against the extension API v0.6).
- The Zed project must be **trusted** — Zed gates language servers from running in untrusted workspaces. If you don't see diagnostics, check the trust prompt at the top of the editor.

The extension finds `koka` in this order:

1. The `lsp.koka.binary.path` setting (if set).
2. The `koka` binary on your `PATH`.
3. **Auto-download** — if neither of the above resolves, the extension fetches the latest Koka release from GitHub for your platform (macOS arm64/x64, Linux arm64/x64, Windows x64) and caches it under the extension's working directory. The download includes the standard library, so the auto-installed compiler is fully functional from Zed (though it won't be on your shell `PATH`).

To get a system-wide install you can also use from the terminal, run the official installer:

```bash
curl -sSL https://github.com/koka-lang/koka/releases/latest/download/install.sh | sh
```

or `brew install koka` on macOS.

## Install as a dev extension

This extension is not yet on the Zed extensions registry. To run it locally:

1. Clone this repo.
2. In Zed, open the command palette (`cmd-shift-p`) and run **`zed: extensions`**.
3. Click **"Install Dev Extension"** and select this directory.

Zed will compile the WASM extension and the tree-sitter grammar in place. To iterate on changes, edit files and click **"Rebuild"** on the dev extension entry, or remove and re-add it.

## Settings

The extension respects standard Zed LSP settings under the `koka` server name. In your `settings.json`:

```json
{
  "lsp": {
    "koka": {
      "binary": {
        "path": "/absolute/path/to/koka",
        "arguments": ["--verbose"]
      },
      "settings": {
        "inlayHints": { "showImplicitArguments": true }
      }
    }
  }
}
```

- `binary.path` — override the resolved `koka` binary (otherwise resolved from `PATH`).
- `binary.arguments` — extra args appended after `--language-server --buildtag=zed -i<root> --lsstdio`.
- `settings` — forwarded to the language server as workspace configuration under the `koka.*` namespace.

## Credits

- [Koka](https://github.com/koka-lang/koka) — the language and compiler (which provides the language server and the original VS Code extension this port mirrors).
- [koka-community/tree-sitter-koka](https://github.com/koka-community/tree-sitter-koka) — the tree-sitter grammar and highlight queries.

## License

Apache-2.0.
