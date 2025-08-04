use tauri::{Runtime, WebviewWindow};

#[cfg(target_os = "macos")]
use window_vibrancy::{NSVisualEffectMaterial, NSVisualEffectState};

/// 设置窗口属性
pub fn set_window_attribute<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), Box<dyn std::error::Error>> {
    // 设置窗口阴影
    window.set_decorations(true)?;
    #[cfg(any(windows, target_os = "macos"))]
    window.set_shadow(true)?;

    // macOS 特定设置
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(
        window,
        NSVisualEffectMaterial::HudWindow,
        Some(NSVisualEffectState::Active),
        None,
    )
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Windows 特定设置
    #[cfg(target_os = "windows")]
    {
        use window_vibrancy::apply_blur;
        apply_blur(window, Some((18, 18, 18, 125)))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    }

    // 设置窗口基本属性
    window.set_always_on_top(true)?;
    window.set_resizable(true)?;
    window.set_title("SwitchHostsR")?;

    // 设置窗口大小限制
    window.set_min_size(Some(tauri::LogicalSize::new(800.0, 600.0)))?;

    Ok(())
}
