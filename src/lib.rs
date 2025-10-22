use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{
    self as zed, Result, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

struct LiveServerExtension {
    cached_binary_path: Option<String>,
}

impl LiveServerExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "frederik-uni/live-server-lsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();

        let asset_name = format!(
            "live-server-lsp-{arch}-{os}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-musl",
                zed::Os::Windows => "pc-windows-msvc.exe",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("live-server-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;

        let binary_path = format!(
            "{version_dir}/{bin_name}",
            bin_name = match platform {
                zed::Os::Windows => "live-server.exe",
                zed::Os::Mac | zed::Os::Linux => "live-server",
            }
        );

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|err| format!("failed to download file: {err}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries = fs::read_dir(".")
                .map_err(|err| format!("failed to list working directory {err}"))?;
            for entry in entries {
                let entry = entry.map_err(|err| format!("failed to load directory entry {err}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for LiveServerExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id)?,
            args: vec!["--eager".to_string()], //"--public", "--port", "1234"
            env: Default::default(),
        })
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "live-server-start" => {
                let output_text = "🚀 Starting Live Server...\n\nTo check status, use /live-server-status\nTo stop the server, use /live-server-stop";

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..output_text.len()).into(),
                        label: "Live Server - Start".to_string(),
                    }],
                    text: output_text.to_string(),
                })
            }
            "live-server-stop" => {
                let output_text = "⏹️  Stopping Live Server...\n\nThe Live Server has been stopped.\nTo start again, use /live-server-start";

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..output_text.len()).into(),
                        label: "Live Server - Start".to_string(),
                    }],
                    text: output_text.to_string(),
                })
            }
            "live-server-status" => {
                let output_text = "🌐 Opening Live Server in browser...\n\n🔗 URL: http://127.0.0.1:57391\n\nIf the browser doesn't open automatically, copy the URL above and paste it in your browser.";

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (9..output_text.len()).into(),
                        label: "Live Server - Open Browser".to_string(),
                    }],
                    text: output_text.to_string(),
                })
            }
            
            "live-server-open" => {
                let output_text: &'static str = "🌐 Opening Live Server in browser...\n\n🔗 URL: http://127.0.0.1:57391\n\nIf the browser doesn't open automatically, copy the URL above and paste it in your browser.";
                
                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection{
                        range: (0..output_text.len()).into(),
                        label: "Live Server - Open Browser".to_string(),
                    }],
                    text: output_text.to_string(),
                })
            }
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }
}

zed::register_extension!(LiveServerExtension);
