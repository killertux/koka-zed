use std::fs;

use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result};

struct KokaExtension {
    cached_binary_path: Option<String>,
}

impl KokaExtension {
    fn resolve_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
        configured_path: Option<String>,
    ) -> Result<String> {
        if let Some(path) = configured_path {
            return Ok(path);
        }
        if let Some(path) = worktree.which("koka") {
            return Ok(path);
        }
        self.ensure_downloaded(language_server_id)
    }

    fn ensure_downloaded(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "koka-lang/koka",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let os = match platform {
            zed::Os::Mac => "macos",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "windows",
        };
        let arch = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "x64",
            zed::Architecture::X86 => {
                return Err(format!(
                    "Koka has no prebuilt asset for {os}-x86. \
                     Install Koka manually and ensure `koka` is on your PATH."
                ));
            }
        };

        let asset_suffix = format!("-{os}-{arch}.tar.gz");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name.starts_with("koka-") && a.name.ends_with(&asset_suffix))
            .ok_or_else(|| {
                format!(
                    "no Koka release asset matched suffix `{asset_suffix}` in release {}",
                    release.version
                )
            })?;

        // Tarball expands to `bin/`, `share/`, `lib/` at the top level. Extract into a
        // version-named dir so we can keep multiple installs side-by-side and the
        // `koka` binary finds its stdlib via `../share/koka/v<ver>/...`.
        let version_dir = format!("koka-{}", release.version.trim_start_matches('v'));
        let bin_suffix = if matches!(platform, zed::Os::Windows) {
            "bin/koka.exe"
        } else {
            "bin/koka"
        };
        let binary_path = format!("{version_dir}/{bin_suffix}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )
            .map_err(|e| format!("failed to download Koka: {e}"))?;

            // Drop any older auto-downloaded versions to keep the cache lean.
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    if name.starts_with("koka-") && name != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            return Err(format!(
                "downloaded Koka archive but binary not found at `{binary_path}`"
            ));
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for KokaExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.binary);

        let configured_path = binary_settings.as_ref().and_then(|b| b.path.clone());
        let extra_args: Vec<String> = binary_settings
            .as_ref()
            .and_then(|b| b.arguments.clone())
            .unwrap_or_default();

        let command = self.resolve_binary(language_server_id, worktree, configured_path)?;

        let root = worktree.root_path();
        let mut args = vec![
            "--language-server".to_string(),
            "--buildtag=zed".to_string(),
            format!("-i{}", root),
            "--lsstdio".to_string(),
        ];
        args.extend(extra_args);

        Ok(zed::Command {
            command,
            args,
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.settings.clone())
            .unwrap_or_default();

        Ok(Some(zed::serde_json::json!({ "koka": settings })))
    }
}

zed::register_extension!(KokaExtension);
