use leptos::*;
use leptos_router::*;

const NAVT: &str = "block py-2 pl-3 pr-4 text-white rounded md:p-0 active:border-blue-500";
const NAVB: &str =
    "bg-white bg-gray-900 w-full z-20 top-0 left-0 border-b border-gray-200 border-gray-600";
const NAVLIST: &str = "flex flex-col p-4 md:p-0 mt-4 font-medium border border-gray-100 bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700";

#[component]
pub fn RouterExample(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <div id="root">
        // we wrap the whole app in a <Router/> to allow client-side navigation
        // from our nav links below
        <Router>
          // <nav> and <main> will show on every route
          <nav class=NAVB>
          <ul class=NAVLIST >
          <li class="-mb-px mr-1">
            <A href="configuration" class=NAVT >"Configuration"</A>
          </li>
          <li class="-mb-px mr-1">
            // But we can also use a normal class attribute like it is a normal component
            <A href="settings" class=NAVT >"Settings"</A>
            </li>
            <li class="-mb-px mr-1">
            // It also supports signals!
            <A href="about" class=move || NAVT>"About"</A>
            </li>
            </ul>
          </nav>
          <main>
            <Routes>
              <Route path="configuration" view=move |cx| view! { cx,  <Configuration/> } >  </Route>
              <Route path="settings" view=move |cx| view! { cx,  <Settings/> } > </Route>
              <Route path="about" view=move |cx| view! { cx,  <About/> } > </Route>
            </Routes>
          </main>
        </Router>
      </div>
    }
}

#[component]
fn Configuration(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <p>"This is Configuration."</p>
      <p>"This is Configuration."</p>
    }
}

#[component]
fn About(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <p>"This is About."</p>
    }
}

#[component]
fn Settings(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <p>"This is Settings."</p>
    }
}

pub fn main() {
    mount_to_body(|cx| view! { cx,  <RouterExample /> })
}
