use paths::URL_SCHEME;
use gpui::{AsyncApp, actions};

actions!(
    cli,
    [
        /// Registers the sotid:// URL scheme handler.
        RegisterSotidScheme
    ]
);

pub async fn register_sotid_scheme(cx: &AsyncApp) -> anyhow::Result<()> {
    cx.update(|cx| cx.register_url_scheme(URL_SCHEME)).await
}
