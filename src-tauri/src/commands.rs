use crate::Model;
use tauri::{AppHandle, Manager, Window};
use url::Url;

const GITHUB_AUTH_URL: &str = "https://github.com/login";
const GITHUB_REDIRECT_URL: &str = "http://127.0.0.1:5173";

#[tauri::command]
pub async fn github_auth_flow(app: AppHandle) {
    let (tx, rx) = std::sync::mpsc::channel::<url::Url>();
    // NOTE: label determines which notifier is triggered from auth-init.html
    let label = "auth-github";
    let window = tauri::WindowBuilder::new(&app, label, tauri::WindowUrl::App("index.html".into()))
        .on_navigation(move |url: url::Url| {
            let str = url.as_str();
            if str.starts_with(GITHUB_REDIRECT_URL) {
                tx.send(url).unwrap();
                return false;
            }
            true
        })
        .build()
        .unwrap();

    // wait for auth-init.html to notify ready
    app.state::<Model>().notifiers.github_auth_ready.notified().await;

    // <opportunity to clear cookies or cache on PlatformWebview before navigating>
    // ...
    
    // navigate to the auth url
    let auth_url = Url::parse(GITHUB_AUTH_URL).unwrap();
    webview_navigate(&window, auth_url).unwrap();

    // await the captured redirect url
    let redirect_url = rx.recv().unwrap();

    // do something interesting with "redirect_url"
}

#[tauri::command]
pub async fn github_auth_ready(app: AppHandle) {
    app.state::<Model>().notifiers.github_auth_ready.notify_waiters();
}


pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type BoxResult<T> = Result<T, BoxError>;

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
fn webview_navigate(window: &Window, url: Url) -> BoxResult<()> {
    use webkit2gtk::WebViewExt;
    window.with_webview(move |webview| {
        let webview = webview.inner();
        webview.load_uri(url.as_str());
    })?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn webview_navigate(window: &Window, url: Url) -> BoxResult<()> {
    use cacao::{
        foundation::{id, NSString},
        objc::*,
    };
    window
        .with_webview(move |webview| unsafe {
            let webview = webview.inner();
            let string = NSString::new(url.as_str());
            let url: id = msg_send![class!(NSURL), URLWithString: string];
            let request: id = msg_send![class!(NSURLRequest), requestWithURL: url];
            let _navigation: id = msg_send![webview, loadRequest: request];
        })
        .map_err(Into::into)
}

#[cfg(target_os = "windows")]
fn webview_navigate(window: &Window, url: Url) -> BoxResult<()> {
    use tauri::window::PlatformWebview;
    use webview2_com::Error::WindowsError;
    use windows::core::HSTRING;
    unsafe fn run(webview: PlatformWebview, url: Url) -> Result<(), wry::Error> {
        let webview = webview.controller().CoreWebView2().map_err(WindowsError)?;
        let url = &HSTRING::from(url.as_str());
        webview.Navigate(url).map_err(WindowsError)?;
        Ok(())
    }
    let (call_tx, call_rx) = oneshot::channel();
    window
        .with_webview(move |webview| unsafe {
            let result = run(webview, url).map_err(Into::into);
            call_tx.send(result).unwrap();
        })
        .map_err(Into::into)
        .and(call_rx.recv().unwrap())
}