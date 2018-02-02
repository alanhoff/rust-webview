#include "webview.h"

size_t get_priv_size() {
    return sizeof (struct webview_priv);
}

int r_webview_init(struct webview *w)
{
    return webview_init(w);
}

int r_webview_loop(struct webview *w, int blocking)
{
    return webview_loop(w, blocking);
}

void r_webview_exit(struct webview *w)
{
    webview_exit(w);
}

int r_webview_eval(struct webview *w, const char *js)
{
    webview_eval(w, js);
}

int r_webview_inject_css(struct webview *w, const char *css)
{
    webview_inject_css(w, css);
}

void r_webview_set_title(struct webview *w, const char *title)
{
    webview_set_title(w, title);
}

void r_webview_set_fullscreen(struct webview *w, int fullscreen)
{
    webview_set_fullscreen(w, fullscreen);
}

void r_webview_dialog(struct webview *w, enum webview_dialog_type dlgtype,
                      int flags, const char *title, const char *arg,
                      char *result, size_t resultsz)
{
    webview_dialog(w, dlgtype, flags, title, arg, result, resultsz);
}

void r_webview_dispatch(struct webview *w, webview_dispatch_fn fn, void *arg)
{
    webview_dispatch(w, fn, arg);
}

void r_webview_terminate(struct webview *w)
{
    webview_terminate(w);
}
