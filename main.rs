// Весь проект: yuaibro - одним файлом

/* === CARGO TOML START ===

[package]
name = "yuaibro"
version = "0.1.0"
edition = "2021"
rust-version = "1.90"

[dependencies]
log = "0.4"
env_logger = "0.11"
anyhow = "1.0"
dirs = "5.0"
serde_yaml = "0.9"
pollster = "0.3"
serde = { version = "1.0", features = ["derive"] }
toml = "0.9.8"
uuid = { version = "1.10", features = ["v4"] }
tendril = "0.4"
markup5ever_rcdom = "0.3"
tokio = { version = "1.48.0", features = ["full"] }
ssh2 = "=0.9.5"
sysinfo = { version = "0.37.2", optional = true } 
once_cell = "=1.21.3"
image = "=0.25.8"
async-trait = "=0.1.89"
bytes = "=1.10.1"
url = "=2.5.7"
futures = "=0.3.31"
fontdb = "=0.23.0"
rayon = "1.11.0"
crossbeam-channel = "0.5.15"
parking_lot = "0.12.5"

[dependencies.boa_engine]
version = "0.20"
optional = true

[dependencies.winit]
version = "0.30.12"
optional = true

[dependencies.wgpu]
version = "27.0.1"
optional = true

[dependencies.egui]
version = "0.33"
optional = true

[dependencies.egui-winit]
version = "0.33"
optional = true

[dependencies.egui-wgpu]
version = "0.33"
optional = true

[dependencies.egui_extras]
version = "0.33"
optional = true

[dependencies.html5ever]
version = "0.27"
optional = true

[dependencies.markup5ever]
version = "0.11"
optional = true

[dependencies.cssparser]
version = "0.35"
optional = true

[dependencies.selectors]
version = "0.25"
optional = true

[dependencies.wasmtime]
version = "25.0"
optional = true

[dependencies.reqwest]
version = "0.12.24"
optional = true

[dependencies.rustls]
version = "0.23"
optional = true

[dependencies.libp2p]
version = "0.56"
optional = true

[dependencies.wasmcloud-host]
version = "0.26"
optional = true

[dependencies.wasmcloud-control-interface]
version = "0.27"
optional = true

[dependencies.ed25519-dalek]
version = "2.2"
optional = true

[dependencies.rand]
version = "0.9"
optional = true

[dependencies.fontdue]
version = "0.9"
optional = true

[dependencies.flate2]
version = "1.0"
optional = true

[dependencies.brotli]
version = "8.0"
optional = true

[dependencies.dashmap]
version = "6.1"
optional = true

[dependencies.prost]
version = "0.13"
optional = true

[dependencies.oci-distribution]
version = "0.11"
optional = true

[dependencies.serde_json]
version = "1.0.145"
optional = true


[features]
default = ["ui", "network", "vdom", "sysinfo"] 
ui = ["winit", "wgpu", "egui", "egui-winit", "egui-wgpu", "egui_extras"]
network = ["reqwest", "rustls"]
vdom = ["html5ever", "markup5ever", "cssparser", "selectors", "serde_json"]
js = ["boa_engine"]
orchestration = ["wasmcloud-host", "wasmcloud-control-interface", "wasmtime"]
p2p = ["libp2p"]
security = ["ed25519-dalek", "rand"]
rendering = ["fontdue", "flate2", "brotli"]
wasm = ["wasmtime", "prost", "oci-distribution"]
reactive = ["dashmap"]

[profile.release]
opt-level = "z"  # Минимальный размер кода
lto = true       # Link Time Optimization (оптимизация при линковке)
strip = true     # Убираем символы отладки (Cargo 1.60+)
panic = "abort"  # Отключаем unwinding
codegen-units = 1 # Улучшает оптимизацию, но увеличивает время компиляции
=== CARGO TOML END === */

// === FILE: core\config.rs ===
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use dirs::home_dir;
#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub modules: ModulesConfig,
    pub settings: Settings,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct ModulesConfig {
    pub core_enabled: bool,
    pub ui_enabled: bool,
    pub network_enabled: bool,
    pub dom_enabled: bool,
    pub js_enabled: bool,
    pub wasmcloud_enabled: bool,
    pub p2p_enabled: bool,
    pub vdom_enabled: bool,
    pub security_enabled: bool,
    pub rendering_enabled: bool,
    pub wasm_enabled: bool,
    pub reactive_enabled: bool,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Settings {
    pub cache_enabled: bool,
    pub logging_level: String,
    pub max_threads: usize,
    pub default_url: String,
    pub font_family: String,
	pub scale_factor: Option<f32>,
	pub graphics_backend: Option<String>,
}
impl Config {
    pub fn get_config_path() -> std::path::PathBuf {
        home_dir()
            .unwrap_or_default()
            .join(".cosmonaut")
            .join("settings.toml")
    }
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let config_str = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Не удалось прочитать конфигурацию из {}: {}", path.display(), e))?;
        let config: Config = toml::from_str(&config_str)
            .map_err(|e| anyhow::anyhow!("Ошибка парсинга конфигурации: {}", e))?;
        log::info!("Конфигурация загружена из {:?}", path);
        Ok(config)
    }
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?; // Создаём директорию, если не существует
        }
        let mut file = File::create(path)?;
        let toml = toml::to_string(self)
            .map_err(|e| anyhow::anyhow!("Ошибка сериализации конфигурации: {}", e))?;
        file.write_all(toml.as_bytes())?;
        log::info!("Конфигурация сохранена в {:?}", path);
        Ok(())
    }
    pub fn validate(&self) -> Result<()> {
        if self.settings.max_threads == 0 {
            return Err(anyhow::anyhow!("max_threads должен быть больше 0"));
        }
        if self.settings.font_family.is_empty() {
            return Err(anyhow::anyhow!("font_family не может быть пустым"));
        }
        log::info!("Конфигурация прошла валидацию");
        Ok(())
    }
}

// === FILE: core\engine.rs ===
use anyhow::Result;
use tokio::sync::broadcast::{channel, Sender as BroadcastSender, Receiver as BroadcastReceiver};
use std::sync::{Arc, Mutex};
use crate::dom::parser::FrameworkType;
use reqwest::header::HeaderMap;
use std::time::{Duration, Instant};

use crate::core::config::Config;
use crate::core::interfaces::{
    NetworkTrait, JsRuntimeTrait, WasmRuntimeTrait, OrchestratorTrait, PluginManagerTrait,
    LibManagerTrait, YuaidbTrait, SecurityManagerTrait, ReactiveCoreTrait, SessionManagerTrait,
    ProfileManagerTrait, ServiceWorkerTrait, SchedulerTrait, IoManagerTrait, WasmManifestTrait,
};

// События движка
#[derive(Clone, Debug)]
pub enum EngineEvent {
	FrameworkDetected(FrameworkType),
    PageLoaded,
    Error(String),
    ScriptExecuted,
    PluginLoaded(String),
    UrlRequest { url: String, response_tx: tokio::sync::mpsc::UnboundedSender<UrlResponse> },
    UrlResponse(UrlResponse),
}

// Структура ответа сети
#[derive(Clone, Debug)]
pub struct UrlResponse {
    pub url: String,
    pub html: String,
    pub headers: HeaderMap,
    pub status: u16,
    pub duration: Duration,
}

pub struct BroEngine {
    network: Arc<dyn NetworkTrait + Send + Sync>,
    js_runtime: Arc<dyn JsRuntimeTrait + Send + Sync>,
    wasm_runtime: Arc<dyn WasmRuntimeTrait + Send + Sync>,
    orchestrator: Arc<dyn OrchestratorTrait + Send + Sync>,
    plugin_manager: Arc<dyn PluginManagerTrait + Send + Sync>,
    lib_manager: Arc<dyn LibManagerTrait + Send + Sync>,
    db: Arc<dyn YuaidbTrait + Send + Sync>,
    security: Arc<dyn SecurityManagerTrait + Send + Sync>,
    reactive_core: Arc<dyn ReactiveCoreTrait + Send + Sync>,
    session_manager: Arc<dyn SessionManagerTrait + Send + Sync>,
    profile_manager: Arc<dyn ProfileManagerTrait + Send + Sync>,
    service_worker: Arc<dyn ServiceWorkerTrait + Send + Sync>,
    scheduler: Arc<dyn SchedulerTrait + Send + Sync>,
    io_manager: Arc<dyn IoManagerTrait + Send + Sync>,
    wasm_manifest: Arc<dyn WasmManifestTrait + Send + Sync>,
    tx: BroadcastSender<EngineEvent>, // Broadcast Sender для событий
    rx: Arc<Mutex<BroadcastReceiver<EngineEvent>>>, // Обернутый Receiver для shared mutable доступа
}

impl BroEngine {
    pub async fn new(
        network: Arc<dyn NetworkTrait + Send + Sync>,
        js_runtime: Arc<dyn JsRuntimeTrait + Send + Sync>,
        wasm_runtime: Arc<dyn WasmRuntimeTrait + Send + Sync>,
        orchestrator: Arc<dyn OrchestratorTrait + Send + Sync>,
        plugin_manager: Arc<dyn PluginManagerTrait + Send + Sync>,
        lib_manager: Arc<dyn LibManagerTrait + Send + Sync>,
        db: Arc<dyn YuaidbTrait + Send + Sync>,
        security: Arc<dyn SecurityManagerTrait + Send + Sync>,
        reactive_core: Arc<dyn ReactiveCoreTrait + Send + Sync>,
        session_manager: Arc<dyn SessionManagerTrait + Send + Sync>,
        profile_manager: Arc<dyn ProfileManagerTrait + Send + Sync>,
        service_worker: Arc<dyn ServiceWorkerTrait + Send + Sync>,
        scheduler: Arc<dyn SchedulerTrait + Send + Sync>,
        io_manager: Arc<dyn IoManagerTrait + Send + Sync>,
        wasm_manifest: Arc<dyn WasmManifestTrait + Send + Sync>,
        _config: Option<Config>,
        _tx: Option<BroadcastSender<EngineEvent>>,
        _rx: Option<BroadcastReceiver<EngineEvent>>,
    ) -> Result<Self> {
        let (tx, rx) = match (_tx, _rx) {
            (Some(tx), Some(rx)) => (tx, rx),
            _ => channel(1024), // Broadcast канал с буфером
        };
        let rx = Arc::new(Mutex::new(rx)); // Оборачиваем в Arc<Mutex> для shared доступа
        Ok(Self {
            network,
            js_runtime,
            wasm_runtime,
            orchestrator,
            plugin_manager,
            lib_manager,
            db,
            security,
            reactive_core,
            session_manager,
            profile_manager,
            service_worker,
            scheduler,
            io_manager,
            wasm_manifest,
            tx,
            rx,
        })
    }

    pub fn send_event(&self, event: EngineEvent) {
        if let Err(e) = self.tx.send(event) { // Отправка события в broadcast
            log::error!("Ошибка отправки события движка: {}", e);
        }
    }

    pub fn receive_event(&self) -> Option<EngineEvent> {
        self.rx.lock().unwrap().try_recv().ok() // Неблокирующий приём через lock
    }

	pub fn event_receiver(&self) -> BroadcastReceiver<EngineEvent> {
        self.rx.lock().unwrap().resubscribe() // Создание нового ресивера через resubscribe
    }

    // Централизованный метод для выполнения сетевых запросов
    pub async fn fetch_url(&self, url: String) -> Result<UrlResponse> {
        // Проверка валидности URL
        if url.is_empty() || url == "about:blank" {
            return Err(anyhow::anyhow!("Недопустимый URL: {}", url));
        }

        let start_time = Instant::now();
        log::info!("Отправка сетевого запроса для URL: {}", url);

        // Выполнение запроса через NetworkTrait
        let (html, headers) = self
            .network
            .fetch_html_with_headers(&url)
            .await
            .map_err(|e| anyhow::anyhow!("Ошибка сетевого запроса для URL {}: {}", url, e))?;

        let duration = start_time.elapsed();
        let status = 200; // Предполагаем успешный статус, так как fetch_html_with_headers не возвращает статус явно

        let response = UrlResponse {
            url: url.clone(),
            html,
            headers,
            status,
            duration,
        };

        // Отправляем событие с ответом
        self.send_event(EngineEvent::UrlResponse(response.clone()));
        log::info!(
            "Сетевой запрос завершён: URL={}, статус={}, длительность={:?}",
            url,
            status,
            duration
        );

        Ok(response)
    }
}

// === FILE: core\event_loop.rs ===
// === FILE: core/event_loop.rs ===
use anyhow::Result;
use std::sync::Arc;
use winit::{
    event::WindowEvent,
    event_loop::{EventLoop, ActiveEventLoop},
    window::{Window as WinitWindow, WindowAttributes},
    application::ApplicationHandler,
};
use crate::core::{
    config::Config,
    engine::BroEngine,
};
use crate::ui::window::Window;
use tokio::task;
use log;

pub struct EventLoopContext {
    pub config: Config,
    pub engine: Arc<BroEngine>,
}

pub struct App {
    main_window: Option<Arc<WinitWindow>>,
    ctx: EventLoopContext,
    window: Option<Window>,
}

impl App {
    pub fn new(ctx: EventLoopContext) -> Self {
        Self {
            main_window: None,
            ctx,
            window: None,
        }
    }

    fn init_window(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        if self.main_window.is_none() {
            // Создаем окно через ActiveEventLoop
            log::debug!("Создание окна через ActiveEventLoop");
            let winit_window = event_loop.create_window(
                WindowAttributes::default().with_title("БРА")
            )?;
            self.main_window = Some(Arc::new(winit_window));
        }

        if self.window.is_none() {
            let window = task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(Window::new(
                    self.main_window.as_ref().unwrap().clone(),
                    &self.ctx.config,
                    self.ctx.engine.clone(),
                    true,
                    event_loop,
                ))
            })?;
            self.window = Some(window);
        }
        Ok(())
    }
}

impl ApplicationHandler<()> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Приложение возобновлено");
        if let Err(e) = self.init_window(event_loop) {
            log::error!("Ошибка инициализации окна: {}", e);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = &mut self.window {
            match &event {
                WindowEvent::CloseRequested => {
                    if window_id == window.window.id() {
                        log::info!("Закрытие основного окна, выход");
                        std::process::exit(0);
                    } else {
                        let panel_type = window
                            .window_state
                            .detached_windows
                            .iter()
                            .find(|(_, detached)| detached.window.id() == window_id)
                            .map(|(ptype, _)| ptype.clone());
                        if let Some(ptype) = panel_type {
                            window.window_state.detached_windows.remove(&ptype);
                            log::info!("Закрыто откреплённое окно: {:?}", ptype);
                        }
                    }
                }
                WindowEvent::Resized(new_size) => {
                    if let Err(e) = window.resize(*new_size, window_id) {
                        log::error!("Ошибка изменения размера окна: {}", e);
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Err(e) = task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(window.render(event_loop))
                    }) {
                        log::error!("Ошибка рендеринга: {}", e);
                    }
                    return;
                }
                WindowEvent::KeyboardInput { event: key_event, .. } => {
                    log::info!("Ввод с клавиатуры: {:?}", &key_event.physical_key);
                }
                WindowEvent::Focused(focused) => {
                    window.set_focused(*focused);
                }
                _ => {}
            }
            if let Err(e) = window.handle_event(&event, window_id) {
                log::error!("Ошибка обработки события окна: {}", e);
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            if window.egui_ctx.has_requested_repaint() {
                window.window.request_redraw();
            }
            for detached in window.window_state.detached_windows.values() {
                if detached.visible && detached.egui_ctx.has_requested_repaint() {
                    detached.window.request_redraw();
                }
            }
        }
    }
}

pub fn run(event_loop: EventLoop<()>, ctx: EventLoopContext) -> Result<()> {
    let mut app = App::new(ctx);
    event_loop.run_app(&mut app)?;
    Ok(())
}

// === FILE: core\installer.rs ===
use anyhow::{Result, Context};
use dirs::home_dir;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use log::info;
pub struct Installer;
impl Installer {
    pub fn get_cosmonaut_dir() -> Result<PathBuf> {
        let home = home_dir()
            .ok_or_else(|| anyhow::anyhow!("Не удалось определить домашнюю директорию"))?;
        Ok(home.join(".cosmonaut"))
    }
    pub fn setup() -> Result<()> {
        let cosmonaut_dir = Self::get_cosmonaut_dir()?;
        let cache_dir = cosmonaut_dir.join("cache");
        let wasm_modules_dir = cosmonaut_dir.join("wasm_modules");
        let plugins_dir = cosmonaut_dir.join("plugins");
        let providers_dir = cosmonaut_dir.join("providers");
        let libs_dir = cosmonaut_dir.join("libs");
        let models_dir = cosmonaut_dir.join("models");
        let devtools_dir = cosmonaut_dir.join("devtools");
        let wasm_manifests_dir = cosmonaut_dir.join("wasm_manifests");
        let logs_dir = cosmonaut_dir.join("logs");
        let yuaidb_dir = cosmonaut_dir.join("yuaidb");
        let certs_ca_dir = cosmonaut_dir.join("certs").join("ca");
        let certs_user_dir = cosmonaut_dir.join("certs").join("user");
        let certs_revoked_dir = cosmonaut_dir.join("certs").join("revoked");
        let directories = vec![
            cosmonaut_dir.clone(),
            cache_dir,
            wasm_modules_dir,
            plugins_dir,
            providers_dir,
            libs_dir,
            models_dir,
            devtools_dir,
            wasm_manifests_dir,
            logs_dir,
            yuaidb_dir,
            certs_ca_dir,
            certs_user_dir,
            certs_revoked_dir,
        ];
        for dir in directories {
            create_dir_all(&dir)
                .with_context(|| format!("Не удалось создать директорию {}", dir.display()))?;
            info!("Создана директория: {}", dir.display());
        }
        let settings_path = cosmonaut_dir.join("settings.toml");
        if !settings_path.exists() {
            let default_settings = r#"
[modules]
core_enabled = true
ui_enabled = true
network_enabled = true
dom_enabled = false
js_enabled = false
wasmcloud_enabled = false
p2p_enabled = false
vdom_enabled = false
security_enabled = false
rendering_enabled = false
wasm_enabled = false
reactive_enabled = false
[settings]
cache_enabled = true
logging_level = "warn"
max_threads = 4
default_url = "https://yuai.ru"
font_family = "System Monospace"
graphics_backend = "auto"  # auto, vulkan, dx12, metal, gl
"#;
            let mut file = File::create(&settings_path)
                .with_context(|| format!("Не удалось создать файл {}", settings_path.display()))?;
            file.write_all(default_settings.as_bytes())
                .with_context(|| format!("Не удалось записать в файл {}", settings_path.display()))?;
            info!("Создан файл конфигурации: {}", settings_path.display());
        } else {
            info!("Файл конфигурации уже существует: {}", settings_path.display());
        }
        Ok(())
    }
}

// === FILE: core\interfaces.rs ===
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use std::path::Path;
use reqwest::header::HeaderMap;
use egui::Ui; // Необходимо для метода incremental_render
use serde_json::Value as JsonValue; // Необходимо для методов create_signals/stores
use crate::dom::parser::ParsedNode; // Необходимо для методов bind/apply/load

#[async_trait]
pub trait NetworkTrait: Send + Sync {
    async fn fetch_html(&self, url: &str) -> Result<String>;
    async fn fetch_html_with_headers(&self, url: &str) -> Result<(String, HeaderMap)>;
}
#[async_trait]
pub trait DomTrait: Send + Sync {
    async fn parse_html(&self, html: &str) -> Result<String>;
}
#[async_trait]
pub trait JsRuntimeTrait: Send + Sync {
    async fn execute_script(&self, script: &str) -> Result<()>;
}
#[async_trait]
pub trait OrchestratorTrait: Send + Sync {
    async fn deploy_actor(&self, actor_id: &str, wasm_bytes: &[u8]) -> Result<()>;
    async fn start_provider(&self, provider_id: &str) -> Result<()>;
}
#[async_trait]
pub trait PluginManagerTrait: Send + Sync {
    async fn new(plugins_dir: impl AsRef<Path> + Send, providers_dir: impl AsRef<Path> + Send) -> Result<Self> where Self: Sized;
}
#[async_trait]
pub trait LibManagerTrait: Send + Sync {
    async fn new(libs_dir: impl AsRef<Path> + Send) -> Result<Self> where Self: Sized;
    async fn load_library(&self, library_name: &str) -> Result<()>;
    async fn unload_library(&self, library_name: &str) -> Result<()>;
}
#[async_trait]
pub trait YuaidbTrait: Send + Sync {
    async fn new(db_dir: impl AsRef<Path> + Send) -> Result<Self> where Self: Sized;
    async fn insert(&self, key: &str, value: &str) -> Result<()>;
    async fn get(&self, key: &str) -> Result<Option<String>>;
    async fn delete(&self, key: &str) -> Result<()>;
}


#[async_trait]
pub trait SecurityManagerTrait: Send + Sync {
    async fn new(config: &super::config::Config) -> Result<Self> where Self: Sized;
    async fn configure_tls(&self) -> Result<Arc<rustls::ClientConfig>>;
    async fn sign_component(&self, data: &[u8]) -> Result<Vec<u8>>;
    async fn verify_component(&self, data: &[u8], signature: &[u8]) -> Result<()>;
}

#[async_trait]
pub trait ReactiveCoreTrait: Send + Sync {
    fn create_signals(&self, state: &JsonValue) -> Result<()>;
    fn bind_angular_scopes(&self, node: &ParsedNode) -> Result<()>;
    fn create_stores(&self, state: &JsonValue) -> Result<()>;
    fn apply_svelte_hashes(&self, node: &ParsedNode) -> Result<()>;
    fn load_wasm_module(&self, node: &ParsedNode) -> Result<()>;
    async fn incremental_render(&self, ui: &mut Ui, node: &ParsedNode) -> Result<()>;
}

#[async_trait]
pub trait SessionManagerTrait: Send + Sync {
    async fn new(db: Arc<dyn YuaidbTrait + Send + Sync>) -> Result<Self> where Self: Sized;
    async fn create_session(&self, r_id: &str) -> Result<String>;
    async fn get_session(&self, session_id: &str) -> Result<Option<String>>;
    async fn delete_session(&self, session_id: &str) -> Result<()>;
}
#[async_trait]
pub trait ProfileManagerTrait: Send + Sync {
    async fn new(certs_dir: impl AsRef<Path> + Send) -> Result<Self> where Self: Sized;
    async fn create_profile(&self, r_id: &str) -> Result<String>;
    async fn load_profile(&self, profile_id: &str) -> Result<Option<Vec<u8>>>;
    async fn delete_profile(&self, profile_id: &str) -> Result<()>;
}
#[async_trait]
pub trait CompositorTrait: Send + Sync {
    async fn new(window: &winit::window::Window, config: &()) -> Result<Self> where Self: Sized;
    async fn render_text(&self, text: &str);
    async fn resize(&self, width: u32, height: u32) -> Result<()>;
}
#[async_trait]
pub trait ServiceWorkerTrait: Send + Sync {
    async fn new(network: Arc<dyn NetworkTrait + Send + Sync>, db: Arc<dyn YuaidbTrait + Send + Sync>) -> Result<Self> where Self: Sized;
    async fn register(&self, script_url: &str) -> Result<()>;
    async fn cache_resource(&self, url: &str, data: &[u8]) -> Result<()>;
    async fn handle_push(&self, payload: &str) -> Result<()>;
}
#[async_trait]
pub trait SchedulerTrait: Send + Sync {
    async fn new(
        wasm_runtime: Option<Arc<dyn WasmRuntimeTrait + Send + Sync>>,
        js_runtime: Option<Arc<dyn JsRuntimeTrait + Send + Sync>>,
        orchestrator: Option<Arc<dyn OrchestratorTrait + Send + Sync>>,
    ) -> Result<Self> where Self: Sized;
    async fn schedule_wasm_task(&self, module: &[u8]) -> Result<()>;
    async fn schedule_js_task(&self, script: &str) -> Result<()>;
    async fn schedule_orchestration_task(&self, actor_id: &str) -> Result<()>;
}
#[async_trait]
pub trait IoManagerTrait: Send + Sync {
    async fn new(network: Arc<dyn NetworkTrait + Send + Sync>, db: Arc<dyn YuaidbTrait + Send + Sync>) -> Result<Self> where Self: Sized;
}
#[async_trait]
pub trait WasmRuntimeTrait: Send + Sync {
    async fn new(config: &super::config::Config) -> Result<Self> where Self: Sized;
}
#[async_trait]
pub trait WasmManifestTrait: Send + Sync {
    async fn new() -> Result<Self> where Self: Sized;
    async fn load_yaml(&self, path: &Path) -> Result<serde_yaml::Value>;
    async fn load_oci(&self, oci_ref: &str) -> Result<serde_yaml::Value>;
    async fn validate(&self, manifest: &serde_yaml::Value) -> Result<()>;
}


// === FILE: core\io_manager.rs ===
use crate::core::interfaces::{IoManagerTrait, NetworkTrait, YuaidbTrait};
use anyhow::Result;
use std::sync::Arc;
use async_trait::async_trait;
pub struct IoManager;
#[async_trait]
impl IoManagerTrait for IoManager {
    async fn new(
        _network: Arc<dyn NetworkTrait + Send + Sync>,
        _db: Arc<dyn YuaidbTrait + Send + Sync>
    ) -> Result<Self> {
        log::info!("Менеджер ввода-вывода инициализирован");
        Ok(Self)
    }
}

// === FILE: core\js_runtime.rs ===
use crate::core::interfaces::JsRuntimeTrait;
use anyhow::Result;
use async_trait::async_trait;
pub struct JsRuntime;
impl JsRuntime {
    pub fn new() -> Result<Self> {
        log::info!("JS runtime инициализирован");
        Ok(Self)
    }
}
#[async_trait]
impl JsRuntimeTrait for JsRuntime {
    async fn execute_script(&self, _script: &str) -> Result<()> {
        log::info!(
            "Выполнение JS-скрипта: {}...",
            &_script[.._script.len().min(50)]
        );
        Ok(())
    }
}

// === FILE: core\libs.rs ===
use crate::core::interfaces::LibManagerTrait;
use anyhow::Result;
use std::path::{Path, PathBuf};
use async_trait::async_trait;
pub struct LibManager {
    _libs_dir: PathBuf,
}
#[async_trait]
impl LibManagerTrait for LibManager {
    async fn new(libs_dir: impl AsRef<Path> + Send) -> Result<Self> {
        let libs_dir = libs_dir.as_ref().to_path_buf();
        log::info!("Менеджер библиотек инициализирован с каталогом: {:?}", libs_dir);
        Ok(Self { _libs_dir: libs_dir })
    }
    async fn load_library(&self, _library_name: &str) -> Result<()> {
        log::info!("Загрузка библиотеки: {}", _library_name);
        Ok(())
    }
    async fn unload_library(&self, _library_name: &str) -> Result<()> {
        log::info!("Выгрузка библиотеки: {}", _library_name);
        Ok(())
    }
}

// === FILE: core\mod.rs ===
pub mod engine;
pub mod event_loop;
pub mod scheduler;
pub mod runtime;
pub mod js_runtime;
pub mod orchestrator;
pub mod wasm_manifest;
pub mod provider;
pub mod plugins;
pub mod libs;
pub mod yuaidb;
pub mod config;
pub mod security;
pub mod reactive;
pub mod service_worker;
pub mod io_manager;
pub mod session;
pub mod profile;
pub mod installer;
pub mod interfaces;
pub mod ssh;
pub mod page_state;

// === FILE: core\orchestrator.rs ===
use crate::core::interfaces::OrchestratorTrait;
use anyhow::Result;
use async_trait::async_trait;
pub struct Orchestrator;
impl Orchestrator {
    pub fn new() -> Result<Self> {
        log::info!("Оркестратор инициализирован");
        Ok(Self)
    }
}
#[async_trait]
impl OrchestratorTrait for Orchestrator {
    async fn deploy_actor(&self, _actor_id: &str, _wasm_bytes: &[u8]) -> Result<()> {
        log::info!("Деплой актора: {}", _actor_id);
        Ok(())
    }
    async fn start_provider(&self, _provider_id: &str) -> Result<()> {
        log::info!("Запуск провайдера: {}", _provider_id);
        Ok(())
    }
}

// === FILE: core\page_state.rs ===
use std::sync::Arc;
use parking_lot::RwLock;
pub use crate::dom::parser::FrameworkType;
#[derive(Default, Clone)]
pub struct PageState {
    pub url: String,
    pub html: String,
    pub dom: String, // Пока как текстовое дерево → потом заменим на норм AST
    pub detected_framework: Option<FrameworkType>,
}

impl PageState {
    pub fn parse_framework_type(name: &str) -> Option<FrameworkType> {
        match name {
            "ReactNext" => Some(FrameworkType::ReactNext),
            "Angular" => Some(FrameworkType::Angular),
            "VueNuxt" => Some(FrameworkType::VueNuxt),
            "SvelteKit" => Some(FrameworkType::SvelteKit),
            "WasmModule" => Some(FrameworkType::WasmModule),
            _ => None,
        }
    }
}

pub type SharedPageState = Arc<RwLock<PageState>>;


// === FILE: core\plugins.rs ===
use crate::core::interfaces::PluginManagerTrait;
use anyhow::Result;
use std::path::Path;
use async_trait::async_trait;
pub struct PluginManager;
#[async_trait]
impl PluginManagerTrait for PluginManager {
    async fn new(
        _plugins_dir: impl AsRef<Path> + Send,
        _providers_dir: impl AsRef<Path> + Send,
    ) -> Result<Self> {
        log::info!("Менеджер плагинов инициализирован");
        Ok(Self)
    }
}

// === FILE: core\profile.rs ===
use crate::core::interfaces::ProfileManagerTrait;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use async_trait::async_trait;
pub struct ProfileManager {
    _certs_dir: PathBuf,
}
#[async_trait]
impl ProfileManagerTrait for ProfileManager {
    async fn new(certs_dir: impl AsRef<Path> + Send) -> Result<Self> {
        let certs_dir = certs_dir.as_ref().to_path_buf();
        if !certs_dir.exists() {
            fs::create_dir_all(&certs_dir)?;
        }
        log::info!("Менеджер профилей инициализирован с каталогом: {:?}", certs_dir);
        Ok(Self { _certs_dir: certs_dir })
    }
    async fn create_profile(&self, _r_id: &str) -> Result<String> {
        log::info!("Создание профиля для r_id: {}", _r_id);
        Ok("".to_string())
    }
    async fn load_profile(&self, _profile_id: &str) -> Result<Option<Vec<u8>>> {
        log::info!("Загрузка профиля: {}", _profile_id);
        Ok(None)
    }
    async fn delete_profile(&self, _profile_id: &str) -> Result<()> {
        log::info!("Удаление профиля: {}", _profile_id);
        Ok(())
    }
}

// === FILE: core\provider.rs ===
use std::sync::Arc;
pub struct Provider;
impl Provider {
    pub fn new() -> Arc<Self> {
        println!("Provider initialized");
        Arc::new(Self)
    }
}

// === FILE: core\reactive.rs ===
// === FILE: core\reactive.rs ===
use crate::dom::parser::ParsedNode;
// NOTE: egui::Ui требует, чтобы egui был в зависимостях
use egui::Ui; 
use serde_json::Value as JsonValue;
use anyhow::Result;
use async_trait::async_trait;
use log;

// ИСПРАВЛЕНИЕ: Импортируем официальный трейт из interfaces
use crate::core::interfaces::ReactiveCoreTrait; 

pub struct ReactiveCore;

// ИСПРАВЛЕННЫЙ БЛОК: Реализация new для СТРУКТУРЫ
impl ReactiveCore {
    // Конструктор структуры - это то, что вы вызываете в main.rs
    // Arc::new(ReactiveCore::new().await)
    pub async fn new() -> Self {
        log::info!("ReactiveCore initialized.");
        Self
    }
}

#[async_trait]
// Теперь ReactiveCore реализует трейт, импортированный из interfaces.rs
impl ReactiveCoreTrait for ReactiveCore {
    
    // Реализация методов трейта (ваши заглушки)
    fn create_signals(&self, _state: &JsonValue) -> Result<()> {
        Ok(())
    }
    fn bind_angular_scopes(&self, _node: &ParsedNode) -> Result<()> {
        Ok(())
    }
    fn create_stores(&self, _state: &JsonValue) -> Result<()> {
        Ok(())
    }
    fn apply_svelte_hashes(&self, _node: &ParsedNode) -> Result<()> {
        Ok(())
    }
    fn load_wasm_module(&self, _node: &ParsedNode) -> Result<()> {
        Ok(())
    }
    // Здесь мы должны реализовать только те методы, которые есть в трейте interfaces::ReactiveCoreTrait
    // NOTE: incremental_render, bind_angular_scopes и т.д. должны быть в трейте
    // Если их там нет, компилятор будет ругаться. 
    // Предполагаем, что они там есть.
    async fn incremental_render(&self, _ui: &mut Ui, _node: &ParsedNode) -> Result<()> {
        Ok(())
    }
}


// === FILE: core\runtime.rs ===
use crate::core::config::Config;
use crate::core::interfaces::WasmRuntimeTrait;
use anyhow::Result;
use async_trait::async_trait;
pub struct Runtime {
    #[cfg(feature = "wasm")]
    engine: wasmtime::Engine,
}
#[async_trait]
impl WasmRuntimeTrait for Runtime {
    async fn new(_config: &Config) -> Result<Self> {
        #[cfg(feature = "wasm")]
        {
            let mut wasmtime_config = wasmtime::Config::new();
            wasmtime_config.wasm_multi_memory(true);
            let engine = wasmtime::Engine::new(&wasmtime_config)?;
            log::info!("WASM runtime инициализирован с wasmtime");
            Ok(Self { engine })
        }
        #[cfg(not(feature = "wasm"))]
        {
            log::info!("WASM runtime отключён");
            Ok(Self {})
        }
    }
}

// === FILE: core\scheduler.rs ===
use crate::core::interfaces::{JsRuntimeTrait, OrchestratorTrait, SchedulerTrait, WasmRuntimeTrait};
use anyhow::Result;
use std::sync::Arc;
use async_trait::async_trait;
pub struct Scheduler {
    wasm_runtime: Option<Arc<dyn WasmRuntimeTrait + Send + Sync>>,
    js_runtime: Option<Arc<dyn JsRuntimeTrait + Send + Sync>>,
    orchestrator: Option<Arc<dyn OrchestratorTrait + Send + Sync>>,
}
#[async_trait]
impl SchedulerTrait for Scheduler {
    async fn new(
        wasm_runtime: Option<Arc<dyn WasmRuntimeTrait + Send + Sync>>,
        js_runtime: Option<Arc<dyn JsRuntimeTrait + Send + Sync>>,
        orchestrator: Option<Arc<dyn OrchestratorTrait + Send + Sync>>,
    ) -> Result<Self> {
        log::info!("Планировщик инициализирован");
        Ok(Self { wasm_runtime, js_runtime, orchestrator })
    }
    async fn schedule_wasm_task(&self, _module: &[u8]) -> Result<()> {
        if self.wasm_runtime.is_some() {
            log::info!("Планирование WASM-задачи");
            Ok(())
        } else {
            Err(anyhow::anyhow!("WASM runtime отключён"))
        }
    }
    async fn schedule_js_task(&self, _script: &str) -> Result<()> {
        if self.js_runtime.is_some() {
            log::info!("Планирование JS-задачи");
            Ok(())
        } else {
            Err(anyhow::anyhow!("JS runtime отключён"))
        }
    }
    async fn schedule_orchestration_task(&self, _actor_id: &str) -> Result<()> {
        if self.orchestrator.is_some() {
            log::info!("Планирование задачи оркестрации");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Оркестрация отключена"))
        }
    }
}

// === FILE: core\security.rs ===
use crate::core::config::Config;
use crate::core::interfaces::SecurityManagerTrait;
use anyhow::Result;
use std::sync::Arc;
use rustls::ClientConfig;
use async_trait::async_trait;
pub struct Security;
#[async_trait]
impl SecurityManagerTrait for Security {
    async fn new(_config: &Config) -> Result<Self> {
        log::info!("Менеджер безопасности инициализирован");
        Ok(Self)
    }
    async fn configure_tls(&self) -> Result<Arc<ClientConfig>> {
        log::info!("Настройка TLS");
        let root_store = rustls::RootCertStore::empty();
        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        Ok(Arc::new(config))
    }
    async fn sign_component(&self, _data: &[u8]) -> Result<Vec<u8>> {
        log::info!("Подпись компонента");
        Ok(vec![])
    }
    async fn verify_component(&self, _data: &[u8], _signature: &[u8]) -> Result<()> {
        log::info!("Проверка подписи компонента");
        Ok(())
    }
}

// === FILE: core\service_worker.rs ===
use crate::core::interfaces::{NetworkTrait, ServiceWorkerTrait, YuaidbTrait};
use anyhow::Result;
use std::sync::Arc;
use async_trait::async_trait;
pub struct ServiceWorker;
#[async_trait]
impl ServiceWorkerTrait for ServiceWorker {
    async fn new(
        _network: Arc<dyn NetworkTrait + Send + Sync>,
        _db: Arc<dyn YuaidbTrait + Send + Sync>
    ) -> Result<Self> {
        log::info!("Сервис-воркер инициализирован");
        Ok(Self)
    }
    async fn register(&self, _script_url: &str) -> Result<()> {
        log::info!("Регистрация сервис-воркера для URL: {}", _script_url);
        Ok(())
    }
    async fn cache_resource(&self, _url: &str, _data: &[u8]) -> Result<()> {
        log::info!("Кэширование ресурса: {}", _url);
        Ok(())
    }
    async fn handle_push(&self, _payload: &str) -> Result<()> {
        log::info!("Обработка push-уведомления: {}", _payload);
        Ok(())
    }
}

// === FILE: core\session.rs ===
use crate::core::interfaces::{SessionManagerTrait, YuaidbTrait};
use anyhow::Result;
use std::sync::Arc;
use async_trait::async_trait;
pub struct SessionManager;
#[async_trait]
impl SessionManagerTrait for SessionManager {
    async fn new(_db: Arc<dyn YuaidbTrait + Send + Sync>) -> Result<Self> {
        log::info!("Менеджер сессий инициализирован");
        Ok(Self)
    }
    async fn create_session(&self, _r_id: &str) -> Result<String> {
        log::info!("Создание сессии для r_id: {}", _r_id);
        Ok("".to_string())
    }
    async fn get_session(&self, _session_id: &str) -> Result<Option<String>> {
        log::info!("Получение сессии: {}", _session_id);
        Ok(None)
    }
    async fn delete_session(&self, _session_id: &str) -> Result<()> {
        log::info!("Удаление сессии: {}", _session_id);
        Ok(())
    }
}

// === FILE: core\ssh.rs ===
use ssh2::Session;
use std::net::TcpStream;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use tokio::sync::mpsc::UnboundedReceiver;
pub struct SshClient {
    session: Arc<Mutex<Session>>,
}
impl SshClient {
    pub fn connect(user: &str, host: &str) -> Result<Self> {
        let tcp = TcpStream::connect(host)?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_agent(user)?;
        Ok(Self {
            session: Arc::new(Mutex::new(session)),
        })
    }
    pub async fn run_command(&self, cmd: &str) -> Result<String> {
        let cmd = cmd.to_string(); // скопировали команду для 'static
        let session = self.session.clone();
        tokio::task::spawn_blocking(move || -> Result<String> {
            let sess = session.lock().unwrap();
            let mut channel = sess.channel_session()?;
            channel.exec(&cmd)?;
            let mut output = String::new();
            channel.read_to_string(&mut output)?;
            channel.wait_close()?;
            Ok(output)
        }).await?
    }
    pub async fn interactive_shell(&self, mut rx: UnboundedReceiver<String>) -> Result<String> {
        let session = self.session.clone();
        tokio::task::spawn_blocking(move || -> Result<String> {
            let sess = session.lock().unwrap();
            let mut channel = sess.channel_session()?;
            channel.request_pty("xterm", None, None)?;
            channel.shell()?;
            let mut buffer = Vec::new();
            let mut tmp = [0u8; 1024];
            loop {
                if let Ok(cmd) = rx.try_recv() {
                    channel.write_all(cmd.as_bytes())?;
                    channel.write_all(b"\n")?;
                }
                let n = channel.read(&mut tmp)?;
                if n == 0 { break; }
                buffer.extend_from_slice(&tmp[..n]);
            }
            channel.wait_close()?;
            Ok(String::from_utf8_lossy(&buffer).to_string())
        }).await?
    }
}

// === FILE: core\wasm_manifest.rs ===
use crate::core::interfaces::WasmManifestTrait;
use anyhow::Result;
use std::path::Path;
use async_trait::async_trait;
use serde_yaml::Value;
pub struct WasmManifest;
#[async_trait]
impl WasmManifestTrait for WasmManifest {
    async fn new() -> Result<Self> {
        log::info!("Менеджер WASM-манифестов инициализирован");
        Ok(Self)
    }
    async fn load_yaml(&self, _path: &Path) -> Result<Value> {
        log::info!("Загрузка WASM-манифеста из {:?}", _path);
        Ok(Value::Null)
    }
    async fn load_oci(&self, _oci_ref: &str) -> Result<Value> {
        log::info!("Загрузка WASM-манифеста из OCI: {}", _oci_ref);
        Ok(Value::Null)
    }
    async fn validate(&self, _manifest: &Value) -> Result<()> {
        log::info!("Валидация WASM-манифеста");
        Ok(())
    }
}

// === FILE: core\yuaidb.rs ===
use crate::core::interfaces::YuaidbTrait;
use anyhow::Result;
use std::path::{Path, PathBuf};
use async_trait::async_trait;
pub struct Yuaidb {
    _db_dir: PathBuf,
}
#[async_trait]
impl YuaidbTrait for Yuaidb {
    async fn new(db_dir: impl AsRef<Path> + Send) -> Result<Self> {
        let db_dir = db_dir.as_ref().to_path_buf();
        log::info!("База данных инициализирована с каталогом: {:?}", db_dir);
        Ok(Self { _db_dir: db_dir })
    }
    async fn insert(&self, _key: &str, _value: &str) -> Result<()> {
        log::info!("Вставка данных в db: {} -> {}", _key, _value);
        Ok(())
    }
    async fn get(&self, _key: &str) -> Result<Option<String>> {
        log::info!("Получение данных: {}", _key);
        Ok(None)
    }
    async fn delete(&self, _key: &str) -> Result<()> {
        log::info!("Удаление данных: {}", _key);
        Ok(())
    }
}

// === FILE: dom\css_parser.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct CSSparser;
impl CSSparser {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("CSSparser initialized");
        Ok(Self)
    }
}

// === FILE: dom\hydrate.rs ===
// === FILE: dom/hydrate.rs ===
use crate::dom::parser::{ParsedNode, FrameworkType};
use crate::dom::tree::DomRenderer;
use crate::core::interfaces::ReactiveCoreTrait;
use crate::core::reactive::ReactiveCore; 
use egui::Ui;
use anyhow::Result;
use std::sync::Arc;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Статические переменные для состояния гидрации и логов
static HYDRATION_STATE: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
static LOG_BUFFER: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

// Функция гидрации и рендеринга DOM
pub async fn hydrate_and_render(ui: &mut Ui, dom_renderer: &mut DomRenderer, mut node: ParsedNode) -> Result<Option<FrameworkType>> {
    // Очищаем буфер логов для текущего вызова
    let mut log_buffer = LOG_BUFFER.lock().unwrap();
    log_buffer.clear();

    // Переменная для хранения типа фреймворка
    let mut detected_framework: Option<FrameworkType> = None;

    // Проверяем, нужно ли выполнять гидрацию
    let should_hydrate = match &node {
        ParsedNode::Element { framework, attrs, .. } => {
            if let Some(fw) = framework {
                let mut state = HYDRATION_STATE.lock().unwrap();
                let node_key = format!("{:?}-{:?}", fw, attrs.get("id").unwrap_or(&String::new()));
                if state.contains(&node_key) {
                    false // Гидрация уже выполнена для этого узла
                } else {
                    state.push(node_key);
                    // Сохраняем фреймворк
                    detected_framework = Some(fw.clone());
                    // Добавляем лог о распознанном фреймворке
                    log_buffer.push(match fw {
                        FrameworkType::ReactNext => "Обнаружен React/Next.js, гидрация состояния".to_string(),
                        FrameworkType::Angular => "Обнаружен Angular, привязка компонентов".to_string(),
                        FrameworkType::VueNuxt => "Обнаружен Vue/Nuxt, создание stores".to_string(),
                        FrameworkType::SvelteKit => "Обнаружен SvelteKit, применение хешей".to_string(),
                        FrameworkType::WasmModule => "Обнаружен WASM-модуль, загрузка".to_string(),
                    });
                    true
                }
            } else {
                false
            }
        }
        _ => false,
    };

    // Выполняем гидрацию, если требуется
    if should_hydrate {
        let reactive_core = Arc::new(ReactiveCore);
        if let ParsedNode::Element { framework, reactive_state, .. } = &mut node {
            if let Some(fw) = framework {
                match fw {
                    FrameworkType::ReactNext => {
                        if let Some(state) = reactive_state {
                            reactive_core.create_signals(state)?;
                            log_buffer.push("Гидрация React завершена".to_string());
                        }
                    }
                    FrameworkType::Angular => {
                        reactive_core.bind_angular_scopes(&node)?;
                        log_buffer.push("Гидрация Angular завершена".to_string());
                    }
                    FrameworkType::VueNuxt => {
                        if let Some(state) = reactive_state {
                            reactive_core.create_stores(state)?;
                            log_buffer.push("Гидрация Vue завершена".to_string());
                        }
                    }
                    FrameworkType::SvelteKit => {
                        reactive_core.apply_svelte_hashes(&node)?;
                        log_buffer.push("Гидрация Svelte завершена".to_string());
                    }
                    FrameworkType::WasmModule => {
                        reactive_core.load_wasm_module(&node)?;
                        log_buffer.push("WASM-модуль загружен".to_string());
                    }
                }
                // Инкрементальный рендеринг без VDOM
                reactive_core.incremental_render(ui, &node).await?;
                log_buffer.push("Инкрементальный рендер без VDOM".to_string());
            }
        }
    }

    // Рендерим узел
    dom_renderer.render_node(ui, &node).await;

    // Выводим все накопленные логи
    for log in log_buffer.iter() {
        log::info!("{}", log);
    }

    Ok(detected_framework)
}

// === FILE: dom\layout.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Layout;
impl Layout {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Layout initialized");
        Ok(Self)
    }
}

// === FILE: dom\mod.rs ===
pub mod parser;
pub mod css_parser;
pub mod tree;
pub mod style;
pub mod hydrate;
pub mod vdom;
pub mod mutation;
pub mod layout;


// === FILE: dom\mutation.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Mutation;
impl Mutation {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Mutation initialized");
        Ok(Self)
    }
}

// === FILE: dom\parser.rs ===
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use anyhow::Result;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver}; 
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub enum ParsedNode {
    Text(String),
    Element {
        tag: String,
        attrs: HashMap<String, String>,
        children: Vec<ParsedNode>,
        reactive_state: Option<JsonValue>,
        framework: Option<FrameworkType>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum FrameworkType {
    ReactNext,
    Angular,
    VueNuxt,
    SvelteKit,
    WasmModule,
}

// Канал для передачи типа фреймворка
static FRAMEWORK_CHANNEL: Lazy<(UnboundedSender<FrameworkType>, Mutex<Option<UnboundedReceiver<FrameworkType>>>)> =
    Lazy::new(|| {
        let (tx, rx) = unbounded_channel();
        (tx, Mutex::new(Some(rx))) 
    });

pub fn get_framework_receiver() -> UnboundedReceiver<FrameworkType> {
    let mut lock = FRAMEWORK_CHANNEL.1.lock().expect("Ошибка блокировки Mutex");
    lock.take().expect("Приёмник фреймворка уже был извлечён")
}

pub fn parse_and_process(html: &str) -> Result<(ParsedNode, Option<FrameworkType>)> {
    let dom = parse_document(RcDom::default(), Default::default()).one(html);

    if let Some(body) = extract_body(&dom) {
        let mut node = process_node(body);
        let detected_framework = hydrate_framework_markers(&mut node, html)?;
        Ok((node, detected_framework)) // возвращаем FrameworkType наружу
    } else {
        Ok((ParsedNode::Text("".into()), None))
    }
}

fn extract_body(dom: &RcDom) -> Option<Handle> {
    fn find_body(handle: &Handle) -> Option<Handle> {
        match &handle.data {
            NodeData::Element { name, .. } if &*name.local == "body" => Some(handle.clone()),
            _ => handle
                .children
                .borrow()
                .iter()
                .find_map(|child| find_body(child)),
        }
    }
    find_body(&dom.document)
}

fn process_node(handle: Handle) -> ParsedNode {
    match &handle.data {
        NodeData::Text { contents } => {
            let text = contents.borrow().trim().to_string();
            ParsedNode::Text(text)
        }
        NodeData::Element { name, attrs, .. } => {
            let tag = name.local.to_lowercase();
            if tag == "script" || tag == "style" {
                return ParsedNode::Text("".into());
            }
            let mut attrs_map = HashMap::new();
            for attr in attrs.borrow().iter() {
                attrs_map.insert(attr.name.local.to_string(), attr.value.to_string());
            }
            let children = handle
                .children
                .borrow()
                .iter()
                .map(|child| process_node(child.clone()))
                .filter(|node| match node {
                    ParsedNode::Text(s) => !s.is_empty(),
                    _ => true,
                })
                .collect::<Vec<_>>();
            ParsedNode::Element {
                tag,
                attrs: attrs_map,
                children,
                reactive_state: None,
                framework: None,
            }
        }
        _ => ParsedNode::Text("".into()),
    }
}

fn hydrate_framework_markers(node: &mut ParsedNode, html: &str) -> Result<Option<FrameworkType>> {
    if let ParsedNode::Element {
        attrs,
        reactive_state,
        framework,
        children,
        ..
    } = node
    {
        log::info!("Проверка маркеров фреймворка в элементе с атрибутами: {:?}", attrs);

        // --- ЛОГИКА ОБНАРУЖЕНИЯ ---
        let detected_fw = if html.contains(r#"id="__NEXT_DATA__""#) {
            // Извлечение состояния и обнаружение
            if let Some(json_str) = extract_script_content(html, "__NEXT_DATA__") {
                *reactive_state = Some(serde_json::from_str(&json_str)?);
                log::info!("Обнаружен React/Next.js");
                Some(FrameworkType::ReactNext)
            } else {
                log::info!("Маркер React/Next.js найден, но состояние не извлечено");
                None
            }
        } else if attrs.contains_key("ng-version") {
            log::info!("Обнаружен Angular");
            Some(FrameworkType::Angular)
        } else if html.contains("window.__NUXT__") {
            // Извлечение состояния и обнаружение
            if let Some(json_str) = extract_script_content(html, "__NUXT__") {
                *reactive_state = Some(serde_json::from_str(&json_str)?);
                log::info!("Обнаружен Vue/Nuxt");
                Some(FrameworkType::VueNuxt)
            } else {
                log::info!("Маркер Vue/Nuxt найден, но состояние не извлечено");
                None
            }
        } else if html.contains(r#"data-sveltekit-fetched"#) || attrs.contains_key("data-h") {
            // Извлечение состояния и обнаружение
            if let Some(json_str) = extract_script_content(html, "data-sveltekit-fetched") {
                *reactive_state = Some(serde_json::from_str(&json_str)?);
                log::info!("Обнаружен SvelteKit");
                Some(FrameworkType::SvelteKit)
            } else {
                log::info!("Маркер SvelteKit найден, но состояние не извлечено");
                None
            }
        } else if html.contains(".wasm") || html.contains(r#"<script type="module">"#) {
            log::info!("Обнаружен WASM-модуль");
            Some(FrameworkType::WasmModule)
        } else {
            None
        };

        // Отправка обнаруженного фреймворка в канал
        if let Some(fw) = &detected_fw {
            if let Err(e) = FRAMEWORK_CHANNEL.0.send(fw.clone()) {
                log::error!("Ошибка отправки типа фреймворка в канал: {}", e);
            } else {
                log::info!("Тип фреймворка отправлен в канал: {:?}", fw);
            }
        }

        // --- КОНЕЦ ЛОГИКИ ОБНАРУЖЕНИЯ ---

        // 1. Сохраняем обнаруженный фреймворк в текущем узле (для DOM)
        *framework = detected_fw.clone();

        // 2. Если фреймворк обнаружен на этом уровне, возвращаем его
        if detected_fw.is_some() {
            return Ok(detected_fw);
        }

        // 3. Рекурсия по детям, если фреймворк еще не найден
        for child in children.iter_mut() {
            if let Some(fw) = hydrate_framework_markers(child, html)? {
                // Возвращаем первый найденный фреймворк и прекращаем поиск
                return Ok(Some(fw));
            }
        }
    }
    // Если ничего не найдено в этом узле или его детях, возвращаем None
    Ok(None)
}

fn extract_script_content(html: &str, pattern: &str) -> Option<String> {
    if let Some(start) = html.find(&format!(r#"<script id="{}" type="application/json">"#, pattern)) {
        let content_start = start + format!(r#"<script id="{}" type="application/json">"#, pattern).len();
        if let Some(end) = html[content_start..].find("</script>") {
            return Some(html[content_start..content_start + end].trim().to_string());
        }
    } else if let Some(start) = html.find(pattern) { // Фолбек для window.__NUXT__ или data-sveltekit-fetched
        // Это грубая эвристика и может требовать уточнения в зависимости от точной разметки,
        // но она пытается захватить блоб JSON из присвоения переменной JavaScript.
        let end = html[start..].find("</script>").unwrap_or(html.len());
        let script_content = &html[start..start + end];

        // Грубое извлечение JSON-подобного блоба из присвоения переменной JavaScript
        if let Some(json_start) = script_content.find('{') {
            let potential_json = &script_content[json_start..];
            let mut open_brackets = 0;
            let mut json_end = 0;
            for (i, c) in potential_json.chars().enumerate() {
                if c == '{' {
                    open_brackets += 1;
                } else if c == '}' {
                    open_brackets -= 1;
                    if open_brackets == 0 {
                        json_end = i + 1;
                        break;
                    }
                }
            }
            if json_end > 0 {
                return Some(potential_json[..json_end].to_string());
            }
        }
    }
    None
}

pub fn collect_text(node: &ParsedNode) -> String {
    match node {
        ParsedNode::Text(text) => text.clone(),
        ParsedNode::Element { children, .. } => {
            let mut text = String::new();
            for child in children {
                let child_text = collect_text(child);
                if !child_text.is_empty() {
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(&child_text);
                }
            }
            text
        }
    }
}

// === FILE: dom\style.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Style;
impl Style {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Style initialized");
        Ok(Self)
    }
}

// === FILE: dom\tree.rs ===
use crate::dom::parser::{ParsedNode, collect_text};
use egui::{Ui, RichText, TextStyle, Label, ColorImage, TextureHandle};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use image::ImageFormat;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use reqwest::Client;
use std::collections::{HashMap, VecDeque};

static IMAGE_CACHE: Lazy<Mutex<HashMap<String, Arc<TextureHandle>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct DomRenderer {
    pub clicked_links: VecDeque<String>,
    pub clicked_buttons: VecDeque<String>,
    image_tx: UnboundedSender<(String, Result<image::DynamicImage>)>,
    image_rx: UnboundedReceiver<(String, Result<image::DynamicImage>)>,
    egui_ctx: Option<egui::Context>,
    needs_repaint: bool, // Флаг необходимости перерисовки
}

impl DomRenderer {
    pub fn new() -> Self {
        let (image_tx, image_rx) = tokio::sync::mpsc::unbounded_channel();
        log::info!("Создание DomRenderer");
        Self {
            clicked_links: VecDeque::new(),
            clicked_buttons: VecDeque::new(),
            image_tx,
            image_rx,
            egui_ctx: None,
            needs_repaint: true, // Изначально требуется рендер
        }
    }

    pub fn set_egui_context(&mut self, ctx: egui::Context) {
        log::info!("Установка контекста egui для DomRenderer");
        self.egui_ctx = Some(ctx);
    }

    pub fn needs_repaint(&self) -> bool {
        self.needs_repaint
    }

    pub fn clear_repaint_flag(&mut self) {
        log::info!("Сброс флага перерисовки в DomRenderer");
        self.needs_repaint = false;
    }

    pub fn process_image_responses(&mut self, ui: &mut Ui) {
        let mut image_processed = false;
        while let Ok((src, result)) = self.image_rx.try_recv() {
            log::info!("Обработка ответа для изображения: {}", src);
            let mut cache = IMAGE_CACHE.lock().unwrap();
            match result {
                Ok(img) => {
                    let rgba = img.to_rgba8();
                    let size = [rgba.width() as usize, rgba.height() as usize];
                    let color_image = ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
                    if let Some(ctx) = &self.egui_ctx {
                        let tex = ctx.load_texture(src.clone(), color_image, Default::default());
                        cache.insert(src.clone(), Arc::new(tex));
                        image_processed = true;
                        self.needs_repaint = true; // Изображение загружено, нужна перерисовка
                        log::info!("Изображение {} успешно загружено и добавлено в кэш", src);
                    }
                }
                Err(e) => {
                    log::warn!("Ошибка загрузки изображения {}: {}", src, e);
                    cache.insert(src.clone(), Arc::new(ui.ctx().load_texture(
                        src.clone(),
                        ColorImage::new([1, 1], vec![egui::Color32::GRAY]),
                        Default::default(),
                    )));
                }
            }
        }
        if image_processed && self.egui_ctx.as_ref().map_or(false, |ctx| ctx.has_requested_repaint()) {
            log::info!("DomRenderer запросил перерисовку из-за новых изображений");
            self.needs_repaint = true;
        }
    }

    pub async fn render_node(&mut self, ui: &mut Ui, node: &ParsedNode) {
        match node {
            ParsedNode::Text(text) => {
                if !text.is_empty() {
                    //log::info!("Рендер текста: {} (длина: {})", text, text.len());
                    ui.label(RichText::new(text));
                }
            }
            ParsedNode::Element { tag, attrs, children, .. } => {
                //log::info!("Рендер элемента: {}", tag);
                match tag.as_str() {
                    "a" => {
                        if let Some(href) = attrs.get("href") {
                            let link_text = collect_text(node);
                            //log::info!("Рендер ссылки: {} ({})", link_text, href);
                            let response = ui.link(link_text).clicked();
                            if response {
                                self.clicked_links.push_back(href.clone());
                                self.needs_repaint = true; // Клик требует перерисовки
                                log::info!("Клик по ссылке: {}, добавлено в очередь", href);
                            }
                        }
                    }
                    "button" => {
                        let text = collect_text(node);
                        //log::info!("Рендер кнопки: {}", text);
                        let response = ui.button(&text).clicked();
                        if response {
                            self.clicked_buttons.push_back("button".into());
                            self.needs_repaint = true; // Клик требует перерисовки
                            log::info!("Клик по кнопке: {}, добавлено в очередь", text);
                        }
                    }
                    "input" => {
                        log::info!("Рендер input: {:?}", attrs);
                        self.render_input(ui, attrs);
                    }
                    "img" => {
                        if let Some(src) = attrs.get("src") {
                            //log::info!("Рендер изображения: {}", src);
                            let mut cache = IMAGE_CACHE.lock().unwrap();
                            if let Some(tex) = cache.get(src) {
                                ui.image(&**tex);
                            } else {
                                ui.label("[Загрузка изображения...]");
                                let tx = self.image_tx.clone();
                                tokio::spawn(Self::load_image_async(src.clone(), tx));
                                cache.insert(src.clone(), Arc::new(ui.ctx().load_texture(
                                    src.clone(),
                                    ColorImage::new([1, 1], vec![egui::Color32::GRAY]),
                                    Default::default(),
                                )));
                                log::info!("Запущена асинхронная загрузка изображения: {}", src);
                                self.needs_repaint = true; // Загрузка изображения требует перерисовки
                            }
                        }
                    }
                    "span" => {
                        //log::info!("Рендер span");
                        let _ = ui.horizontal_wrapped(|ui| {
                            for child in children {
                                pollster::block_on(self.render_node(ui, child));
                            }
                        });
                    }
                    "ul" | "ol" => {
                        //log::info!("Рендер списка: {}", tag);
                        for child in children {
                            pollster::block_on(self.render_node(ui, child));
                        }
                    }
                    "li" => {
                        //log::info!("Рендер элемента списка");
                        let _ = ui.horizontal(|ui| {
                            ui.label("•");
                            for child in children {
                                pollster::block_on(self.render_node(ui, child));
                            }
                        });
                    }
                    "details" => {
                        //log::info!("Рендер details");
                        let summary = children
                            .iter()
                            .find(|c| matches!(c, ParsedNode::Element { tag, .. } if tag == "summary"))
                            .map(|c| collect_text(c))
                            .unwrap_or_else(|| "Подробности".into());
                        let _ = ui.collapsing(summary, |ui| {
                            for child in children {
                                if let ParsedNode::Element { tag, .. } = child {
                                    if tag != "summary" {
                                        pollster::block_on(self.render_node(ui, child));
                                    }
                                }
                            }
                        });
                        self.needs_repaint = true; // Изменение состояния details требует перерисовки
                    }
                    "br" => {
                        //log::info!("Рендер br");
                        ui.end_row();
                    }
                    "hr" => {
                        //log::info!("Рендер hr");
                        let _ = ui.separator();
                    }
                    "p" | "div" | "section" => {
                        //log::info!("Рендер контейнера: {}", tag);
                        let _ = ui.group(|ui| {
                            for child in children {
                                pollster::block_on(self.render_node(ui, child));
                            }
                        });
                    }
                    "pre" => {
                        //log::info!("Рендер pre");
                        let text = collect_text(node);
                        ui.add(
                            Label::new(
                                RichText::new(text)
                                    .text_style(TextStyle::Monospace)
                            )
                            .wrap()
                        );
                    }
                    _ => {
                        //log::info!("Рендер прочих элементов: {}", tag);
                        for child in children {
                            pollster::block_on(self.render_node(ui, child));
                        }
                    }
                }
            }
        }
    }

    async fn load_image_async(src: String, tx: UnboundedSender<(String, Result<image::DynamicImage>)>) {
        log::info!("Запуск асинхронной загрузки изображения: {}", src);
        let client = Client::new();
        let result = async {
            let resp = client.get(&src).send().await?;
            let bytes = resp.bytes().await?;
            let img = image::load_from_memory_with_format(&bytes, ImageFormat::from_path(&src)?)?;
            Ok(img)
        }.await;
        let _ = tx.send((src.clone(), result));
        log::info!("Завершена асинхронная загрузка изображения: {}", src);
    }

    fn render_input(&mut self, ui: &mut Ui, attrs: &HashMap<String, String>) {
        //log::info!("Рендер поля ввода: {:?}", attrs);
        let input_type = attrs
            .get("type")
            .map(|t| t.to_lowercase())
            .unwrap_or_else(|| "text".into());
        match input_type.as_str() {
            "text" | "email" | "password" | "number" => {
                let mut value = attrs.get("value").cloned().unwrap_or_default();
                let placeholder = attrs.get("placeholder").cloned().unwrap_or_default();
                let edit = ui.add(
                    egui::TextEdit::singleline(&mut value)
                        .hint_text(placeholder)
                        .desired_width(150.0),
                );
                if edit.changed() {
                    self.clicked_buttons.push_back(format!("input_changed:{}", value));
                    self.needs_repaint = true; // Изменение ввода требует перерисовки
                    log::info!("Введено значение в поле ввода: {}", value);
                }
            }
            "checkbox" => {
                let checked = attrs.contains_key("checked");
                let mut val = checked;
                if ui.checkbox(&mut val, "").clicked() {
                    self.clicked_buttons.push_back(format!("checkbox_changed:{}", val));
                    self.needs_repaint = true; // Изменение чекбокса требует перерисовки
                    log::info!("Чекбокс изменён: {}", val);
                }
            }
            "radio" => {
                let selected = attrs.contains_key("checked");
                let val = selected;
                if ui.radio(val, "").clicked() {
                    self.clicked_buttons.push_back(format!("radio_selected:{}", val));
                    self.needs_repaint = true; // Изменение радиокнопки требует перерисовки
                    log::info!("Радиокнопка выбрана: {}", val);
                }
            }
            _ => {
                log::warn!("Неподдерживаемый тип ввода: {}", input_type);
                ui.label(format!("[Неподдерживаемый тип ввода: {}]", input_type));
            }
        }
    }

    pub fn get_last_link_click(&mut self) -> Option<String> {
        log::info!("Получение последнего клика по ссылке");
        self.clicked_links.pop_back()
    }

    pub fn get_last_button_click(&mut self) -> Option<String> {
        log::info!("Получение последнего клика по кнопке");
        self.clicked_buttons.pop_back()
    }
}

// === FILE: dom\vdom.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct VDOM;
impl VDOM {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("VDOM initialized");
        Ok(Self)
    }
}

// === FILE: net\cache.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Cache;
impl Cache {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Cache initialized");
        Ok(Self)
    }
}

// === FILE: net\fetch.rs ===
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use reqwest::header::HeaderMap;

use crate::core::config::Config;
use crate::core::engine::{BroEngine, EngineEvent, UrlResponse};
use crate::core::interfaces::{NetworkTrait, YuaidbTrait};
use async_trait::async_trait;

pub struct Network {
    client: reqwest::Client,
    db: Option<Arc<dyn YuaidbTrait + Send + Sync>>,
}

impl Network {
    pub fn new(config: &Config, db: Option<Arc<dyn YuaidbTrait + Send + Sync>>) -> Result<Self> {
        let client_builder = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10));

        let client = client_builder.build()?;
        log::info!("Сетевой модуль инициализирован");
        Ok(Self { client, db })
    }
}

#[async_trait]
impl NetworkTrait for Network {
    async fn fetch_html(&self, url: &str) -> Result<String> {
        let start = Instant::now();
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context(format!("Не удалось выполнить запрос к {}", url))?;

        let status = response.status().as_u16();
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Ошибка HTTP: {}", status));
        }

        let html = response
            .text()
            .await
            .context(format!("Не удалось декодировать ответ от {}", url))?;

        log::info!("Успешно загружен HTML для {} за {:?}", url, start.elapsed());
        Ok(html)
    }

    async fn fetch_html_with_headers(&self, url: &str) -> Result<(String, HeaderMap)> {
        let start = Instant::now();
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context(format!("Не удалось выполнить запрос к {}", url))?;

        let status = response.status().as_u16();
        let headers = response.headers().clone();
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Ошибка HTTP: {}", status));
        }

        let html = response
            .text()
            .await
            .context(format!("Не удалось декодировать ответ от {}", url))?;

        log::info!("Успешно загружен HTML с заголовками для {} за {:?}", url, start.elapsed());
        Ok((html, headers))
    }
}

// Асинхронный сетевой сервис для обработки событий движка
pub async fn run_network_service(network: Arc<dyn NetworkTrait + Send + Sync>, engine: Arc<BroEngine>) {
    log::info!("Запуск сетевого сервиса");
    loop {
        // Неблокирующий приём событий
        match engine.receive_event() {
            Some(EngineEvent::UrlRequest { url, response_tx }) => {
                let url_clone = url.clone();
                log::info!("Обработка сетевого запроса для URL: {}", url_clone);
                let network_clone = network.clone();
                let engine_clone = engine.clone();
                // Запускаем обработку запроса в отдельной задаче
                tokio::spawn(async move {
                    let start = Instant::now();
                    match network_clone.fetch_html_with_headers(&url_clone).await {
                        Ok((html, headers)) => {
                            let response = UrlResponse {
                                url: url_clone.clone(),
                                html,
                                headers,
                                status: 200,
                                duration: start.elapsed(),
                            };
                            if let Err(e) = response_tx.send(response) {
                                log::error!("Ошибка отправки ответа для URL {}: {}", url_clone, e);
                                engine_clone.send_event(EngineEvent::Error(format!(
                                    "Ошибка отправки ответа: {}",
                                    e
                                )));
                            } else {
                                log::info!("Ответ отправлен для URL: {}", url_clone);
                            }
                        }
                        Err(e) => {
                            log::error!("Ошибка загрузки URL {}: {}", url_clone, e);
                            engine_clone.send_event(EngineEvent::Error(format!(
                                "Ошибка загрузки {}: {}",
                                url_clone, e
                            )));
                        }
                    }
                });
            }
            Some(event) => {
                log::debug!("Пропуск события: {:?}", event);
            }
            None => {
                // Небольшая задержка для предотвращения активного опроса
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        }
    }
}

// Функция загрузки URL с использованием движка
pub async fn load_url_generic(url: String, engine: &Arc<BroEngine>) -> Result<(String, HeaderMap, RequestLog)> {
    let response = engine.fetch_url(url.clone()).await?;
    let log = RequestLog {
        url: url.clone(),
        method: "GET".to_string(),
        status: response.status,
        headers: response.headers.clone(),
        duration: response.duration,
    };
    Ok((response.html, response.headers, log))
}

pub struct RequestLog {
    pub url: String,
    pub method: String,
    pub status: u16,
    pub headers: HeaderMap,
    pub duration: Duration,
}

// === FILE: net\ipfs.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct IPFS;
impl IPFS {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("IPFS initialized");
        Ok(Self)
    }
}

// === FILE: net\load_url.rs ===
use crate::core::engine::{BroEngine, UrlResponse};
use crate::ui::devtools::RequestLog;
use reqwest::header::HeaderMap;
use anyhow::Result;
use std::sync::Arc;
use std::time::{Instant, Duration};

pub async fn load_url_generic(
    url: String,
    engine: &Arc<BroEngine>,
) -> Result<(String, HeaderMap, RequestLog)> {
    let start_time = Instant::now();
    
    // Выполняем сетевой запрос через BroEngine
    match engine.fetch_url(url.clone()).await {
        Ok(response) => {
            let headers = response.headers.clone();
            let log = RequestLog {
                url: url.clone(),
                method: "GET".into(),
                status: response.status,
                headers: headers.clone(),
                duration: start_time.elapsed(),
                html: response.html.clone(),
            };
            Ok((response.html, headers, log))
        }
        Err(e) => {
            // Если сеть отключена или произошла ошибка
            let html = format!(
                "<html><body><h1>Ошибка загрузки {}</h1><p>{}</p></body></html>",
                url, e
            );
            let headers = HeaderMap::new();
            let log = RequestLog {
                url,
                method: "GET".into(),
                status: 0,
                headers: headers.clone(),
                duration: Duration::from_millis(0),
                html: html.clone(),
            };
            Ok((html, headers, log))
        }
    }
}

// === FILE: net\mod.rs ===
pub mod fetch;
pub mod webrtc;
pub mod ipfs;
pub mod websocket;
pub mod proto;
pub mod tls;
pub mod p2p;
pub mod cache;
pub mod stream;
pub mod load_url;
pub use fetch::Network;


// === FILE: net\p2p.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct P2P;
impl P2P {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("P2P initialized");
        Ok(Self)
    }
}

// === FILE: net\proto.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Proto;
impl Proto {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Proto initialized");
        Ok(Self)
    }
}

// === FILE: net\stream.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Stream;
impl Stream {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Stream initialized");
        Ok(Self)
    }
}

// === FILE: net\tls.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct TLS;
impl TLS {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("TLS initialized");
        Ok(Self)
    }
}

// === FILE: net\webrtc.rs ===
use anyhow::Result;
use crate::core::config::Config;
pub struct WebRTC;
impl WebRTC {
    pub fn new(_config: &Config) -> Result<Self> {
        println!("WebRTC initialized");
        Ok(Self)
    }
    pub fn create_peer_connection(&self, _peer_id: &str) -> Result<()> {
        println!("Creating WebRTC peer connection for peer: {}", _peer_id);
        Ok(())
    }
    pub fn send_data(&self, _peer_id: &str, _data: &[u8]) -> Result<()> {
        println!("Sending data to peer: {}", _peer_id);
        Ok(())
    }
    pub fn close_connection(&self, _peer_id: &str) -> Result<()> {
        println!("Closing WebRTC connection for peer: {}", _peer_id);
        Ok(())
    }
}

// === FILE: net\websocket.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Websocket;
impl Websocket {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Websocket initialized");
        Ok(Self)
    }
}

// === FILE: ui\aichat.rs ===
use egui::{Context as EguiContext, SidePanel, ScrollArea, TextEdit, Ui};
use crate::ui::devtools::PanelAction; // Импорт из devtools, так как общий
pub fn render_aichat(
    ctx: &EguiContext,
    ai_chat_input: &mut String,
    ai_chat_history: &[(String, String)],
    send_ai_message: &mut bool,
) {
    SidePanel::right("ai_chat_panel")
        .resizable(true)
        .default_width(400.0)
        .min_width(200.0)
        .show(ctx, |ui| {
            render_aichat_ui(ui, ai_chat_input, ai_chat_history, send_ai_message, false)
        });
}
pub fn render_aichat_ui(
    ui: &mut Ui,
    ai_chat_input: &mut String,
    ai_chat_history: &[(String, String)],
    send_ai_message: &mut bool,
    is_detached: bool,
) -> Option<PanelAction> {
    let mut action: Option<PanelAction> = None;
    ui.horizontal(|ui| {
		if ui.button(if is_detached { "🗔" } else { "🗔" }).clicked() {
            action = Some(if is_detached { PanelAction::Attach } else { PanelAction::Detach });
        }
		ui.heading("Чат с AI (F11)");
    });
    ScrollArea::vertical().show(ui, |ui| {
        ui.label("История чата (заглушка):");
        for (user_msg, ai_response) in ai_chat_history {
            ui.label(format!("Вы: {}", user_msg));
            ui.label(format!("AI: {}", ai_response));
            ui.add_space(5.0);
        }
    });
    ui.add_space(10.0);
    ui.label("Ваш запрос:");
    ui.add(TextEdit::multiline(ai_chat_input).desired_rows(3).min_size(egui::vec2(100.0, 50.0)));
    if ui.button("Отправить").clicked() {
        *send_ai_message = true;
    }
    action
}

// === FILE: ui\clipboard.rs ===
use anyhow::Result;
pub struct Clipboard;
impl Clipboard {
    pub fn new() -> Result<Self> {
        println!("Clipboard initialized");
        Ok(Self)
    }
}

// === FILE: ui\compositor.rs ===
use crate::core::interfaces::CompositorTrait;
use anyhow::Result;
use winit::window::Window;
use async_trait::async_trait;
pub struct Compositor;
#[async_trait]
impl CompositorTrait for Compositor {
    async fn new(_window: &Window, _config: &()) -> Result<Self> {
        log::info!("Компоновщик инициализирован");
        Ok(Self)
    }
    async fn render_text(&self, _text: &str) {
        log::info!("Отрисовка текста: {}", _text);
    }
    async fn resize(&self, _width: u32, _height: u32) -> Result<()> {
        log::info!("Изменение размера: {}x{}", _width, _height);
        Ok(())
    }
}

// === FILE: ui\devtools.rs ===
use crate::core::engine::{EngineEvent, BroEngine, UrlResponse};
use crate::core::page_state::FrameworkType;
use egui::{Context as EguiContext, SidePanel, ScrollArea, TextEdit, Ui, TextStyle, Id};
use std::time::{Duration, Instant};
use reqwest::header::HeaderMap;
use crate::ui::heavy_calculator::HeavyCalculator;
use tokio::sync::broadcast;
use tokio::sync::mpsc::UnboundedSender;
use sysinfo::System;
use std::sync::Arc;

#[derive(PartialEq, Clone, Copy)]
pub enum DevToolsTab {
    Html,
    Network,
    Debug,
}

#[derive(Clone)]
pub struct RequestLog {
    pub url: String,
    pub method: String,
    pub status: u16,
    pub headers: HeaderMap,
    pub duration: Duration,
    pub html: String,
}

pub struct DevToolsState {
    pub engine: Arc<BroEngine>,
    pub detected_framework: Option<FrameworkType>,
    pub network_logs: Vec<RequestLog>,
    pub debug_info: DebugInfo,
    pub rx_engine: broadcast::Receiver<EngineEvent>,
	
}

impl DevToolsState {
    pub fn poll_events(&mut self) {
        while let Ok(event) = self.rx_engine.try_recv() {
            match event {
                EngineEvent::FrameworkDetected(fw) => {
                    self.detected_framework = Some(fw);
                }
                EngineEvent::UrlResponse(response) => {
                    if !self.network_logs.iter().any(|log| log.url == response.url && log.duration == response.duration) {
                        self.network_logs.push(RequestLog {
                            url: response.url,
                            method: "GET".to_string(),
                            status: response.status,
                            headers: response.headers,
                            duration: response.duration,
                            html: response.html,
                        });
                    }
                }
                EngineEvent::Error(err) => {
                    self.network_logs.push(RequestLog {
                        url: "Ошибка".to_string(),
                        method: "N/A".to_string(),
                        status: 0,
                        headers: HeaderMap::new(),
                        duration: Duration::ZERO,
                        html: err,
                    });
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct DebugInfo {
    pub data_received: usize,
    pub data_size: String,
    pub transfer_speed: f64,
    pub render_time: Duration,
    pub server_response_time: Duration,
    pub dom_elements: usize,
    pub memory_usage: String,
    pub frame_rate: f64,
    pub last_frame_time: Option<Instant>,
    pub frame_count: u32,
    pub fps_update_time: Option<Instant>,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            last_frame_time: Some(Instant::now()),
            fps_update_time: Some(Instant::now()),
            ..Default::default()
        }
    }

    pub fn update(&mut self, html: &str, render_time: Duration, response_time: Duration) {
        self.data_received = html.len();
        self.data_size = Self::format_size(html.len());
        self.transfer_speed = if response_time.as_secs_f64() > 0.0 {
            html.len() as f64 / response_time.as_secs_f64() / 1024.0
        } else {
            0.0
        };
        self.render_time = render_time;
        self.server_response_time = response_time;
        self.dom_elements = html.matches("<div").count() + html.matches("<p").count() + html.matches("<span").count();
        let mut sys = System::new_all();
        sys.refresh_all();
        self.memory_usage = Self::format_size(sys.used_memory() as usize);
        let now = Instant::now();
        if let Some(last) = self.last_frame_time {
            let delta = now - last;
            if delta > Duration::from_millis(0) {
                self.frame_rate = 1.0 / delta.as_secs_f64();
            }
        }
        self.last_frame_time = Some(now);
        self.frame_count += 1;
        if let Some(update_time) = self.fps_update_time {
            if now - update_time > Duration::from_secs(1) {
                self.frame_rate = self.frame_count as f64 / (now - update_time).as_secs_f64();
                self.frame_count = 0;
                self.fps_update_time = Some(now);
            }
        }
    }

    pub fn format_size(bytes: usize) -> String {
        if bytes >= 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else if bytes >= 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else {
            format!("{} B", bytes)
        }
    }

    pub fn format_duration(duration: Duration) -> String {
        if duration.as_secs() > 0 {
            format!("{:.2} s", duration.as_secs_f64())
        } else if duration.as_millis() > 0 {
            format!("{} ms", duration.as_millis())
        } else {
            format!("{} μs", duration.as_micros())
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum PanelAction {
    Attach,
    Detach,
}

pub fn render_devtools(
    ctx: &EguiContext,
    devtools_tab: &mut DevToolsTab,
    html: &mut String,
    headers: &HeaderMap,
    debug_info: &DebugInfo,
    network_logs: &Vec<RequestLog>,
    tx_ssh: &UnboundedSender<String>,
    devtools_state: &DevToolsState,
) {
    SidePanel::right("devtools_panel")
        .resizable(true)
        .default_width(400.0)
        .min_width(200.0)
        .show(ctx, |ui| {
            render_devtools_ui(ui, devtools_tab, html, headers, debug_info, network_logs, tx_ssh, false, devtools_state);
        });
}

pub fn render_devtools_ui(
    ui: &mut Ui,
    devtools_tab: &mut DevToolsTab,
    html: &mut String,
    headers: &HeaderMap,
    debug_info: &DebugInfo,
    network_logs: &Vec<RequestLog>,
    tx_ssh: &UnboundedSender<String>,
    is_detached: bool,
    devtools_state: &DevToolsState,
) -> Option<PanelAction> {
    let mut action: Option<PanelAction> = None;

    ui.horizontal(|ui| {
        if ui.button(if is_detached { "Прикрепить" } else { "Открепить" }).clicked() {
            action = Some(if is_detached { PanelAction::Attach } else { PanelAction::Detach });
            log::info!("Переключение состояния панели DevTools: {:?}", action);
        }
        ui.heading("Инструменты (F12)");
    });

    ui.horizontal_wrapped(|ui| {
        ui.set_min_width(150.0);
        ui.selectable_value(devtools_tab, DevToolsTab::Html, "HTML");
        ui.selectable_value(devtools_tab, DevToolsTab::Network, "Сеть");
        ui.selectable_value(devtools_tab, DevToolsTab::Debug, "Дебаг");
    });

    ui.separator();

    match *devtools_tab {
        DevToolsTab::Html => {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("Редактировать HTML:");
                ui.label(format!(
					"Фреймворк: {}",
					devtools_state.detected_framework
						.as_ref()  
						.map(|fw| format!("{:?}", fw))
						.unwrap_or("Не обнаружен".to_string())
				));
                ui.add_space(5.0);
                ui.add(
                    TextEdit::multiline(html)
                        .desired_rows(20)
                        .font(TextStyle::Monospace),
                );
            });
        }
        DevToolsTab::Network => {
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading("История HTTP-запросов:");
                for log in devtools_state.network_logs.iter().rev() {
                    ui.group(|ui| {
                        ui.label(format!("{} {} {}", log.method, log.url, log.status));
                        ui.label(format!("Длительность: {}", DebugInfo::format_duration(log.duration)));
                        ui.label(format!("Размер ответа: {}", DebugInfo::format_size(log.html.len())));
                        ui.label(format!("Скорость передачи: {:.2} KB/s",
                            if log.duration.as_secs_f64() > 0.0 {
                                log.html.len() as f64 / log.duration.as_secs_f64() / 1024.0
                            } else {
                                0.0
                            }
                        ));

                        let mut headers_text = String::new();
                        for (name, value) in log.headers.iter() {
                            if let Ok(value_str) = value.to_str() {
                                headers_text.push_str(&format!("{}: {}\n", name, value_str));
                            }
                        }

                        let desired_size = egui::Vec2::new(ui.available_width(), 20.0);
                        ui.add_sized(
                            desired_size,
                            egui::TextEdit::multiline(&mut headers_text)
                                .font(TextStyle::Monospace),
                        );
                    });
                    ui.add_space(5.0);
                }
            });
        }
        DevToolsTab::Debug => {
            ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(10.0);
                ui.label("Производительность:");
                ui.indent("performance", |ui| {
                    ui.label(format!("FPS: {:.1}", debug_info.frame_rate));
                    ui.label(format!(
                        "Время рендера: {}",
                        DebugInfo::format_duration(debug_info.render_time)
                    ));
                });
                ui.add_space(10.0);
                ui.label("DOM и ресурсы:");
                ui.indent("resources", |ui| {
                    ui.label(format!("Элементы DOM: {}", debug_info.dom_elements));
                    ui.label(format!("Память: {}", debug_info.memory_usage));
                });
                ui.add_space(10.0);
                let heavy_result_id = Id::new("heavy_calculator_result");
                let mut calculator: HeavyCalculator = ui.memory_mut(|mem| {
                    mem.data.get_temp(heavy_result_id).unwrap_or_default()
                });
                if ui.button("Дай синус бра!").clicked() {
                    calculator.compute(30_000_000);
                    ui.memory_mut(|mem| {
                        mem.data.insert_temp(heavy_result_id, calculator.clone());
                    });
                    log::info!("Выполнен тест производительности (синус)");
                }
                ui.add_space(10.0);
                if let Some(result_str) = calculator.format_result(DebugInfo::format_duration) {
                    ui.add_space(10.0);
                    ui.label(result_str);
                }
            });
        }
    }
    action
}

// === FILE: ui\font.rs ===
use anyhow::Result;
pub struct Font;
impl Font {
    pub fn new() -> Result<Self> {
        println!("Font initialized");
        Ok(Self)
    }
}

// === FILE: ui\heavy_calculator.rs ===
use rayon::prelude::*;
use std::time::{Duration, Instant};
use sysinfo::{System, RefreshKind};
#[derive(Clone, Default)]
pub struct HeavyCalculator {
    pub result: Option<(f64, Duration, i64, f32)>, // (значение, время, дельта памяти, дельта CPU)
}
impl HeavyCalculator {
    pub fn compute(&mut self, n: u32) {
        let rk = RefreshKind::everything();
        let mut sys = System::new_with_specifics(rk);
        sys.refresh_specifics(rk);
        let before_mem = sys.used_memory();
        let before_cpu = sys.global_cpu_usage();
        let start = Instant::now();
        let value = self.heavy(n);
        let duration = start.elapsed();
        sys.refresh_specifics(rk);
        let after_mem = sys.used_memory();
        let after_cpu = sys.global_cpu_usage();
        let mem_delta = (after_mem as i64 - before_mem as i64) / 1024;
        let cpu_delta = after_cpu - before_cpu;
        self.result = Some((value, duration, mem_delta, cpu_delta));
    }
    pub fn format_result(&self, format_duration: impl Fn(Duration) -> String) -> Option<String> {
        self.result.map(|(value, duration, mem_delta, cpu_delta)| {
            format!(
                "Результат: {:.4}\nВремя: {}\nПамять (delta): {} KB\nCPU (delta): {:.2}%",
                value,
                format_duration(duration),
                mem_delta,
                cpu_delta
            )
        })
    }
    pub fn heavy(&self, n: u32) -> f64 {
        (0..n)
            .into_par_iter()
            .map(|i| {
                let x = i as f64;
                x.sqrt() * x.sin()
            })
            .sum()
    }
}

// === FILE: ui\input.rs ===
use anyhow::Result;
pub struct Input;
impl Input {
    pub fn new() -> Result<Self> {
        println!("Input initialized");
        Ok(Self)
    }
}

// === FILE: ui\mod.rs ===
// === FILE: ui/mod.rs ===
use anyhow::Result;
use crate::core::engine::BroEngine;
use log;

pub mod aichat;
pub mod devtools;
pub mod render;
pub mod settings;
pub mod toolbar;
pub mod window;
pub mod window_manager;
pub mod wgpu_render;
pub mod heavy_calculator;

pub use window::ToolbarPosition;

// --- Точка входа UI-клиента ---
// Функция, которая запускает UI-клиент, используя инжектированный Engine.
pub async fn run(engine: std::sync::Arc<BroEngine>) -> Result<()> {
    use std::sync::Arc;
    use winit::event_loop::EventLoop;

    log::info!("UI: стартуем реальный UI (winit/egui/wgpu)");

    let config = crate::core::config::Config::load_from_file(
        crate::core::config::Config::get_config_path(),
    )?;

    // Создаем EventLoop
    let event_loop = EventLoop::new()?;

    let ctx = crate::core::event_loop::EventLoopContext {
        engine,
        config,
    };

    crate::core::event_loop::run(event_loop, ctx)?;

    Ok(())
}

// === FILE: ui\render.rs ===
use crate::dom::hydrate::hydrate_and_render;
use crate::dom::parser::{parse_and_process, ParsedNode, FrameworkType};
use crate::dom::tree::DomRenderer;
use egui::Ui;
use anyhow::Result;

pub struct HtmlRenderer {
    pub dom_renderer: DomRenderer,
    pub scale_factor: f32,
    last_html: String, // Кэш последнего HTML для проверки изменений
    cached_node: Option<ParsedNode>, // Кэш распарсенного DOM-дерева
    needs_repaint: bool, // Флаг необходимости перерисовки
}

impl HtmlRenderer {
    pub fn new(scale_factor: f32) -> Self {
        log::info!("Создание HtmlRenderer с масштабом: {}", scale_factor);
        Self {
            dom_renderer: DomRenderer::new(),
            scale_factor,
            last_html: String::new(),
            cached_node: None,
            needs_repaint: true, // Изначально требуется рендер
        }
    }

    pub fn set_scale_factor(&mut self, scale: f32) {
        if (self.scale_factor - scale).abs() > 0.01 {
            log::info!("Изменение масштаба HtmlRenderer: {} -> {}", self.scale_factor, scale);
            self.scale_factor = scale;
            self.needs_repaint = true; // Масштаб изменился, нужна перерисовка
        }
    }

    pub fn needs_repaint(&self) -> bool {
        self.needs_repaint || self.dom_renderer.needs_repaint()
    }

    pub fn clear_repaint_flag(&mut self) {
        log::info!("Сброс флага перерисовки в HtmlRenderer");
        self.needs_repaint = false;
        self.dom_renderer.clear_repaint_flag();
    }








// === FILE: ui/render.rs (Исправленная функция render_html_ui) ===

pub async fn render_html_ui(&mut self, ui: &mut Ui, html: &str) -> Result<Option<FrameworkType>> {
    let node: ParsedNode;
    let mut framework_to_return: Option<FrameworkType> = None;
    let mut should_hydrate = false;
    
    // Объявляем detected_framework заранее, чтобы избежать ошибки E0425.
    let mut detected_framework: Option<FrameworkType> = None; // 👈 ИСПРАВЛЕНИЕ 1: Добавляем объявление

    // 1. Определение, нужно ли парсить и гидрировать
    let was_repaint_needed = self.needs_repaint; // Сохраняем начальное состояние needs_repaint

    if self.last_html != html {
        // HTML ИЗМЕНИЛСЯ -> ПАРСИНГ и ПОЛНЫЙ РЕНДЕР
        log::info!("HTML изменился, выполняется парсинг: {} -> {} (длина: {})",
                     self.last_html.len(), html.len(), html.len());
        
        let (new_node, framework) = parse_and_process(html)?; // parse_and_process возвращает фреймворк
        node = new_node;
        self.last_html = html.to_string();
        self.cached_node = Some(node.clone());
        self.needs_repaint = true;
        should_hydrate = true;
        
        detected_framework = framework; // 👈 ИСПРАВЛЕНИЕ 2: Присваиваем значение
        
        // Удаляем: framework_to_return = detected_framework; (Это произойдет ниже в конце функции)

    } else if let Some(cached) = &self.cached_node {
        // HTML НЕ ИЗМЕНИЛСЯ -> ИСПОЛЬЗУЕМ КЭШ
        log::info!("HTML не изменился, используется кэшированное дерево");
        node = cached.clone();
        
        // Решаем, нужна ли гидрация/полный рендер по другим причинам
        if was_repaint_needed {
             // Флаг поднят (клик, масштаб и т.д.) -> НУЖЕН ПОЛНЫЙ РЕНДЕР
             log::info!("Требуется полный рендер из-за флага needs_repaint.");
             should_hydrate = true;
        }
    } else {
        // Кэш пуст при неизменном HTML (случай первого запуска) -> ПАРСИНГ и ПОЛНЫЙ РЕНДЕР
        log::info!("Кэш пуст при неизменном HTML, выполняется парсинг HTML");
        
        let (new_node, framework) = parse_and_process(html)?; // parse_and_process возвращает фреймворк
        node = new_node;
        self.cached_node = Some(node.clone());
        self.needs_repaint = true;
        should_hydrate = true;
        
        detected_framework = framework; // 👈 ИСПРАВЛЕНИЕ 3: Присваиваем значение
    }

    // 2. Выполнение Гидрации/Рендера
    if should_hydrate {
        // ВЫЗЫВАЕМ ПОЛНЫЙ ЦИКЛ: ГИДРАЦИЯ + РЕНДЕР
        log::info!("Запуск (условной) гидрации и рендера для HTML (длина: {})", html.len());
        
        // Передаем detected_framework в framework_to_return, если он был обнаружен при парсинге.
        let framework = hydrate_and_render(ui, &mut self.dom_renderer, node).await?;
        
        if framework.is_some() {
            framework_to_return = framework;
        } else if detected_framework.is_some() { // 👈 ИСПРАВЛЕНИЕ 4: Если framework_to_return не был установлен
             framework_to_return = detected_framework; 
        }

        self.needs_repaint = false; // Сброс флага
        
    } else {
        // ВЫЗЫВАЕМ ТОЛЬКО РЕНДЕР (если не нужна гидрация)
        log::info!("HTML и флаги чисты. Только рендер кэшированного DOM.");
        self.dom_renderer.render_node(ui, &node).await;
        // Здесь мы не сбрасываем needs_repaint, т.к. мы его не использовали.
        // Если он был true, то should_hydrate был бы true.
    }
    
    // 3. Управление флагами для следующего кадра
    if ui.ctx().has_requested_repaint() || self.dom_renderer.needs_repaint() {
        
		//////////////////////////////////////////////////////////////////
		self.needs_repaint = false; // true - если имеем дело с хтмл css анимацией!		
        //log::info!("HtmlRenderer запросил перерисовку после рендера (needs_repaint=true для следующего кадра).");
		//////////////////////////////////////////////////////////////////
		
		
    } else {
        self.needs_repaint = false;
    }


    Ok(framework_to_return)
}












    pub fn get_last_link_click(&mut self) -> Option<String> {
        if let Some(link) = self.dom_renderer.clicked_links.pop_front() {
            log::info!("Обнаружен клик по ссылке: {}", link);
            self.needs_repaint = true; // Клик требует перерисовки
            Some(link)
        } else {
            None
        }
    }

    pub fn get_last_button_click(&mut self) -> Option<String> {
        if let Some(button) = self.dom_renderer.clicked_buttons.pop_front() {
            log::info!("Обнаружен клик по кнопке: {}", button);
            self.needs_repaint = true; // Клик требует перерисовки
            Some(button)
        } else {
            None
        }
    }

    pub fn process_image_responses(&mut self, ui: &mut Ui) {
        let had_images = !self.dom_renderer.clicked_links.is_empty() || !self.dom_renderer.clicked_buttons.is_empty();
        self.dom_renderer.process_image_responses(ui);
        if had_images && self.dom_renderer.needs_repaint() {
            log::info!("DomRenderer запросил перерисовку из-за обработки изображений");
            self.needs_repaint = true;
            ui.ctx().request_repaint();
        }
    }

    pub fn set_egui_context(&mut self, ctx: egui::Context) {
        log::info!("Установка контекста egui для HtmlRenderer");
        self.dom_renderer.set_egui_context(ctx);
        self.needs_repaint = true; // Изменение контекста требует перерисовки
    }
}


// === FILE: ui\settings.rs ===
use egui::{Context as EguiContext, SidePanel, ScrollArea, ComboBox, Ui};
use crate::core::config::Config;
use crate::ui::devtools::PanelAction; // Импорт из devtools
pub fn render_settings(
    ctx: &EguiContext,
    config: &mut Config,
    save_config: &mut bool,
) {
    SidePanel::right("settings_panel")
        .resizable(true)
        .default_width(400.0)
        .min_width(200.0)
        .show(ctx, |ui| {
            render_settings_ui(ui, config, save_config, false)
        });
}
pub fn render_settings_ui(
    ui: &mut Ui,
    config: &mut Config,
    save_config: &mut bool,
    is_detached: bool,
) -> Option<PanelAction> {
    let mut action: Option<PanelAction> = None;
    ui.horizontal(|ui| {
		if ui.button(if is_detached { "🗔" } else { "🗔" }).clicked() {
            action = Some(if is_detached { PanelAction::Attach } else { PanelAction::Detach });
        }
		ui.heading("Настройки");
    });
    ScrollArea::vertical().show(ui, |ui| {
        ui.group(|ui| {
            ui.label("Модули:");
            ui.checkbox(&mut config.modules.core_enabled, "Core");
            ui.checkbox(&mut config.modules.ui_enabled, "UI");
            ui.checkbox(&mut config.modules.network_enabled, "Network");
            ui.checkbox(&mut config.modules.dom_enabled, "DOM");
            ui.checkbox(&mut config.modules.js_enabled, "JS");
            ui.checkbox(&mut config.modules.wasmcloud_enabled, "WasmCloud");
            ui.checkbox(&mut config.modules.p2p_enabled, "P2P");
            ui.checkbox(&mut config.modules.vdom_enabled, "VDOM");
            ui.checkbox(&mut config.modules.security_enabled, "Security");
            ui.checkbox(&mut config.modules.rendering_enabled, "Rendering");
            ui.checkbox(&mut config.modules.wasm_enabled, "WASM");
            ui.checkbox(&mut config.modules.reactive_enabled, "Reactive");
        });
        ui.add_space(20.0);
        ui.group(|ui| {
            ui.label("Общие настройки:");
            ui.checkbox(&mut config.settings.cache_enabled, "Включить кэширование");
            ui.label("Уровень логирования:");
            ComboBox::from_id_source("logging_level")
                .selected_text(&config.settings.logging_level)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut config.settings.logging_level, "trace".to_string(), "Trace");
                    ui.selectable_value(&mut config.settings.logging_level, "debug".to_string(), "Debug");
                    ui.selectable_value(&mut config.settings.logging_level, "info".to_string(), "Info");
                    ui.selectable_value(&mut config.settings.logging_level, "warn".to_string(), "Warn");
                    ui.selectable_value(&mut config.settings.logging_level, "error".to_string(), "Error");
                });
            ui.label("Максимальное количество потоков:");
            ui.add(
                egui::Slider::new(&mut config.settings.max_threads, 1..=16)
                    .text("потоков")
                    .clamp_to_range(true),
            );
        });
        ui.add_space(20.0);
        if ui.button("Сохранить настройки").clicked() {
            *save_config = true;
        }
    });
    action
}

// === FILE: ui\toolbar.rs ===
use egui::{Ui, TextEdit, Button, CornerRadius, Frame, Vec2, Margin, Rect, Layout, Align};
use std::collections::VecDeque;
use log;
pub fn render_toolbar(
    ui: &mut Ui,
    url_input: &mut String,
    url: &mut String,
    bookmarks: &VecDeque<String>,
    show_devtools: &mut bool,
    show_settings: &mut bool,
    status_message: &mut String,
    do_refresh: &mut bool,
    do_add_bookmark: &mut bool,
    new_url: &mut Option<String>,
    toolbar_position: &mut super::ToolbarPosition,
    show_toolbar: &mut bool,
) {
    let padding: f32 = 5.0;
    let height: f32 = 36.0;
    Frame::NONE
        .outer_margin(Margin::same(padding as i8))
        .show(ui, |ui| {
            let mut style = (*ui.ctx().style()).clone();
            let rounding = CornerRadius::same(12);
            style.visuals.widgets.inactive.corner_radius = rounding;
            style.visuals.widgets.hovered.corner_radius = rounding;
            style.visuals.widgets.active.corner_radius = rounding;
            ui.set_style(style);
            ui.spacing_mut().button_padding = Vec2::splat(padding);
            let available = ui.available_rect_before_wrap();
            let total_width = available.width();
            let col_width = total_width / 3.0;
            let collapse_threshold = 150.0;
            let menu_button_width = 24.0;
            log::debug!("Total width: {}, Col width: {}", total_width, col_width);
            let btn_w = 24.0;
            let inner_padding = padding * 2.0;
            let is_col1_collapsed = col_width < collapse_threshold;
            let is_col3_collapsed = col_width < collapse_threshold;
            let col2_width = if is_col1_collapsed || is_col3_collapsed {
                let col1_width = if is_col1_collapsed { menu_button_width } else { col_width };
                let col3_width = if is_col3_collapsed { menu_button_width } else { col_width };
                total_width - col1_width - col3_width
            } else {
                col_width
            };
            let rect1 = Rect::from_min_size(available.min, Vec2::new(if is_col1_collapsed { menu_button_width } else { col_width }, height));
            let rect2 = Rect::from_min_size(rect1.right_top(), Vec2::new(col2_width, height));
            let rect3 = Rect::from_min_size(rect2.right_top(), Vec2::new(if is_col3_collapsed { menu_button_width } else { col_width }, height));
            ui.allocate_ui_at_rect(rect1, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    if is_col1_collapsed {
                        ui.menu_button("☰", |ui| {
                            let spacing = ui.spacing_mut();
                            spacing.button_padding = Vec2::splat(padding);
                            Frame::NONE.outer_margin(Margin::same(5)).show(ui, |ui| {
                                if ui.add_sized([120.0, 24.0], Button::new("<")).clicked() {
                                    *status_message = "Назад нажато".to_owned();
                                    ui.close();
                                }
                                if ui.add_sized([120.0, 24.0], Button::new(">")).clicked() {
                                    *status_message = "Вперёд нажато".to_owned();
                                    ui.close();
                                }
                                if ui.add_sized([120.0, 24.0], Button::new("🔄")).clicked() {
                                    *do_refresh = true;
                                    ui.close();
                                }
                            });
                        });
                    } else {
						if ui.add_sized([24.0, 24.0], Button::new("🗔")).clicked() {
                            *toolbar_position = super::ToolbarPosition::Detached;
                            *show_toolbar = false;
                            *status_message = "Панель инструментов откреплена".to_string();
                        }
                        if ui.add_sized([24.0, 24.0], Button::new("<")).clicked() {
                            *status_message = "Назад нажато".to_owned();
                        }
                        if ui.add_sized([24.0, 24.0], Button::new(">")).clicked() {
                            *status_message = "Вперёд нажато".to_owned();
                        }
                        if ui.add_sized([24.0, 24.0], Button::new("🔄")).clicked() {
                            *do_refresh = true;
                        }
                    }
                });
            });
            ui.allocate_ui_at_rect(rect2, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    let edit_w = col2_width - btn_w - inner_padding * 2.0;
                    let text_edit = ui.add_sized(
                        [edit_w.max(100.0), height - inner_padding],
                        TextEdit::singleline(url_input)
                            .margin(Margin::same(padding as i8))
                            .hint_text("Куда собрался?"),
                    );
                    if text_edit.lost_focus() && ui.ctx().input(|i| i.key_pressed(egui::Key::Enter)) {
                        *url = url_input.clone();
                        *new_url = Some(url.clone());
                    }
                    if ui.add_sized([btn_w, 24.0], Button::new("▶")).clicked() {
                        *url = url_input.clone();
                        *new_url = Some(url.clone());
                    }
                });
            });
            ui.allocate_ui_at_rect(rect3, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let gap = 6.0;
                    if is_col3_collapsed {
                        ui.menu_button("⋮", |ui| {
                            let spacing = ui.spacing_mut();
                            spacing.button_padding = Vec2::splat(padding);
                            Frame::NONE.outer_margin(Margin::same(0)).show(ui, |ui| {
                                if ui.add_sized([120.0, 24.0], Button::new("Открепить")).clicked() {
                                    *toolbar_position = super::ToolbarPosition::Detached;
                                    *show_toolbar = false;
                                    *status_message = "Панель инструментов откреплена".to_string();
                                    ui.close();
                                }
                                if ui.add_sized([120.0, 24.0], Button::new("Настройки")).clicked() {
                                    *show_settings = !*show_settings;
                                    *status_message = if *show_settings {
                                        "Настройки открыты".to_owned()
                                    } else {
                                        "Настройки закрыты".to_owned()
                                    };
                                    ui.close();
                                }
                                if ui.add_sized([120.0, 24.0], Button::new("Инструменты разработчика")).clicked() {
                                    *show_devtools = !*show_devtools;
                                    *status_message = if *show_devtools {
                                        "Инструменты разработчика открыты".to_owned()
                                    } else {
                                        "Инструменты разработчика закрыты".to_owned()
                                    };
                                    ui.close();
                                }
                                ui.separator();
                                if ui.add_sized([120.0, 24.0], Button::new("➕ Добавить закладку")).clicked() {
                                    *do_add_bookmark = true;
                                    ui.close();
                                }
                                ui.separator();
                                for b in bookmarks {
                                    if ui.add_sized([120.0, 24.0], Button::new(b)).clicked() {
                                        *url_input = b.clone();
                                        *new_url = Some(b.clone());
                                        ui.close();
                                    }
                                }
                            });
                        });
                    } else {
                        if ui.add_sized([24.0, 24.0], Button::new("⚙")).clicked() {
                            *show_settings = !*show_settings;
                            *status_message = if *show_settings {
                                "Настройки открыты".to_owned()
                            } else {
                                "Настройки закрыты".to_owned()
                            };
                        }
                        
                        ui.add_space(gap);
                        if ui.add_sized([24.0, 24.0], Button::new("💻")).clicked() {
                            *show_devtools = !*show_devtools;
                            *status_message = if *show_devtools {
                                "Инструменты разработчика открыты".to_owned()
                            } else {
                                "Инструменты разработчика закрыты".to_owned()
                            };
                        }
                        ui.add_space(gap);
                        ui.menu_button("☆", |ui| {
                            let spacing = ui.spacing_mut();
                            spacing.button_padding = Vec2::splat(padding);
                            Frame::NONE.outer_margin(Margin::same(0)).show(ui, |ui| {
                                if ui.add_sized([120.0, 24.0], Button::new("➕ Добавить")).clicked() {
                                    *do_add_bookmark = true;
                                    ui.close();
                                }
                                ui.separator();
                                for b in bookmarks {
                                    if ui.add_sized([120.0, 24.0], Button::new(b)).clicked() {
                                        *url_input = b.clone();
                                        *new_url = Some(b.clone());
                                        ui.close();
                                    }
                                }
                            });
                        });
                    }
                });
            });
        });
}

// === FILE: ui\wgpu_render.rs ===
use wgpu::*;
use egui_wgpu::{Renderer, ScreenDescriptor};
use anyhow::Result;
use egui::{Context};
use std::time::{Instant, Duration};

#[derive(Debug, Clone, Copy)]
pub enum GraphicsBackend {
    Auto,
    Vulkan,
    DirectX12,
    Metal,
    OpenGL,
}

impl GraphicsBackend {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "vulkan" => Self::Vulkan,
            "dx12" | "directx12" | "directx" => Self::DirectX12,
            "metal" => Self::Metal,
            "gl" | "opengl" => Self::OpenGL,
            _ => Self::Auto,
        }
    }

    pub fn to_wgpu_backends(&self) -> Backends {
        match self {
            Self::Auto => Backends::PRIMARY,
            Self::Vulkan => Backends::VULKAN,
            Self::DirectX12 => Backends::DX12,
            Self::Metal => Backends::METAL,
            Self::OpenGL => Backends::GL,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Vulkan => "vulkan",
            Self::DirectX12 => "dx12",
            Self::Metal => "metal",
            Self::OpenGL => "opengl",
        }
    }
}

pub struct RenderManager {
    last_render_time: Option<Instant>,
    last_activity: Instant,
    frame_count: u32,
    fps_update_time: Instant,
    pub is_active: bool,
    pub idle_mode: bool,
    pub total_frames_rendered: u32,
    pub total_frames_skipped: u32,
    pub is_minimized: bool,
    enable_fps_throttling: bool,
    last_size: (u32, u32),
}

impl RenderManager {
    pub fn new(enable_optimizations: bool) -> Self {
        Self {
            last_render_time: None,
            last_activity: Instant::now(),
            frame_count: 0,
            fps_update_time: Instant::now(),
            is_active: true,
            idle_mode: false,
            total_frames_rendered: 0,
            total_frames_skipped: 0,
            is_minimized: false,
            enable_fps_throttling: enable_optimizations,
            last_size: (0, 0),
        }
    }

    pub fn set_window_active(&mut self, active: bool) {
        self.is_active = active;
        if active {
            self.update_activity();
        }
    }

    pub fn set_minimized(&mut self, minimized: bool) {
        self.is_minimized = minimized;
        if !minimized {
            self.update_activity();
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
        if self.idle_mode {
            log::debug!("Выход из idle режима");
        }
        self.idle_mode = false;
    }

    pub fn should_render(&mut self, ctx: &Context) -> bool {
        if self.is_minimized {
            self.total_frames_skipped += 1;
            return false;
        }

        let now = Instant::now();
        let time_since_activity = self.last_activity.elapsed();
        let (max_fps, mode_name) = if time_since_activity > Duration::from_secs(2) {
            self.idle_mode = true;
            (30, "idle")
        } else if !self.is_active {
            self.idle_mode = true;
            (1, "inactive")
        } else {
            self.idle_mode = false;
            (60, "active")
        };

        if self.enable_fps_throttling {
            if let Some(last_render) = self.last_render_time {
                let min_frame_time = Duration::from_millis(1000 / max_fps as u64);
                if now.duration_since(last_render) < min_frame_time {
                    self.total_frames_skipped += 1;
                    return false;
                }
            }
            if self.idle_mode && !ctx.has_requested_repaint() {
                self.total_frames_skipped += 1;
                return false;
            }
        }

        self.last_render_time = Some(now);
        self.total_frames_rendered += 1;
        self.frame_count += 1;

        if now.duration_since(self.fps_update_time) > Duration::from_secs(2) {
            let fps = self.frame_count as f32 / 2.0;
            let efficiency = if self.total_frames_rendered + self.total_frames_skipped > 0 {
                (self.total_frames_rendered as f32 / (self.total_frames_rendered + self.total_frames_skipped) as f32) * 100.0
            } else {
                0.0
            };
            log::debug!(
                "Режим: {}, FPS: {:.1}, Эффективность: {:.1}% (пропущено: {})",
                mode_name,
                fps,
                efficiency,
                self.total_frames_skipped
            );
            self.frame_count = 0;
            self.fps_update_time = now;
        }
        true
    }

    pub fn tessellate(
        &mut self,
        ctx: &Context,
        shapes: &[egui::epaint::ClippedShape],
        pixels_per_point: f32,
    ) -> Vec<egui::epaint::ClippedPrimitive> {
        let primitives = ctx.tessellate(shapes.to_vec(), pixels_per_point);
        for prim in &primitives {
            if let egui::epaint::ClippedPrimitive {
                primitive: egui::epaint::Primitive::Mesh(mesh),
                ..
            } = prim
            {
                if mesh.vertices.is_empty() || mesh.indices.is_empty() {
                    log::warn!("Обнаружен пустой меш при тесселяции");
                }
            }
        }
        primitives
    }

    pub fn update_screen_descriptor(&mut self, width: u32, height: u32, pixels_per_point: f32) -> ScreenDescriptor {
        let safe_width = width.max(1);
        let safe_height = height.max(1);
        if self.last_size != (safe_width, safe_height) {
            log::debug!("Обновлены размеры экрана: {}x{}", safe_width, safe_height);
            self.last_size = (safe_width, safe_height);
        }
        ScreenDescriptor {
            size_in_pixels: [safe_width, safe_height],
            pixels_per_point,
        }
    }

    pub fn get_stats(&self) -> String {
        let total = self.total_frames_rendered + self.total_frames_skipped;
        let efficiency = if total > 0 {
            (self.total_frames_rendered as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        format!(
            "Рендер: {} кадров, Пропущено: {} (Эффективность: {:.1}%)",
            self.total_frames_rendered,
            self.total_frames_skipped,
            efficiency
        )
    }

    pub fn clear_cache(&mut self) {
        log::debug!("Очистка кэша рендера");
    }

    pub fn set_all_optimizations(&mut self, enabled: bool) {
        self.enable_fps_throttling = enabled;
    }

    pub fn set_fps_throttling(&mut self, enabled: bool) {
        self.enable_fps_throttling = enabled;
    }
}

pub fn render_frame(
    device: &Device,
    queue: &Queue,
    surface: &Surface,
    surface_config: &SurfaceConfiguration,
    renderer: &mut Renderer,
    ctx: &Context,
    full_output: &egui::FullOutput,
    render_manager: &mut RenderManager,
) -> Result<bool> {
    // Проверка минимальных размеров и состояния минимизации
    if render_manager.is_minimized || surface_config.width == 0 || surface_config.height == 0 {
        render_manager.total_frames_skipped += 1;
        log::trace!("Пропуск рендеринга: минимизировано или нулевые размеры");
        return Ok(false);
    }

    // Проверка необходимости рендеринга
    if !render_manager.should_render(ctx) {
        log::trace!("Пропуск рендеринга: троттлинг или отсутствие изменений");
        return Ok(false);
    }

    // Получение текущей текстуры поверхности
    let output_frame = match surface.get_current_texture() {
        Ok(frame) => frame,
        Err(SurfaceError::Lost | SurfaceError::Outdated) => {
            // Переконфигурируем поверхность
            surface.configure(device, surface_config);
            match surface.get_current_texture() {
                Ok(frame) => frame,
                Err(e) => {
                    log::warn!("Повторная ошибка получения текстуры: {}", e);
                    return Ok(false);
                }
            }
        }
        Err(SurfaceError::OutOfMemory) => {
            return Err(anyhow::anyhow!("Недостаточно памяти для текстуры поверхности"));
        }
        Err(e) => {
            log::warn!("Ошибка получения текстуры поверхности: {}", e);
            return Ok(false);
        }
    };

    // Создание представления текстуры
    let view = output_frame
        .texture
        .create_view(&TextureViewDescriptor::default());




// Обновление текстур egui с проверкой размеров
for (id, image_delta) in &full_output.textures_delta.set {
    let size = image_delta.image.size();
    let [width, height] = size;
    renderer.update_texture(device, queue, *id, image_delta);
}












    // Освобождение неиспользуемых текстур
    for id in &full_output.textures_delta.free {
        renderer.free_texture(id);
    }

    // Пропуск рендеринга, если нет данных для отрисовки
    if full_output.shapes.is_empty() {
        log::trace!("Пропуск рендеринга: нет shapes для отрисовки");
        return Ok(true);
    }

    // Тесселяция UI
    let paint_jobs = render_manager.tessellate(ctx, &full_output.shapes, full_output.pixels_per_point);

    // Обновление дескриптора экрана
    let screen_descriptor = render_manager.update_screen_descriptor(
        surface_config.width,
        surface_config.height,
        full_output.pixels_per_point,
    );

 
 
 



// Создание командного энкодера
let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
    label: Some("egui_encoder"),
});

// Обновление буферов для рендеринга
if !paint_jobs.is_empty() {
    // В этом месте encoder заимствуется, но это временное заимствование.
    renderer.update_buffers(device, queue, &mut encoder, &paint_jobs, &screen_descriptor);
}

// Рендеринг кадра
if !paint_jobs.is_empty() {
    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("egui_rpass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
    })
    // ВОЗВРАЩАЕМ .forget_lifetime() ИЗ-ЗА ТРЕБОВАНИЯ 'static / E0505
    .forget_lifetime(); 
    
    // Используем rpass для рендеринга
    renderer.render(&mut rpass, &paint_jobs, &screen_descriptor);

    // rpass теперь можно игнорировать, так как он не держит &mut encoder.
} 

// Отправка команд в очередь
if !paint_jobs.is_empty() {
    queue.submit(std::iter::once(encoder.finish()));
}
 
 
 
 
 
 

 

    // Презентация кадра
    output_frame.present();
    log::trace!("Кадр успешно отрендерен: {}x{}", surface_config.width, surface_config.height);

    Ok(true)
}

pub fn create_optimal_surface_config(
    surface_caps: &SurfaceCapabilities,
    size: (u32, u32),
) -> Result<SurfaceConfiguration> {
    let surface_format = surface_caps
        .formats
        .get(0)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Нет доступных surface formats"))?;
    let preferred_modes = [
		PresentMode::Mailbox, // Mailbox для лучшей производительности без tearing
		PresentMode::Fifo, // FIFO (очередь) гарантирует отсутствие tearing, синхронизирует с вертикальной разверткой
		PresentMode::FifoRelaxed, // Ослабленная FIFO: минимальная задержка, возможен tearing при пропуске кадра
		PresentMode::Immediate, // Немедленная подача кадра: максимальная скорость, но возможен tearing

    ];
    let present_mode = preferred_modes
        .iter()
        .find(|mode| surface_caps.present_modes.contains(mode))
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Нет доступных present modes"))?;
    log::debug!(
        "Выбран режим представления: {:?} (для предотвращения suboptimal present)",
        present_mode
    );
    Ok(SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.0.max(1),
        height: size.1.max(1),
        present_mode,
        alpha_mode: surface_caps.alpha_modes.get(0).copied().unwrap_or_default(),
        view_formats: vec![surface_format],
        desired_maximum_frame_latency: 2,
    })
}

pub fn create_energy_efficient_instance() -> Instance {
    Instance::new(&InstanceDescriptor {
        backends: Backends::PRIMARY,
        flags: InstanceFlags::empty(),
        ..Default::default()
    })
}

pub fn create_energy_efficient_adapter(instance: &Instance, surface: &Surface) -> Result<Adapter> {
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: PowerPreference::LowPower,
        force_fallback_adapter: false,
        compatible_surface: Some(surface),
    }))
    .map_err(|e| anyhow::anyhow!("Ошибка при запросе адаптера: {e}"))?;
    Ok(adapter)
}

// === FILE: ui\window.rs ===
use crate::core::config::Config;
use crate::core::engine::{BroEngine, EngineEvent, UrlResponse};
use crate::core::page_state::SharedPageState;
use crate::dom::parser::{parse_and_process, FrameworkType};
use crate::ui::devtools::{DebugInfo, DevToolsTab, RequestLog, PanelAction, DevToolsState};
use crate::ui::wgpu_render;
use crate::ui::window_manager::{DetachedWindowData, PanelType, WindowManager, WindowState};
use crate::ui::render::HtmlRenderer;
use crate::ui::{aichat, devtools, settings, toolbar};
use anyhow::{Result, Context};
use egui::{Context as EguiContext, FontDefinitions, FontId, FontFamily, TextStyle, Color32, CornerRadius, Style, TopBottomPanel, Frame, SidePanel, CentralPanel, ScrollArea};
use egui::epaint::mutex::RwLock;
use egui::widgets::text_edit::TextEditOutput;
use egui_extras::{Column, TableBuilder};
use egui_winit::State as EguiState;
use pollster::FutureExt as _;
use std::collections::VecDeque;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::Window as WinitWindow;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration, DeviceDescriptor, Limits};
use egui_wgpu::{Renderer, RendererOptions};
use reqwest::header::HeaderMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};


#[derive(Debug, Clone, PartialEq)]
pub enum ToolbarPosition {
    Top,
    Bottom,
    Floating,
    Detached,
    Hidden,
}

pub struct Window {
    pub window: Arc<WinitWindow>,
    pub config: Config,
    pub egui_state: EguiState,
    pub egui_ctx: EguiContext,
    pub url: String,
    pub html: String,
    pub status_message: String,
    pub bookmarks: VecDeque<String>,
    pub show_devtools: bool,
    pub show_settings: bool,
    pub show_toolbar: bool,
    pub devtools_tab: DevToolsTab,
    pub headers: HeaderMap,
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface_config: SurfaceConfiguration,
    pub renderer: Renderer,
    pub html_scale_factor: f32,
    pub html_renderer: HtmlRenderer,
    pub tx_ssh: UnboundedSender<String>,
    pub render_manager: wgpu_render::RenderManager,
    pub url_input: String,
    pub ai_chat_input: String,
    pub ai_chat_history: Vec<(String, String)>,
    pub send_ai_message: bool,
    pub save_config: bool,
    pub do_refresh: bool,
    pub do_add_bookmark: bool,
    pub new_url: Option<String>,
    pub debug_info: DebugInfo,
    pub last_request_time: Option<Instant>,
    pub window_state: WindowState,
    pub window_manager: WindowManager,
    pub engine: Arc<BroEngine>,
    pub devtools_state: DevToolsState,
}

impl Window {
    pub async fn new(
        window: Arc<WinitWindow>,
        config: &Config,
        engine: Arc<BroEngine>,
        enable_optimizations: bool,
        event_loop: &ActiveEventLoop,
    ) -> Result<Self> {
        log::info!("Создание нового окна с масштабом: {}", config.settings.scale_factor.unwrap_or(1.0));
        let instance = wgpu_render::create_energy_efficient_instance();
        let surface = instance
            .create_surface(window.clone())
            .context("Не удалось создать wgpu поверхность")?;
        let adapter = wgpu_render::create_energy_efficient_adapter(&instance, &surface)?;
        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            required_limits: Limits::default(),
            ..Default::default()
        }).await.context("Не удалось создать wgpu устройство")?;
        let size = window.inner_size();
        if size.width == 0 || size.height == 0 {
            return Err(anyhow::anyhow!(
                "Недопустимый размер окна: {}x{}",
                size.width,
                size.height
            ));
        }
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_config = wgpu_render::create_optimal_surface_config(
            &surface_caps,
            (size.width, size.height),
        )?;
        surface.configure(&device, &surface_config);
        let renderer = Renderer::new(&device, surface_config.format, RendererOptions::default());
        let egui_ctx = EguiContext::default();
        let system_scale_factor = window.scale_factor() as f32;
        egui_ctx.set_pixels_per_point(system_scale_factor);
        let fonts = egui::FontDefinitions::default();
        egui_ctx.set_fonts(fonts);
        let mut style = Style::default();
        style.text_styles.insert(TextStyle::Body, FontId::new(12.0, FontFamily::Proportional));
        style.text_styles.insert(TextStyle::Heading, FontId::new(20.0, FontFamily::Proportional));
        style.text_styles.insert(TextStyle::Monospace, FontId::new(12.0, FontFamily::Monospace));
        egui_ctx.set_style(style);
        let egui_state = EguiState::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            event_loop,
            None,
            None,
            None,
        );
        let render_manager = wgpu_render::RenderManager::new(enable_optimizations);
        let mut html_renderer = HtmlRenderer::new(system_scale_factor);
        html_renderer.set_egui_context(egui_ctx.clone());
        let window_manager = WindowManager::new(instance.clone(), adapter.clone(), device.clone(), queue.clone());
        window.set_maximized(true);
        window.set_min_inner_size(Some(winit::dpi::PhysicalSize::new(320, 480)));
        let (tx_ssh, _rx_ssh) = unbounded_channel::<String>();
        
        // Создаем DevToolsState с ресивером событий движка
        let devtools_state = DevToolsState {
            engine: engine.clone(),
            detected_framework: None,
            network_logs: Vec::new(),
            debug_info: DebugInfo::new(),
            rx_engine: engine.event_receiver(),
        };

        let instance_self = Self {
            window,
            config: config.clone(),
            egui_state,
            egui_ctx,
            url: config.settings.default_url.clone(),
            url_input: config.settings.default_url.clone(),
            html: r#"<html><body><h1>Добро пожаловать!</h1>
<pre>
____________________ ________   
\______   \______   \\_____  \  
 |    |  _/|       _/ /   |   \ 
 |    |   \|    |   \/    |    \
 |________/|____|___/\_________/
 </pre>
</body>
</html>"#.to_string(),
            status_message: "Готово".to_string(),
            bookmarks: VecDeque::new(),
            show_devtools: false,
            show_settings: false,
            show_toolbar: true,
            devtools_tab: DevToolsTab::Html,
            headers: HeaderMap::new(),
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
            renderer,
            render_manager,
            html_scale_factor: system_scale_factor,
            html_renderer,
            tx_ssh,
            ai_chat_input: String::new(),
            ai_chat_history: Vec::new(),
            send_ai_message: false,
            save_config: false,
            do_refresh: false,
            do_add_bookmark: false,
            new_url: None,
            debug_info: DebugInfo::new(),
            last_request_time: None,
            window_state: WindowState::default(),
            window_manager,           
            engine,
            devtools_state,
        };
        instance_self.window.set_visible(true);
        log::info!("Окно инициализировано, начальный URL: {}", instance_self.url);
        Ok(instance_self)
    }

    pub fn handle_event(&mut self, window_event: &WindowEvent, window_id: winit::window::WindowId) -> Result<()> {
        log::info!("Обработка события окна: {:?}", window_event);
        self.render_manager.update_activity();
        if window_id == self.window.id() {
            if let WindowEvent::ScaleFactorChanged { scale_factor, .. } = window_event {
                let new_scale = *scale_factor as f32;
                self.egui_ctx.set_pixels_per_point(new_scale);
                self.html_renderer.set_scale_factor(new_scale);
                self.html_scale_factor = new_scale;
                log::info!("Изменение системного масштаба: {:.1}x, запрос перерисовки", new_scale);
            }
            let resp = self.egui_state.on_window_event(&self.window, window_event);
            if resp.repaint {
                log::info!("EguiState запросил перерисовку после события окна");
                self.window.request_redraw();
            }
        } else {
            for (panel_type, _) in self.window_state.detached_windows.iter() {
                if self.window_state.detached_windows.get(panel_type).map(|dw| dw.window.id()) == Some(window_id) {
                    self.window_manager.handle_window_event(panel_type, window_event)?;
                }
            }
        }
        Ok(())
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, window_id: winit::window::WindowId) -> Result<()> {
        log::info!("Изменение размера окна: {}x{}", new_size.width, new_size.height);
        if window_id == self.window.id() {
            if new_size.width > 0 && new_size.height > 0 {
                let surface_caps = self.surface.get_capabilities(&self.adapter);
                self.surface_config = wgpu_render::create_optimal_surface_config(
                    &surface_caps,
                    (new_size.width, new_size.height),
                )?;
                self.surface.configure(&self.device, &self.surface_config);
                log::info!("Конфигурация поверхности обновлена, запрос перерисовки");
            }
        } else {
            for (panel_type, _) in self.window_state.detached_windows.iter() {
                if self.window_state.detached_windows.get(panel_type).map(|dw| dw.window.id()) == Some(window_id) {
                    self.window_manager.resize_detached_window(panel_type, new_size)?;
                }
            }
        }
        Ok(())
    }

    pub fn reset_scale_factor(&mut self) {
        self.html_renderer.set_scale_factor(self.html_scale_factor);
        self.egui_ctx.set_pixels_per_point(self.html_scale_factor);
        self.status_message = format!("Масштаб сброшен: {:.1}x", self.html_scale_factor);
        log::info!("Масштаб сброшен: {:.1}x, запрос перерисовки", self.html_scale_factor);
    }

    pub fn change_scale_factor(&mut self, delta: f32) {
        let new_scale = (self.html_scale_factor + delta).max(0.5).min(3.0);
        if (new_scale - self.html_scale_factor).abs() > 0.01 {
            self.html_scale_factor = new_scale;
            self.html_renderer.set_scale_factor(new_scale);
            self.egui_ctx.set_pixels_per_point(new_scale);
            self.status_message = format!("Масштаб: {:.1}x", new_scale);
            log::info!("Масштаб изменён: {:.1}x, запрос перерисовки", new_scale);
        }
    }

    // ИСПРАВЛЕННАЯ ФУНКЦИЯ: Загружает URL и обновляет UI через события движка
    pub fn load_url(&mut self, url: String) {
        log::info!("Загрузка URL: {}", url);
        let engine = self.engine.clone();
        let value = url.clone();
        // Запускаем фоновую задачу для загрузки
        tokio::spawn(async move {
            match engine.fetch_url(value.clone()).await {
                Ok(response) => {
                    log::info!("URL загружен успешно: {}", value);
                    engine.send_event(EngineEvent::UrlResponse(response));
                }
                Err(e) => {
                    log::error!("Ошибка загрузки URL {}: {}", value, e);
                    engine.send_event(EngineEvent::Error(format!("Ошибка загрузки {}: {}", value, e)));
                }
            }
        });
        
        self.status_message = format!("Загрузка: {}...", url);
        self.egui_ctx.request_repaint();
    }

    pub fn navigate_to_url(&mut self, url: String) { 
        log::info!("Навигация к URL: {}", url);
        self.load_url(url);
    }

    fn render_toolbar_panel(&mut self, ctx: &EguiContext, ui: &mut egui::Ui) {
        toolbar::render_toolbar(
            ui,
            &mut self.url_input,
            &mut self.url,
            &self.bookmarks,
            &mut self.show_devtools,
            &mut self.show_settings,
            &mut self.status_message,
            &mut self.do_refresh,
            &mut self.do_add_bookmark,
            &mut self.new_url,
            &mut self.window_state.toolbar_position,
            &mut self.show_toolbar,
        );
        if ctx.has_requested_repaint() {
            log::info!("Панель инструментов запросила перерисовку");
        }
    }

    pub async fn render(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        let html_snapshot = self.html.clone();
        let start_time = Instant::now();
        log::info!("Начало рендеринга окна, HTML длина: {}", html_snapshot.len());

        // ОБНОВЛЕНИЕ: Обрабатываем события движка перед рендером
        self.devtools_state.poll_events();

        if self.save_config {
            log::info!("Сохранение конфигурации");
            if let Err(e) = self.config.save_to_file(Config::get_config_path()) {
                self.status_message = format!("Ошибка сохранения настроек: {}", e);
                log::error!("Ошибка сохранения конфигурации: {}", e);
            } else {
                self.status_message = "Настройки сохранены".to_string();
                if self.url.is_empty() || self.url == "about:blank" {
                    self.url = self.config.settings.default_url.clone();
                    self.url_input = self.config.settings.default_url.clone();
                    log::info!("Установка начального URL: {}", self.url);
                    self.load_url(self.url.clone());
                }
                log::info!("Конфигурация сохранена, запрос перерисовки");
            }
            self.save_config = false;
        }

        if self.do_refresh {
            log::info!("Обновление страницы: {}", self.url);
            self.load_url(self.url.clone());
            self.do_refresh = false;
        }

        if self.do_add_bookmark {
            log::info!("Добавление закладки: {}", self.url);
            if !self.bookmarks.contains(&self.url) {
                self.bookmarks.push_back(self.url.clone());
                self.status_message = format!("Добавлена закладка: {}", self.url);
            }
            self.do_add_bookmark = false;
        }

        if let Some(new_url) = self.new_url.take() {
            log::info!("Навигация к новому URL: {}", new_url);
            self.navigate_to_url(new_url);
        }

        // ОБНОВЛЕНИЕ: Обработка событий движка из DevToolsState
        // События уже обработаны в poll_events(), данные обновлены в devtools_state
        // Копируем актуальные данные в UI состояние
        if let Some(latest_log) = self.devtools_state.network_logs.last() {
            if latest_log.url == self.url && self.html != latest_log.html {
                self.html = latest_log.html.clone();
                self.headers = latest_log.headers.clone();
                self.status_message = format!("Загружено: {}", self.url);
                log::info!("HTML обновлен из сетевого лога для URL: {}", self.url);
            }
        }

        let clicked_link = self.html_renderer.get_last_link_click();
        let clicked_button = self.html_renderer.get_last_button_click();
        let raw_input = self.egui_state.take_egui_input(&self.window);
        let mut reset = false;
        let mut scale_delta: f32 = 0.0;
        let egui_ctx_clone = self.egui_ctx.clone();
        let full_output = egui_ctx_clone.run(raw_input, |ctx| {
            if !ctx.memory(|mem| mem.data.get_temp(egui::Id::new("style_configured")).unwrap_or(false)) {
                let mut style = (*ctx.style()).clone();
                let visuals = &mut style.visuals;
                visuals.widgets.inactive.bg_fill = Color32::from_rgb(40, 40, 60);
                visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 90);
                visuals.widgets.active.bg_fill = Color32::from_rgb(80, 80, 120);
                visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(220, 220, 255);
                visuals.widgets.hovered.fg_stroke.color = Color32::from_rgb(255, 255, 255);
                visuals.panel_fill = Color32::from_rgb(0, 54, 0);
                visuals.selection.bg_fill = Color32::from_rgb(70, 130, 180);
                visuals.extreme_bg_color = Color32::from_rgb(0, 80, 0);
                visuals.widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
                let rounding = CornerRadius::same(12);
                visuals.widgets.inactive.corner_radius = rounding;
                visuals.widgets.hovered.corner_radius = rounding;
                visuals.widgets.active.corner_radius = rounding;
                visuals.widgets.open.corner_radius = rounding;
                visuals.widgets.noninteractive.corner_radius = rounding;
                ctx.set_style(style);
                ctx.memory_mut(|mem| mem.data.insert_temp(egui::Id::new("style_configured"), true));
                log::info!("Стили egui инициализированы");
            }

            if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Num0)) {
                log::info!("Обнаружено нажатие Ctrl+0, сброс масштаба");
                reset = true;
            }
            if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Plus)) {
                log::info!("Обнаружено нажатие Ctrl++, увеличение масштаба");
                scale_delta = 0.1;
            }
            if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Minus)) {
                log::info!("Обнаружено нажатие Ctrl+-, уменьшение масштаба");
                scale_delta = -0.1;
            }
            if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Equals)) {
                log::info!("Обнаружено нажатие Ctrl+=, увеличение масштаба");
                scale_delta = 0.1;
            }

            if self.show_toolbar && !self.window_state.detached_windows.contains_key(&PanelType::Toolbar) {
                match self.window_state.toolbar_position {
                    ToolbarPosition::Top => {
                        TopBottomPanel::top("nav_panel").show(ctx, |ui| {
                            self.render_toolbar_panel(ctx, ui);
                        });
                    }
                    ToolbarPosition::Bottom => {
                        TopBottomPanel::bottom("nav_panel").show(ctx, |ui| {
                            self.render_toolbar_panel(ctx, ui);
                        });
                    }
                    ToolbarPosition::Floating => {
                        egui::Window::new("Панель инструментов")
                            .default_pos(self.window_state.toolbar_floating_pos)
                            .default_size((300.0, 60.0))
                            .resizable(true)
                            .frame(
                                Frame::window(&ctx.style())
                                    .corner_radius(12.0)
                                    .inner_margin(egui::Margin::same(6))
                            )
                            .show(ctx, |ui| {
                                self.render_toolbar_panel(ctx, ui);
                            });
                    }
                    ToolbarPosition::Detached => {
                        if !self.window_state.detached_windows.contains_key(&PanelType::Toolbar) {
                            log::info!("Создание откреплённого окна для панели инструментов");
                            if let Err(e) = self.window_manager.create_detached_window(
                                event_loop,
                                PanelType::Toolbar,
                                (50.0, 50.0),
                                (400.0, 60.0),
                                &self.config,
                            ) {
                                self.status_message = format!("Ошибка создания окна toolbar: {}", e);
                                log::error!("Ошибка создания окна toolbar: {}", e);
                            }
                        }
                    }
                    ToolbarPosition::Hidden => {}
                }
            }

            if self.show_devtools && !self.window_state.detached_windows.contains_key(&PanelType::DevTools) {
                let width = *self.window_state.panel_sizes.get(&PanelType::DevTools).unwrap_or(&400.0);
                let mut new_width = width;
                SidePanel::right("devtools_panel")
                    .resizable(true)
                    .default_width(width)
                    .min_width(200.0)
                    .show(ctx, |ui| {
                        let action = devtools::render_devtools_ui(
                            ui,
                            &mut self.devtools_tab,
                            &mut self.html,
                            &self.headers,
                            &self.debug_info,
                            &self.devtools_state.network_logs,
                            &self.tx_ssh,
                            false,
                            &self.devtools_state,
                        );
                        new_width = ui.available_width();
                        if let Some(PanelAction::Detach) = action {
                            log::info!("Запрос на открепление DevTools");
                            if let Err(e) = self.window_manager.create_detached_window(
                                event_loop,
                                PanelType::DevTools,
                                (ctx.viewport_rect().right() - new_width - 20.0, 20.0),
                                (new_width, ctx.viewport_rect().height() - 40.0),
                                &self.config,
                            ) {
                                self.status_message = format!("Ошибка создания окна DevTools: {}", e);
                                log::error!("Ошибка создания окна DevTools: {}", e);
                            }
                        }
                    });
                self.window_state.panel_sizes.insert(PanelType::DevTools, new_width);
                if ctx.has_requested_repaint() {
                    log::info!("DevTools запросил перерисовку");
                }
            }

            if self.show_settings && !self.window_state.detached_windows.contains_key(&PanelType::Settings) {
                let width = *self.window_state.panel_sizes.get(&PanelType::Settings).unwrap_or(&400.0);
                let mut new_width = width;
                SidePanel::right("settings_panel")
                    .resizable(true)
                    .default_width(width)
                    .min_width(200.0)
                    .show(ctx, |ui| {
                        let action = settings::render_settings_ui(
                            ui,
                            &mut self.config,
                            &mut self.save_config,
                            false,
                        );
                        new_width = ui.available_width();
                        if let Some(PanelAction::Attach) = action {
                            log::info!("Запрос на открепление Settings");
                            if let Err(e) = self.window_manager.create_detached_window(
                                event_loop,
                                PanelType::Settings,
                                (ctx.viewport_rect().right() - new_width - 20.0, 20.0),
                                (new_width, ctx.viewport_rect().height() - 40.0),
                                &self.config,
                            ) {
                                self.status_message = format!("Ошибка создания окна Settings: {}", e);
                                log::error!("Ошибка создания окна Settings: {}", e);
                            }
                        }
                    });
                self.window_state.panel_sizes.insert(PanelType::Settings, new_width);
                if ctx.has_requested_repaint() {
                    log::info!("Settings запросил перерисовку");
                }
            }

            CentralPanel::default()
                .show(ctx, |ui| {
                    ScrollArea::both().auto_shrink([false; 2])
                        .show(ui, |ui| {
                            self.html_renderer.process_image_responses(ui);
                            if let Err(e) = pollster::block_on(self.html_renderer.render_html_ui(ui, &html_snapshot)) {
                                ui.label(format!("Ошибка рендеринга: {}", e));
                                ui.monospace(&html_snapshot);
                                log::error!("Ошибка рендеринга HTML: {}", e);
                            }
                        });
                });
            if ctx.has_requested_repaint() {
                log::info!("CentralPanel запросил перерисовку");
            }

            TopBottomPanel::bottom("status_bar")
                .show(ctx, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(format!("Статус: {}", self.status_message));
                        ui.label(format!("Масштаб: {:.1}x (Ctrl+0 для сброса)", self.html_scale_factor));
                    });
                });
            if ctx.has_requested_repaint() {
                log::info!("StatusBar запросил перерисовку");
            }
        });

        let should_repaint = self.egui_ctx.has_requested_repaint();
        if should_repaint {
            log::info!("Egui контекст запросил перерисовку");
        }

        if reset {
            log::info!("Сброс масштаба");
            self.reset_scale_factor();
        } else if scale_delta.abs() > 0.0 {
            log::info!("Изменение масштаба на: {}", scale_delta);
            self.change_scale_factor(scale_delta);
        }

        let platform_output = full_output.platform_output.clone();
        self.egui_state.handle_platform_output(&self.window, platform_output);

        if let Some(link) = clicked_link {
            log::info!("Навигация по клику на ссылке: {}", link);
            self.navigate_to_url(link);
        }
        if let Some(button) = clicked_button {
            log::info!("Обработка клика по кнопке: {}", button);
            self.status_message = format!("Нажата кнопка: {}", button);
        }

        let size = self.window.inner_size();
        let minimized = self.window.is_minimized().unwrap_or(false);
        let mut frame_rendered = false;
        if !minimized && size.width > 0 && size.height > 0 && should_repaint {
            log::info!("Запуск рендера кадра, размер окна: {}x{}", size.width, size.height);
            frame_rendered = wgpu_render::render_frame(
                &self.device,
                &self.queue,
                &self.surface,
                &self.surface_config,
                &mut self.renderer,
                &self.egui_ctx,
                &full_output,
                &mut self.render_manager,
            )?;
        } else {
            log::info!("Пропуск рендера кадра: минимизировано={} или размер=0", minimized);
            self.render_manager.total_frames_skipped += 1;
        }

        if frame_rendered {
            let is_active = self.render_manager.is_active;
            log::info!("Кадр отрендерен, активность: {}", is_active);
            self.debug_info.update(
                &self.html,
                start_time.elapsed(),
                self.debug_info.server_response_time,
            );
        }

        let panel_types = self.window_manager.get_detached_window_types();
        for panel_type in panel_types {
            let window_data = crate::ui::window_manager::DetachedWindowData {
                devtools_tab: &mut self.devtools_tab,
                html: &mut self.html,
                headers: &self.headers,
                debug_info: &self.debug_info,
                network_logs: &self.devtools_state.network_logs,
                tx_ssh: &self.tx_ssh,
                devtools_state: &self.devtools_state,
                url_input: &mut self.url_input,
                url: &mut self.url,
                bookmarks: &self.bookmarks,
                show_devtools: &mut self.show_devtools,
                show_settings: &mut self.show_settings,
                status_message: &mut self.status_message,
                do_refresh: &mut self.do_refresh,
                do_add_bookmark: &mut self.do_add_bookmark,
                new_url: &mut self.new_url,
                toolbar_position: &mut self.window_state.toolbar_position,
                show_toolbar: &mut self.show_toolbar,
                config: &mut self.config,
                save_config: &mut self.save_config,
                ai_chat_history: &self.ai_chat_history,
                send_ai_message: &mut self.send_ai_message,
                render_manager: &mut self.render_manager,
            };
            self.window_manager.render_detached_window(&panel_type, window_data)?;
        }

        self.window_manager.cleanup_closed_windows();
        self.html_renderer.clear_repaint_flag();

        Ok(())
    }

    pub fn set_focused(&mut self, focused: bool) {
        log::info!("Установка фокуса окна: {}", focused);
        self.render_manager.set_window_active(focused);
    }
}

// === FILE: ui\window_manager.rs ===
use crate::core::config::Config;
use crate::ui::devtools::{render_devtools_ui, PanelAction, DevToolsState};
use crate::ui::{toolbar, aichat, settings, wgpu_render};
use anyhow::{Result, Context};
use egui::{Context as EguiContext, Pos2, FontId, FontFamily, TextStyle, Color32, CornerRadius};
use egui_winit::State as EguiState;
use winit::window::Window as WinitWindow;
use winit::event_loop::ActiveEventLoop;
use winit::event::WindowEvent;
use wgpu::{Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration};
use egui_wgpu::{Renderer, RendererOptions};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PanelType {
    DevTools,
    AIChat,
    Settings,
    Toolbar,
}

pub struct DetachedWindow {
    pub panel_type: PanelType,
    pub window: Arc<WinitWindow>,
    pub surface: Surface<'static>,
    pub renderer: Renderer,
    pub egui_state: EguiState,
    pub egui_ctx: EguiContext,
    pub surface_config: SurfaceConfiguration,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub visible: bool,
    pub should_close: bool,
}

pub struct WindowState {
    pub fullscreen: bool,
    pub toolbar_position: super::ToolbarPosition,
    pub toolbar_floating_pos: Pos2,
    pub detached_windows: HashMap<PanelType, DetachedWindow>,
    pub panel_sizes: HashMap<PanelType, f32>,
}

impl Default for WindowState {
    fn default() -> Self {
        let mut panel_sizes = HashMap::new();
        panel_sizes.insert(PanelType::DevTools, 400.0);
        panel_sizes.insert(PanelType::AIChat, 350.0);
        panel_sizes.insert(PanelType::Settings, 400.0);
        panel_sizes.insert(PanelType::Toolbar, 400.0);
        Self {
            fullscreen: true,
            toolbar_position: super::ToolbarPosition::Top,
            toolbar_floating_pos: Pos2::new(50.0, 50.0),
            detached_windows: HashMap::new(),
            panel_sizes,
        }
    }
}

// === ДАННЫЕ ДЛЯ ОТКРЕПЛЁННЫХ ОКОН — ТОЛЬКО ЧЕРЕЗ API ===
pub struct DetachedWindowData<'a> {
    pub devtools_tab: &'a mut crate::ui::devtools::DevToolsTab,
    pub html: &'a mut String,
    pub headers: &'a reqwest::header::HeaderMap,
    pub debug_info: &'a crate::ui::devtools::DebugInfo,
    pub network_logs: &'a Vec<crate::ui::devtools::RequestLog>,
    pub tx_ssh: &'a UnboundedSender<String>,
    pub devtools_state: &'a DevToolsState,  // ← Единственный источник данных от движка
    pub url_input: &'a mut String,
    pub url: &'a mut String,
    pub bookmarks: &'a std::collections::VecDeque<String>,
    pub show_devtools: &'a mut bool,
    pub show_settings: &'a mut bool,
    pub status_message: &'a mut String,
    pub do_refresh: &'a mut bool,
    pub do_add_bookmark: &'a mut bool,
    pub new_url: &'a mut Option<String>,
    pub toolbar_position: &'a mut super::ToolbarPosition,
    pub show_toolbar: &'a mut bool,
    pub config: &'a mut Config,
    pub save_config: &'a mut bool,
    pub ai_chat_history: &'a Vec<(String, String)>,
    pub send_ai_message: &'a mut bool,
    pub render_manager: &'a mut crate::ui::wgpu_render::RenderManager,
}

pub struct WindowManager {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub window_state: WindowState,
}

impl WindowManager {
    pub fn new(instance: Instance, adapter: Adapter, device: Device, queue: Queue) -> Self {
        Self {
            instance,
            adapter,
            device,
            queue,
            window_state: WindowState::default(),
        }
    }

    fn configure_egui_style(ctx: &EguiContext) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(15.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(12.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(12.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(12.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(10.0, FontFamily::Proportional)),
        ].into();
        
        let visuals = &mut style.visuals;
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(40, 40, 60);
        visuals.widgets.hovered.bg_fill  = Color32::from_rgb(60, 60, 90);
        visuals.widgets.active.bg_fill   = Color32::from_rgb(80, 80, 120);
        visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(220, 220, 255);
        visuals.widgets.hovered.fg_stroke.color  = Color32::from_rgb(255, 255, 255);
        visuals.panel_fill   = Color32::from_rgb(0, 54, 0);
        visuals.selection.bg_fill = Color32::from_rgb(70, 130, 180);
        visuals.extreme_bg_color = Color32::from_rgb(0, 80, 0);
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
        
        let rounding = CornerRadius::same(12);
        visuals.widgets.inactive.corner_radius = rounding;
        visuals.widgets.hovered.corner_radius  = rounding;
        visuals.widgets.active.corner_radius   = rounding;
        visuals.widgets.open.corner_radius     = rounding;
        visuals.widgets.noninteractive.corner_radius = rounding;
        
        ctx.set_style(style);
    }

    pub fn create_detached_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        panel_type: PanelType,
        position: (f32, f32),
        size: (f32, f32),
        config: &Config,
    ) -> Result<()> {
        let title = match panel_type {
            PanelType::DevTools => "Инструменты разработчика",
            PanelType::AIChat => "Чат с ИИ",
            PanelType::Settings => "Настройки",
            PanelType::Toolbar => "Панель инструментов",
        };

        let window = Arc::new(
            event_loop
                .create_window(
                    winit::window::WindowAttributes::default()
                        .with_title(title)
                        .with_position(winit::dpi::PhysicalPosition::new(position.0, position.1))
                        .with_inner_size(winit::dpi::PhysicalSize::new(size.0, size.1))
                        .with_decorations(true)
                        .with_resizable(true),
                )
                .context("Не удалось создать окно")?,
        );

        let surface = self.instance
            .create_surface(window.clone())
            .context("Не удалось создать поверхность для окна")?;

        let surface_caps = surface.get_capabilities(&self.adapter);
        let surface_config = wgpu_render::create_optimal_surface_config(
            &surface_caps,
            (size.0 as u32, size.1 as u32),
        )?;

        surface.configure(&self.device, &surface_config);

        let renderer = Renderer::new(&self.device, surface_config.format, RendererOptions::default());
        
        let egui_ctx = EguiContext::default();
        
        egui_ctx.set_fonts(egui::FontDefinitions::default());
        let scale_factor = config.settings.scale_factor.unwrap_or(1.0);
        egui_ctx.set_pixels_per_point(scale_factor);
        
        Self::configure_egui_style(&egui_ctx);

        let egui_state = EguiState::new(
            egui_ctx.clone(),
            egui::ViewportId::from_hash_of(&panel_type),
            &window,
            None,
            None,
            None,
        );

        self.window_state.detached_windows.insert(
            panel_type.clone(),
            DetachedWindow {
                panel_type,
                window,
                surface,
                renderer,
                egui_state,
                egui_ctx,
                surface_config,
                position,
                size,
                visible: true,
                should_close: false,
            },
        );

        Ok(())
    }

    pub fn render_detached_window(
        &mut self,
        panel_type: &PanelType,
        window_data: DetachedWindowData,
    ) -> Result<()> {
        if let Some(detached) = self.window_state.detached_windows.get_mut(panel_type) {
            if detached.should_close {
                self.window_state.detached_windows.remove(panel_type);
                return Ok(());
            }

            if !detached.visible {
                return Ok(());
            }

            let raw_input = detached.egui_state.take_egui_input(&detached.window);
            let panel_type_clone = panel_type.clone();
            let mut should_attach = false;

            let full_output = detached.egui_ctx.run(raw_input, |ctx| {
                if !ctx.memory(|mem| mem.data.get_temp(egui::Id::new("style_configured")).unwrap_or(false)) {
                    Self::configure_egui_style(ctx);
                    ctx.memory_mut(|mem| mem.data.insert_temp(egui::Id::new("style_configured"), true));
                }

                egui::CentralPanel::default().show(ctx, |ui| {
                    match &panel_type_clone {
                        PanelType::DevTools => {
                            // ← ЧИТАЕМ ИЗ devtools_state (получает из движка)
                            let action = render_devtools_ui(
                                ui,
                                window_data.devtools_tab,
                                window_data.html,
                                window_data.headers,
                                window_data.debug_info,
                                window_data.network_logs,
                                window_data.tx_ssh,
                                true,
                                window_data.devtools_state,  // ← ВСЁ через DevToolsState
                            );
                            if let Some(PanelAction::Attach) = action {
                                should_attach = true;
                            }
                        }
                        PanelType::AIChat => {
                            let action = aichat::render_aichat_ui(
                                ui,
                                window_data.url_input,
                                window_data.ai_chat_history,
                                window_data.send_ai_message,
                                true,
                            );
                            if let Some(PanelAction::Attach) = action {
                                should_attach = true;
                            }
                        }
                        PanelType::Settings => {
                            let action = settings::render_settings_ui(
                                ui,
                                window_data.config,
                                window_data.save_config,
                                true,
                            );
                            if let Some(PanelAction::Attach) = action {
                                should_attach = true;
                            }
                        }
                        PanelType::Toolbar => {
                            toolbar::render_toolbar(
                                ui,
                                window_data.url_input,
                                window_data.url,
                                window_data.bookmarks,
                                window_data.show_devtools,
                                window_data.show_settings,
                                window_data.status_message,
                                window_data.do_refresh,
                                window_data.do_add_bookmark,
                                window_data.new_url,
                                window_data.toolbar_position,
                                window_data.show_toolbar,
                            );
                        }
                    }
                });
            });

            if should_attach {
                detached.should_close = true;
                return Ok(());
            }

            let size = detached.window.inner_size();
            if size.width > 0 && size.height > 0 && !detached.window.is_minimized().unwrap_or(false) {
                wgpu_render::render_frame(
                    &self.device,
                    &self.queue,
                    &detached.surface,
                    &detached.surface_config,
                    &mut detached.renderer,
                    &detached.egui_ctx,
                    &full_output,
                    window_data.render_manager,
                )?;
            }

            let viewport_rect = detached.egui_ctx.viewport_rect();
            if !viewport_rect.is_negative() {
                detached.position = (viewport_rect.min.x, viewport_rect.min.y);
                detached.size = (viewport_rect.width(), viewport_rect.height());
            }

            if detached.egui_ctx.has_requested_repaint() {
                detached.window.request_redraw();
            }
        }
        Ok(())
    }

    pub fn handle_window_event(&mut self, panel_type: &PanelType, event: &WindowEvent) -> Result<()> {
        if let Some(detached) = self.window_state.detached_windows.get_mut(panel_type) {
            let resp = detached.egui_state.on_window_event(&detached.window, event);
            if resp.repaint {
                detached.window.request_redraw();
            }
            
            if let WindowEvent::CloseRequested = event {
                detached.should_close = true;
            }
        }
        Ok(())
    }

    pub fn resize_detached_window(&mut self, panel_type: &PanelType, new_size: winit::dpi::PhysicalSize<u32>) -> Result<()> {
        if let Some(detached) = self.window_state.detached_windows.get_mut(panel_type) {
            if new_size.width > 0 && new_size.height > 0 {
                let surface_caps = detached.surface.get_capabilities(&self.adapter);
                let config = wgpu_render::create_optimal_surface_config(&surface_caps, (new_size.width, new_size.height))?;
                detached.surface.configure(&self.device, &config);
                detached.surface_config = config;
                detached.size = (new_size.width as f32, new_size.height as f32);
            }
        }
        Ok(())
    }
    
    pub fn close_all_detached_windows(&mut self) {
        self.window_state.detached_windows.clear();
    }

    pub fn cleanup_closed_windows(&mut self) {
        self.window_state.detached_windows.retain(|_, detached| !detached.should_close);
    }

    pub fn get_detached_window_types(&self) -> Vec<PanelType> {
        self.window_state.detached_windows.keys().cloned().collect()
    }
}

// === FILE: utils\config.rs ===
use anyhow::Result;
pub struct Config;
impl Config {
    pub fn new() -> Result<Self> {
        println!("Config initialized");
        Ok(Self)
    }
}

// === FILE: utils\log.rs ===
use anyhow::Result;
use log::LevelFilter;
pub struct Log;
impl Log {
    pub fn new() -> Result<Self> {
        println!("Log initialized");
        Ok(Self)
    }
}
pub fn init_logger(_level: LevelFilter) -> Result<()> {
    println!("Logger initialized (stub)");
    Ok(())
}

// === FILE: utils\mem.rs ===
use anyhow::Result;
pub struct Mem;
impl Mem {
    pub fn new() -> Result<Self> {
        println!("Mem initialized");
        Ok(Self)
    }
}

// === FILE: utils\mod.rs ===
pub mod log;
pub mod config;
pub mod timer;
pub mod mem;
pub mod platform;
pub mod net_utils;

// === FILE: utils\net_utils.rs ===
use anyhow::Result;
pub struct NetUtils;
impl NetUtils{
    pub fn new() -> Result<Self> {
        println!("NetUtils initialized");
        Ok(Self)
    }
}

// === FILE: utils\platform.rs ===
use anyhow::Result;
pub struct Platform;
impl Platform{
    pub fn new() -> Result<Self> {
        println!("Platform initialized");
        Ok(Self)
    }
}

// === FILE: utils\timer.rs ===
use anyhow::Result;
pub struct Timer;
impl Timer {
    pub fn new() -> Result<Self> {
        println!("Timer initialized");
        Ok(Self)
    }
}

// === FILE: wasm_api\bindings.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Bindings;
impl Bindings {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Bindings initialized");
        Ok(Self)
    }
}

// === FILE: wasm_api\cloud_bindings.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct CloudBindings;
impl CloudBindings {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("CloudBindings initialized");
        Ok(Self)
    }
}

// === FILE: wasm_api\mod.rs ===
pub mod bindings;
pub mod signals;
pub mod web_sys;
pub mod sandbox;
pub mod cloud_bindings;
pub mod provider_api;

// === FILE: wasm_api\provider_api.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct ProviderAPI;
impl ProviderAPI {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("ProviderAPI initialized");
        Ok(Self)
    }
}

// === FILE: wasm_api\sandbox.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Sandbox;
impl Sandbox {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Sandbox initialized");
        Ok(Self)
    }
}

// === FILE: wasm_api\signals.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct Signals;
impl Signals {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("Signals initialized");
        Ok(Self)
    }
}

// === FILE: wasm_api\web_sys.rs ===
use anyhow::Result;
use std::sync::Arc;
use crate::net::Network;
pub struct WebSys;
impl WebSys {
    pub fn new(_network: Arc<Network>) -> Result<Self> {
        println!("WebSys initialized");
        Ok(Self)
    }
}

// === FILE: main.rs ===
#![windows_subsystem = "windows"]


mod core;
mod net;
mod ui;
mod dom;

use anyhow::Result;
use std::sync::Arc;
use tokio::runtime::Builder;

use crate::core::config::Config;
use crate::core::installer::Installer;
use crate::core::interfaces::{
    NetworkTrait, JsRuntimeTrait, WasmRuntimeTrait, OrchestratorTrait, PluginManagerTrait,
    LibManagerTrait, YuaidbTrait, SecurityManagerTrait, ReactiveCoreTrait, SessionManagerTrait,
    ProfileManagerTrait, ServiceWorkerTrait, SchedulerTrait, IoManagerTrait, WasmManifestTrait,
};
use crate::core::engine::{BroEngine, EngineEvent};
use crate::core::runtime::Runtime as WasmRuntime;
use crate::core::js_runtime::JsRuntime;
use crate::core::orchestrator::Orchestrator;
use crate::core::plugins::PluginManager;
use crate::core::libs::LibManager;
use crate::core::yuaidb::Yuaidb;
use crate::core::security::Security;
use crate::core::reactive::ReactiveCore;
use crate::core::session::SessionManager;
use crate::core::profile::ProfileManager;
use crate::core::service_worker::ServiceWorker;
use crate::core::scheduler::Scheduler;
use crate::core::io_manager::IoManager;
use crate::core::wasm_manifest::WasmManifest;
use crate::net::fetch::{Network, run_network_service};


fn main() -> Result<()> {
    let rt = Builder::new_multi_thread()
        .worker_threads(8) // Должно браться из config.settings.max_threads
        .enable_all()
        .build()?;

    rt.block_on(async {
        // 1. Настройка
        env_logger::init();
        Installer::setup()?;
        let cosmonaut_dir = Installer::get_cosmonaut_dir()?;
        let config = Config::load_from_file(Config::get_config_path())?;
        config.validate()?;

        // 2. Инициализация Core Services
        let db = Arc::new(Yuaidb::new(cosmonaut_dir.join("yuaidb")).await?) as Arc<dyn YuaidbTrait + Send + Sync>;

        // Инициализация сети только если включена feature "network"
        let network: Arc<dyn NetworkTrait + Send + Sync> = if cfg!(feature = "network") {
            Arc::new(Network::new(&config, Some(db.clone()))?) as Arc<dyn NetworkTrait + Send + Sync>
        } else {
            Arc::new(DummyNetwork) as Arc<dyn NetworkTrait + Send + Sync>
        };

        let wasm_runtime = Arc::new(WasmRuntime::new(&config).await?) as Arc<dyn WasmRuntimeTrait + Send + Sync>;
        let js_runtime = if cfg!(feature = "js") {
            Arc::new(JsRuntime::new()?) as Arc<dyn JsRuntimeTrait + Send + Sync>
        } else {
            Arc::new(DummyJsRuntime) as Arc<dyn JsRuntimeTrait + Send + Sync>
        };
        let orchestrator = Arc::new(Orchestrator::new()?) as Arc<dyn OrchestratorTrait + Send + Sync>;
        let plugin_manager = Arc::new(PluginManager::new(cosmonaut_dir.join("plugins"), cosmonaut_dir.join("providers")).await?) as Arc<dyn PluginManagerTrait + Send + Sync>;
        let lib_manager = Arc::new(LibManager::new(cosmonaut_dir.join("libs")).await?) as Arc<dyn LibManagerTrait + Send + Sync>;
        let security = Arc::new(Security::new(&config).await?) as Arc<dyn SecurityManagerTrait + Send + Sync>;
        let reactive_core = Arc::new(ReactiveCore::new().await) as Arc<dyn ReactiveCoreTrait + Send + Sync>;
        let session_manager = Arc::new(SessionManager::new(db.clone()).await?) as Arc<dyn SessionManagerTrait + Send + Sync>;
        let profile_manager = Arc::new(ProfileManager::new(cosmonaut_dir.join("certs").join("user")).await?) as Arc<dyn ProfileManagerTrait + Send + Sync>;
        let service_worker = Arc::new(ServiceWorker::new(network.clone(), db.clone()).await?) as Arc<dyn ServiceWorkerTrait + Send + Sync>;
        let io_manager = Arc::new(IoManager::new(network.clone(), db.clone()).await?) as Arc<dyn IoManagerTrait + Send + Sync>;
        let wasm_manifest = Arc::new(WasmManifest::new().await?) as Arc<dyn WasmManifestTrait + Send + Sync>;
        let scheduler = Arc::new(Scheduler::new(Some(wasm_runtime.clone()), Some(js_runtime.clone()), Some(orchestrator.clone())).await?) as Arc<dyn SchedulerTrait + Send + Sync>;

        // 3. Инициализация движка
        let engine = Arc::new(BroEngine::new(
            network.clone(),
            js_runtime,
            wasm_runtime,
            orchestrator,
            plugin_manager,
            lib_manager,
            db,
            security,
            reactive_core,
            session_manager,
            profile_manager,
            service_worker,
            scheduler,
            io_manager,
            wasm_manifest,
            None,
            None,
            None,
        ).await?);

        // 4. Запуск асинхронного сетевого сервиса, если сеть включена
        if cfg!(feature = "network") {
            tokio::spawn(run_network_service(network, engine.clone()));
        }

        // 5. Запуск UI-клиента, если включена feature "ui"
        let result = if cfg!(feature = "ui") {
            crate::ui::run(engine).await
        } else {
            log::info!("UI отключен, запуск в headless-режиме");
            Ok(())
        };

        log::info!("Приложение завершено с результатом: {:?}", result);
        result
    })
}

// Заглушка для отключенной сети
struct DummyNetwork;

#[async_trait::async_trait]
impl NetworkTrait for DummyNetwork {
    async fn fetch_html(&self, _url: &str) -> Result<String> {
        Err(anyhow::anyhow!("Сетевой модуль отключен"))
    }

    async fn fetch_html_with_headers(&self, _url: &str) -> Result<(String, reqwest::header::HeaderMap)> {
        Err(anyhow::anyhow!("Сетевой модуль отключен"))
    }
}

// Заглушка для отключенного JS
struct DummyJsRuntime;

#[async_trait::async_trait]
impl JsRuntimeTrait for DummyJsRuntime {
    async fn execute_script(&self, _script: &str) -> Result<()> {
        Err(anyhow::anyhow!("JS runtime отключен"))
    }
}

